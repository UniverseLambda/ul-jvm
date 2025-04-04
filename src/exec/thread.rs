pub struct JvmThread {
    pub pc: usize,
    pub stack: Vec<StackFrame>,
}

pub struct StackFrame {
    return_address: usize,
}
