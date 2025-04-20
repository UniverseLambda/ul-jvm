use anyhow::{Context, anyhow, bail};
use log::{debug, trace};

use crate::{exec::runtime_type::RuntimeType, types::JvmInt};

use super::{JvmExecEnv, class::Class, thread::JvmThread};

pub struct JvmProcessUnit<'a> {
    env: &'a JvmExecEnv,
    skip_static_init: bool,
}

impl<'a> JvmProcessUnit<'a> {
    pub fn jpu_new(env: &'a JvmExecEnv, skip_static_init: bool) -> Self {
        Self {
            env,
            skip_static_init,
        }
    }

    pub fn bipush(&self, thread: &mut JvmThread, value: JvmInt) -> anyhow::Result<()> {
        trace!("bipush {value}");

        thread.push_operand_stack(RuntimeType::Int(value))?;

        Ok(())
    }

    pub fn dstore(&self, thread: &mut JvmThread, local_index: u8) -> anyhow::Result<()> {
        trace!("dstore");

        let local_index = local_index as usize;
        // TODO: check for double type
        let value = thread.pop_operand_stack()?;

        thread.store_to_local(local_index, value)?;
        thread.forbid_local(local_index + 1)?;

        Ok(())
    }

    pub fn getstatic(&self, thread: &mut JvmThread, cp_index: u16) -> anyhow::Result<()> {
        trace!("getstatic");

        let field_ref = thread
            .current_frame()?
            .current_class
            .constant_pool
            .get_field_ref(cp_index)
            .ok_or_else(|| anyhow!("no field_ref"))?;

        let target_class = self
            .resolve_class(&field_ref.class.name)
            .context("getstatic")?;

        self.init_static(thread, &target_class)?;

        let zarma = target_class.read_static(&field_ref.name)?;

        thread.current_frame_mut()?.operand_stack.push(zarma);

        Ok(())
    }

    pub fn iadd(&self, thread: &mut JvmThread) -> anyhow::Result<()> {
        trace!("iadd");

        let l = thread.pop_operand_stack()?;
        let r = thread.pop_operand_stack()?;

        let (l, r) = match (l, r) {
            (RuntimeType::Int(l), RuntimeType::Int(r)) => (l, r),
            (l, r) => bail!("expected two ints, got {l:?} and {r:?}"),
        };

        thread.push_operand_stack(RuntimeType::Int(l + r))?;

        Ok(())
    }

    pub fn idiv(&self, thread: &mut JvmThread) -> anyhow::Result<()> {
        trace!("idiv");

        let l = thread.pop_operand_stack()?;
        let r = thread.pop_operand_stack()?;

        let (l, r) = match (l, r) {
            (RuntimeType::Int(l), RuntimeType::Int(r)) => (l, r),
            (l, r) => bail!("expected two ints, got {l:?} and {r:?}"),
        };

        // TODO: throw an exception when r is 0

        thread.push_operand_stack(RuntimeType::Int(l / r))?;

        Ok(())
    }

    pub fn iload(&self, thread: &mut JvmThread, local_index: u8) -> anyhow::Result<()> {
        trace!("iload {local_index}");

        let local_index = local_index as usize;
        let value = thread.read_local(local_index)?;

        match value {
            RuntimeType::Int(_) => (),
            v => bail!("unexpected value (int expected): {v:?}"),
        }

        thread.push_operand_stack(value)
    }

    pub fn invokestatic(&self, thread: &mut JvmThread, cp_index: u16) -> anyhow::Result<()> {
        trace!("invokestatic");

        let current_class: Class = thread.current_frame()?.current_class.clone();

        // TODO: handle synchronized
        let (target_class, name, ty) = current_class
            .constant_pool
            .get_method_ref(cp_index)
            .map(|m| (m.class, m.name, m.ty))
            .or_else(|| {
                current_class
                    .constant_pool
                    .get_interface_method_ref(cp_index)
                    .map(|m| (m.class, m.name, m.ty))
            })
            .ok_or_else(|| anyhow!("no methodref at {cp_index}"))?;

        let target_class = self.resolve_class(&target_class.name)?;
        let method = target_class
            .get_static_method(&name, ty.clone())
            .ok_or_else(|| {
                anyhow!(
                    "no method {ty:?} named {name} found in {}",
                    target_class.name
                )
            })?;

        self.init_static(thread, &target_class)?;

        trace!(
            "invokestatic, calling {}:{name} ({ty:?}) (native: {})",
            target_class.name,
            method.is_native()
        );
        thread.jmp_jvm_method(target_class.clone(), &method);

        Ok(())
    }

    pub fn istore(&self, thread: &mut JvmThread, local_index: u8) -> anyhow::Result<()> {
        trace!("istore {local_index}");

        let local_index = local_index as usize;
        let value = thread.pop_operand_stack()?;

        // TODO: check for int type
        thread.store_to_local(local_index, value)?;
        thread.allow_local(local_index + 1)?;

        Ok(())
    }

    pub fn ld2c_w(&self, thread: &mut JvmThread, cp_index: u16) -> anyhow::Result<()> {
        trace!("ld2c_w");

        let class = thread.current_frame()?.current_class.clone();

        let value = class
            .constant_pool
            .get_long(cp_index)
            .map(RuntimeType::Long)
            .or_else(|| {
                class
                    .constant_pool
                    .get_double(cp_index)
                    .map(RuntimeType::Double)
            });

        if let Some(value) = value {
            thread.push_operand_stack(value)?;
        } else {
            // TODO: implement this case, but I'm not quite sure what I am to expect from here (kinda tired rn)
            todo!("unsupported ld2c_w constant pool value at {cp_index}");
        }

        Ok(())
    }

