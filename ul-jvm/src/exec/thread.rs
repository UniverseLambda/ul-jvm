use std::io::Write;

use anyhow::{anyhow, bail};
use log::{info, trace};

use crate::types::{JvmInt, JvmMethodDescriptor};

use super::{
    JvmExecEnv, class::Class, jpu::JvmProcessUnit, method::Method, runtime_type::RuntimeType,
};

#[derive(Debug)]
pub struct JvmThread {
    pub pc: usize,
    pub stack: Vec<StackFrame>,
    pub operand_stack: Vec<RuntimeType>,
    skip_static_init: bool,
}

#[derive(Debug)]
pub struct StackFrame {
    pub return_pc: usize,
    pub current_class: Class,
    pub locals: Box<[Option<RuntimeType>]>,
}

impl JvmThread {
    pub fn new(class: Class, method: &Method) -> Self {
        let mut instance = Self {
            pc: 0,
            stack: vec![],
            skip_static_init: false,
            operand_stack: vec![],
        };

        instance.call_intro(
            class,
            method
                .start_pc()
                .expect("native or abstract methods cannot be the first to be called "),
            method.local_count(),
        );

        instance
    }

    pub fn read_local(&self, index: usize) -> anyhow::Result<RuntimeType> {
        let frame = self.current_frame()?;
        let Some(local) = frame
            .locals
            .get(index)
            .ok_or_else(|| anyhow!("{index} out of bound for local storage"))?
        else {
            bail!("{index} is unusable");
        };

        Ok(local.clone())
    }

    pub fn store_to_local(&mut self, index: usize, value: RuntimeType) -> anyhow::Result<()> {
        let frame = self.current_frame_mut()?;
        let Some(local) = frame
            .locals
            .get_mut(index)
            .ok_or_else(|| anyhow!("{index} out of bound for local storage"))?
        else {
            bail!("{index} is unusable");
        };

        *local = value;

        Ok(())
    }

    pub fn forbid_local(&mut self, index: usize) -> anyhow::Result<()> {
        self.current_frame_mut()?
            .locals
            .get_mut(index)
            .ok_or_else(|| anyhow!("{index} out of bound for local storage"))?
            .take();

        Ok(())
    }

    pub fn allow_local(&mut self, index: usize) -> anyhow::Result<()> {
        let Some(local) = self.current_frame_mut()?.locals.get_mut(index) else {
            return Ok(());
        };

        if local.is_none() {
            *local = Some(RuntimeType::Int(0));
        }

        Ok(())
    }

    pub fn jmp_to(&mut self, address: usize) {
        self.pc = address;
    }

    pub fn ret(&mut self) -> anyhow::Result<()> {
        let Some(previous_frame) = self.stack.pop() else {
            return Ok(());
        };

        self.operand_stack.clear();
        self.jmp_to(previous_frame.return_pc);

        Ok(())
    }

    pub fn current_frame(&self) -> anyhow::Result<&StackFrame> {
        self.stack.last().ok_or_else(|| anyhow!("not in a thread"))
    }

    pub fn current_frame_mut(&mut self) -> anyhow::Result<&mut StackFrame> {
        self.stack
            .last_mut()
            .ok_or_else(|| anyhow!("not in a thread"))
    }

    pub fn pop_operand_stack(&mut self) -> anyhow::Result<RuntimeType> {
        self.operand_stack
            .pop()
            .ok_or_else(|| anyhow!("tried to pop an empty operand stack"))
    }

    pub fn push_operand_stack(&mut self, value: RuntimeType) {
        self.operand_stack.push(value);
    }

    pub fn run(&mut self, env: &JvmExecEnv) -> anyhow::Result<()> {
        info!("starting thread");

        let jpu = JvmProcessUnit::jpu_new(env, self.skip_static_init);

        while !self.stack.is_empty() {
            let op_code = self.pop_ubyte(env)?;

            trace!("current op-code: 0x{op_code:02x}");

            match op_code {
                v @ 0x02 | v @ 0x03 | v @ 0x04 | v @ 0x05 | v @ 0x06 | v @ 0x07 | v @ 0x08 => {
                    jpu.bipush(self, (v as JvmInt) - 0x03)?
                }
                0x10 => {
                    let sbyte = self.pop_sbyte(env)?;
                    jpu.bipush(self, sbyte as JvmInt)?;
                }
                0xb2 => {
                    let short = self.pop_ushort(env)?;
                    jpu.getstatic(self, short)?
                }
                0xb8 => {
                    let short = self.pop_ushort(env)?;
                    jpu.invokestatic(self, short)?;
                }
                0x14 => {
                    let short = self.pop_ushort(env)?;
                    jpu.ld2c_w(self, short)?;
                }
                0x16 => {
                    let local_index = self.pop_ubyte(env)?;
                    jpu.lload(self, local_index)?;
                }
                v @ 0x1e | v @ 0x1f | v @ 0x20 | v @ 0x21 => jpu.lload(self, v - 0x1e)?,
                0x37 => {
                    let local_index = self.pop_ubyte(env)?;
                    jpu.lstore(self, local_index)?;
                }
                v @ 0x3f | v @ 0x40 | v @ 0x41 | v @ 0x42 => {
                    jpu.lstore(self, v - 0x3f)?;
                }
                0x39 => {
                    let local_index = self.pop_ubyte(env)?;
                    jpu.dstore(self, local_index)?;
                }
                0x15 => {
                    let local_index = self.pop_ubyte(env)?;

                    jpu.iload(self, local_index)?;
                }
                v @ 0x1a | v @ 0x1b | v @ 0x1c | v @ 0x1d => jpu.iload(self, v - 0x1a)?,
                0x36 => {
                    let local_index = self.pop_ubyte(env)?;

                    jpu.istore(self, local_index)?;
                }
                v @ 0x3b | v @ 0x3c | v @ 0x3d | v @ 0x3e => jpu.istore(self, v - 0x3b)?,
                v @ 0x47 | v @ 0x48 | v @ 0x49 | v @ 0x4a => jpu.dstore(self, v - 0x47)?,
                0x60 => jpu.iadd(self)?,
                0x6c => jpu.idiv(self)?,
                0xa9 => {
                    let byte = self.pop_ubyte(env)?;
                    jpu.ret(self, byte)?;
                }
                0xb1 => jpu.vreturn(self)?,
                v => bail!("unknown opcode at 0x{:08X}: 0x{v:02X}", (self.pc - 1)),
            }
        }

        Ok(())
    }

