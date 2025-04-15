use std::any::TypeId;

pub struct JniPtr<P> {
    magic: u32, // 0xDEADBEEF
    typeid: TypeId,
    ptr: *const P,
    magic_end: u32, // 0xCAFEBABE
}

impl<P: 'static> JniPtr<P> {
    pub fn new(v: &'static P) -> Self {
        Self {
            magic: 0xDEADBEEF,
            typeid: TypeId::of::<P>(),
            ptr: v as *const P,
            magic_end: 0xCAFEBABE,
        }
    }

    pub fn get_ref(&self) -> &'static P {
        assert_eq!(self.magic, 0xDEADBEEF);
        assert_eq!(self.typeid, TypeId::of::<P>());
        assert_eq!(self.magic_end, 0xCAFEBABE);

        unsafe { self.ptr.as_ref().unwrap() }
    }
}
