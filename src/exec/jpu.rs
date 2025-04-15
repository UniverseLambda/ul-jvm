use anyhow::{Context, anyhow, bail};

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

    pub fn getstatic(&self, thread: &mut JvmThread, cp_index: u16) -> anyhow::Result<()> {
        let field_ref = thread
            .current_frame()?
            .current_class
            .constant_pool
            .get_field_ref(cp_index)
            .ok_or_else(|| anyhow!("no field_ref"))?;

        let target_class = self
            .resolve_class(&field_ref.class.name)
            .context("getstatic")?;

        if !self.skip_static_init || target_class.name != thread.current_frame()?.current_class.name
        {
            self.init_static(thread, &target_class)?;
        }

        let zarma = target_class.read_static(&field_ref.name)?;

        thread.current_frame_mut()?.operand_stack.push(zarma);

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

    fn init_static(&self, thread: &mut JvmThread, class: &Class) -> anyhow::Result<()> {
        JvmThread::run_clinit_thread(self.env, class.clone())
    }
}