    pub fn run_clinit_thread(env: &JvmExecEnv, class: Class) -> anyhow::Result<()> {
        let Some(method) = class.get_static_method(
            &String::from("<clinit>"),
            JvmMethodDescriptor {
                return_type: None,
                parameter_types: vec![],
            },
        ) else {
            return Ok(());
        };

        let _lock = class.lock_statics();
        if !class.set_initialized_if_needed() {
            return Ok(());
        }

        let mut instance = Self::new(class.clone(), &method);

        instance.skip_static_init = true;
        instance.run(env)
    }

    pub fn jmp_jvm_method(&mut self, class: Class, method: &Method) -> anyhow::Result<()> {
        if self.operand_stack.len() != method.parameters().len() {
            bail!(
                "expected {} parameters in operand stack, but got {}",
                method.parameters().len(),
                self.operand_stack.len()
            );
        }

        self.call_intro(class, method.start_pc().unwrap(), method.local_count());

        let mut offset = 0;

        let params = self.operand_stack.drain(..).collect::<Vec<RuntimeType>>();

        for (idx, param) in params.into_iter().enumerate() {
            let doubled_size = param.is_two_slots();

            self.store_to_local(idx + offset, param)?;

            if doubled_size {
                offset += 1;
                self.forbid_local(idx + offset)?;
            }
        }

        Ok(())
    }

    fn call_intro(&mut self, class: Class, pc: usize, local_count: usize) {
        self.stack.push(StackFrame {
            return_pc: self.pc,
            current_class: class,
            locals: vec![Some(RuntimeType::Int(0)); local_count].into_boxed_slice(),
        });

        self.pc = pc;
    }

    fn pop_ubyte(&mut self, env: &JvmExecEnv) -> anyhow::Result<u8> {
        let byte = env
            .code
            .get(self.pc)
            .ok_or(anyhow!("pc went out of code memory (pc = {})", self.pc))?;

        self.pc += 1;

        Ok(*byte)
    }

    fn pop_sbyte(&mut self, env: &JvmExecEnv) -> anyhow::Result<i8> {
        self.pop_ubyte(env).map(|v| v as i8)
    }

    fn pop_ushort(&mut self, env: &JvmExecEnv) -> anyhow::Result<u16> {
        Ok(u16::from_be_bytes([
            self.pop_ubyte(env)?,
            self.pop_ubyte(env)?,
        ]))
    }

    // fn pop_sint(&mut self, env: &JvmExecEnv) -> anyhow::Result<JvmInt> {
    //     Ok(i32::from_be_bytes([
    //         self.pop_ubyte(env)?,
    //         self.pop_ubyte(env)?,
    //         self.pop_ubyte(env)?,
    //         self.pop_ubyte(env)?,
    //     ]))
    // }

    pub fn dump_to<W: Write>(&self, mut writer: W) -> anyhow::Result<()> {
        writeln!(writer, "========= THREAD DUMP =========")?;
        writeln!(writer, "PC = {}", self.pc)?;
        writeln!(writer, "SSI? {}", self.skip_static_init)?;
        writeln!(writer, "STACK:")?;
        for (idx, frame) in self.stack.iter().enumerate().rev() {
            writeln!(writer, "- frame {idx}")?;
            writeln!(writer, "  class:     {}", frame.current_class.name)?;
            writeln!(writer, "  return PC: {}", frame.return_pc)?;
            writeln!(writer, "  locals:")?;
            for (idx, elem) in frame.locals.iter().enumerate() {
                writeln!(writer, "  - [{idx}]      {elem:?}")?;
            }
        }
        writeln!(writer, "OS:")?;
        for (idx, elem) in self.operand_stack.iter().enumerate().rev() {
            writeln!(writer, "- [{idx}]      {elem:?}")?;
        }

        Ok(())
    }
}