    pub fn lload(&self, thread: &mut JvmThread, local_index: u8) -> anyhow::Result<()> {
        trace!("lload");

        let local_index = local_index as usize;
        let value = thread.read_local(local_index)?;
        // TODO: check to see if local_index + 1 is forbidden

        match value {
            RuntimeType::Long(_) => (),
            v => bail!("unexpected value (long expected): {v:?}"),
        }

        thread.push_operand_stack(value)
    }

    pub fn lstore(&self, thread: &mut JvmThread, local_index: u8) -> anyhow::Result<()> {
        trace!("lstore");

        let local_index = local_index as usize;
        // TODO: check for long type
        let value = thread.pop_operand_stack()?;

        thread.store_to_local(local_index, value)?;
        thread.forbid_local(local_index + 1)?;

        Ok(())
    }

    pub fn ret(&self, thread: &mut JvmThread, local_index: u8) -> anyhow::Result<()> {
        trace!("ret {local_index}");

        let value = match thread.read_local(local_index as usize)? {
            RuntimeType::ReturnAddress(v) => v,
            v => bail!("expected return address, got {v:?}"),
        };

        thread.jmp_to(value);

        Ok(())
    }

    pub fn vreturn(&self, thread: &mut JvmThread) -> anyhow::Result<()> {
        trace!("vreturn");

        thread.ret()?;

        Ok(())
    }

    // TODO: work on a better way to also look for interfaces
    fn resolve_class(&self, class: &String) -> anyhow::Result<Class> {
        // TODO: load missing classes

        let Some(class) = self.env.classes.get(class) else {
            bail!("no class found for {class}");
        };

        Ok(class.clone())
    }

    fn init_static(&self, thread: &JvmThread, class: &Class) -> anyhow::Result<()> {
        if !self.skip_static_init || class.name != thread.current_frame()?.current_class.name {
            debug!("initializing class {}", class.name);
            JvmThread::run_clinit_thread(self.env, class.clone())
        } else {
            Ok(())
        }
    }
}

/*
    Instructions:
    - aaload:               TODO
    - aastore:              TODO
    - aconst_null:          TODO
    - aload:                TODO
    - aload_<n>:            TODO
    - anewarray:            TODO
    - areturn:              TODO
    - arraylength:          TODO
    - astore:               TODO
    - astore_<n>:           TODO
    - athrow:               TODO
    - baload:               TODO
    - bastore:              TODO
    - bipush:               COMPLETED
    - caload:               TODO
    - castore:              TODO
    - checkcast:            TODO
    - d2f:                  TODO
    - d2i:                  TODO
    - d2l:                  TODO
    - dadd:                 TODO
    - daload:               TODO
    - dastore:              TODO
    - dcmp<op>:             TODO
    - dconst_<d>:           TODO
    - ddiv:                 TODO
    - dload:                TODO
    - dload_<n>:            TODO
    - dmul:                 TODO
    - dneg:                 TODO
    - drem:                 TODO
    - dreturn:              TODO
    - dstore:               COMPLETED
    - dstore_<n>:           COMPLETED
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
    - fcmp<op>:             TODO
    - fconst_<f>:           TODO
    - fdiv:                 TODO
    - fload:                TODO
    - fload_<n>:            TODO
    - fmul:                 TODO
    - fneg:                 TODO
    - frem:                 TODO
    - freturn:              TODO
    - fstore:               TODO
    - fstore_<n>:           TODO
    - fsub:                 TODO
    - getfield:             TODO
    - getstatic:            COMPLETED
    - goto:                 TODO
    - goto_w:               TODO
    - i2b:                  TODO
    - i2c:                  TODO
    - i2d:                  TODO
    - i2f:                  TODO
    - i2l:                  TODO
    - i2s:                  TODO
    - iadd:                 COMPLETED
    - iaload:               TODO
    - iand:                 TODO
    - iastore:              TODO
    - iconst_<i>:           COMPLETED
    - idiv:                 COMPLETED
    - if_acmp<cond>:        TODO
    - if_icmp<cond>:        TODO
    - if<cond>:             TODO
    - ifnonnull:            TODO
    - ifnull:               TODO
    - iinc:                 TODO
    - iload:                COMPLETED
    - iload_<n>:            COMPLETED
    - imul:                 TODO
    - ineg:                 TODO
    - instanceof:           TODO
    - invokedynamic:        TODO
    - invokeinterface:      TODO
    - invokespecial:        TODO
    - invokestatic:         PARTIAL
    - invokevirtual:        TODO
    - ior:                  TODO
    - irem:                 TODO
    - ireturn:              TODO
    - ishl:                 TODO
    - ishr:                 TODO
    - istore:               COMPLETED
    - istore_<n>:           COMPLETED
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
    - lconst_<l>:           TODO
    - ldc:                  TODO
    - ldc_w:                TODO
    - ldc2_w:               INCOMPLETE
    - ldiv:                 TODO
    - lload:                COMPLETED
    - lload_<n>:            COMPLETED
    - lmul:                 TODO
    - lneg:                 TODO
    - lookupswitch:         TODO
    - lor:                  TODO
    - lrem:                 TODO
    - lreturn:              TODO
    - lshl:                 TODO
    - lshr:                 TODO
    - lstore:               COMPLETED
    - lstore_<n>:           COMPLETED
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
    - ret:                  DONE
    - return:               DONE
    - saload:               TODO
    - sastore:              TODO
    - sipush:               TODO
    - swap:                 TODO
    - tableswitch:          TODO
    - wide:                 TODO
*/
