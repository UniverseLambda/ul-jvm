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
                v => bail!("unknown opcode at {}: {v}", (self.pc - 1)),
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

/*
    Instructions:
    - aaload:               TODO
    - aastore:              TODO
    - aconst_null:          TODO
    - aload:                TODO
    - aload_&lt;n&gt;:      TODO
    - anewarray:            TODO
    - areturn:              TODO
    - arraylength:          TODO
    - astore:               TODO
    - astore_&lt;n&gt;:     TODO
    - athrow:               TODO
    - baload:               TODO
    - bastore:              TODO
    - bipush:               TODO
    - caload:               TODO
    - castore:              TODO
    - checkcast:            TODO
    - d2f:                  TODO
    - d2i:                  TODO
    - d2l:                  TODO
    - dadd:                 TODO
    - daload:               TODO
    - dastore:              TODO
    - dcmp&lt;op&gt;:       TODO
    - dconst_&lt;d&gt;:     TODO
    - ddiv:                 TODO
    - dload:                TODO
    - dload_&lt;n&gt;:      TODO
    - dmul:                 TODO
    - dneg:                 TODO
    - drem:                 TODO
    - dreturn:              TODO
    - dstore:               TODO
    - dstore_&lt;n&gt;:     TODO
    - dsub:                 TODO
    - dup:                  TODO
    - dup_x1:               TODO
    - dup_x2:               TODO
    - dup2:                 TODO
    - dup2_x1:              TODO
    - dup2_x2:              TODO
    - f2d:                  TODO
    - f2i:                  TODO
    - f2l:                  TODO
    - fadd:                 TODO
    - faload:               TODO
    - fastore:              TODO
    - fcmp&lt;op&gt;:       TODO
    - fconst_&lt;f&gt;:     TODO
    - fdiv:                 TODO
    - fload:                TODO
    - fload_&lt;n&gt;:      TODO
    - fmul:                 TODO
    - fneg:                 TODO
    - frem:                 TODO
    - freturn:              TODO
    - fstore:               TODO
    - fstore_&lt;n&gt;:     TODO
    - fsub:                 TODO
    - getfield:             TODO
    - getstatic:            TODO
    - goto:                 TODO
    - goto_w:               TODO
    - i2b:                  TODO
    - i2c:                  TODO
    - i2d:                  TODO
    - i2f:                  TODO
    - i2l:                  TODO
    - i2s:                  TODO
    - iadd:                 TODO
    - iaload:               TODO
    - iand:                 TODO
    - iastore:              TODO
    - iconst_&lt;i&gt;:     TODO
    - idiv:                 TODO
    - if_acmp&lt;cond&gt;:  TODO
    - if_icmp&lt;cond&gt;:  TODO
    - if&lt;cond&gt;:       TODO
    - ifnonnull:            TODO
    - ifnull:               TODO
    - iinc:                 TODO
    - iload:                TODO
    - iload_&lt;n&gt;:      TODO
    - imul:                 TODO
    - ineg:                 TODO
    - instanceof:           TODO
    - invokedynamic:        TODO
    - invokeinterface:      TODO
    - invokespecial:        TODO
    - invokestatic:         TODO
    - invokevirtual:        TODO
    - ior:                  TODO
    - irem:                 TODO
    - ireturn:              TODO
    - ishl:                 TODO
    - ishr:                 TODO
    - istore:               TODO
    - istore_&lt;n&gt;:     TODO
    - isub:                 TODO
    - iushr:                TODO
    - ixor:                 TODO
    - jsr:                  TODO
    - jsr_w:                TODO
    - l2d:                  TODO
    - l2f:                  TODO
    - l2i:                  TODO
    - ladd:                 TODO
    - laload:               TODO
    - land:                 TODO
    - lastore:              TODO
    - lcmp:                 TODO
    - lconst_&lt;l&gt;:     TODO
    - ldc:                  TODO
    - ldc_w:                TODO
    - ldc2_w:               TODO
    - ldiv:                 TODO
    - lload:                TODO
    - lload_&lt;n&gt;:      TODO
    - lmul:                 TODO
    - lneg:                 TODO
    - lookupswitch:         TODO
    - lor:                  TODO
    - lrem:                 TODO
    - lreturn:              TODO
    - lshl:                 TODO
    - lshr:                 TODO
    - lstore:               TODO
    - lstore_&lt;n&gt;:     TODO
    - lsub:                 TODO
    - lushr:                TODO
    - lxor:                 TODO
    - monitorenter:         TODO
    - monitorexit:          TODO
    - multianewarray:       TODO
    - new:                  TODO
    - newarray:             TODO
    - nop:                  TODO
    - pop:                  TODO
    - pop2:                 TODO
    - putfield:             TODO
    - putstatic:            TODO
    - ret:                  TODO
    - return:               TODO
    - saload:               TODO
    - sastore:              TODO
    - sipush:               TODO
    - swap:                 TODO
    - tableswitch:          TODO
    - wide:                 TODO
*/
