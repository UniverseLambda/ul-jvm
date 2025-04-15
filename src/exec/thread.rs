use anyhow::{anyhow, bail};

use super::{
    JvmExecEnv, class::Class, jpu::JvmProcessUnit, method::Method, runtime_type::RuntimeType,
};

pub struct JvmThread {
    pub pc: usize,
    pub stack: Vec<StackFrame>,
    skip_static_init: bool,
}

pub struct StackFrame {
    pub return_pc: usize,
    pub current_class: Class,
    pub locals: Vec<RuntimeType>,
    pub operand_stack: Vec<RuntimeType>,
}

impl JvmThread {
    pub fn new(class: Class, method: &Method) -> Self {
        let mut instance = Self {
            pc: 0,
            stack: vec![],
            skip_static_init: false,
        };

        instance.call_intro(
            class,
            method
                .start_pc()
                .expect("native or abstract methods cannot be the first to be called "),
        );

        instance
    }

    pub fn current_frame(&self) -> anyhow::Result<&StackFrame> {
        self.stack.last().ok_or_else(|| anyhow!("not in a thread"))
    }

    pub fn current_frame_mut(&mut self) -> anyhow::Result<&mut StackFrame> {
        self.stack
            .last_mut()
            .ok_or_else(|| anyhow!("not in a thread"))
    }

    pub fn run(&mut self, env: &JvmExecEnv) -> anyhow::Result<()> {
        let jpu = JvmProcessUnit::jpu_new(env, self.skip_static_init);

        while !self.stack.is_empty() {
            match self.pop_byte(env)? {
                0x2b => {
                    let short = self.pop_short(env)?;
                    jpu.getstatic(self, short)?
                }
                0x14 => {
                    let short = self.pop_short(env)?;
                    jpu.ld2c_w(self, short)?;
                }
                v => bail!("unknown opcode at 0x{:08X}: 0x{v:02X}", (self.pc - 1)),
            }
        }

        Ok(())
    }

    pub fn run_clinit_thread(env: &JvmExecEnv, class: Class) -> anyhow::Result<()> {
        let Some(method) = class.methods.iter().find_map(|m| {
            if m.0 == "<clinit>" && m.1.parameters().is_empty() && m.1.ret_type().is_none() {
                Some(m.1)
            } else {
                None
            }
        }) else {
            return Ok(());
        };

        let mut instance = Self::new(class.clone(), method);

        instance.skip_static_init = true;
        instance.run(env)
    }

    fn call_intro(&mut self, class: Class, pc: usize) {
        self.stack.push(StackFrame {
            return_pc: self.pc,
            current_class: class,
            locals: vec![],
            operand_stack: vec![],
        });

        self.pc = pc;
    }

    fn pop_byte(&mut self, env: &JvmExecEnv) -> anyhow::Result<u8> {
        let byte = env
            .code
            .get(self.pc)
            .ok_or(anyhow!("pc went out of code memory (pc = {})", self.pc))?;

        self.pc += 1;

        Ok(*byte)
    }

    fn pop_short(&mut self, env: &JvmExecEnv) -> anyhow::Result<u16> {
        Ok(u16::from_be_bytes([
            self.pop_byte(env)?,
            self.pop_byte(env)?,
        ]))
    }
}
