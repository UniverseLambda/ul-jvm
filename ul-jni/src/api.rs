use std::ffi::c_char;

use crate::{
    ptr::JniPtr,
    types::{JniByte, JniInt, JniSize},
};

pub type FfiStr = *const c_char;

unsafe impl<JniEnv, JniObject> Sync for JniInterfaceFunctions<JniEnv, JniObject>
where
    JniEnv: Sync,
    JniObject: Sync,
{
}

unsafe impl<JniEnv, JniObject> Send for JniInterfaceFunctions<JniEnv, JniObject>
where
    JniEnv: Send,
    JniObject: Send,
{
}

#[repr(C)]
#[derive(Default)]
pub struct JniInterfaceFunctions<JniEnv, JniObject> {
    pub res0: Option<*const ()>,                           // TODO
    pub res1: Option<*const ()>,                           // 0TODO
    pub res2: Option<*const ()>,                           // TODO
    pub res3: Option<*const ()>,                           // TODO
    pub get_version: Option<fn(JniPtr<JniEnv>) -> JniInt>, // TODO
    pub define_class:
        Option<fn(JniPtr<JniEnv>, FfiStr, JniPtr<JniObject>, *const JniByte, JniSize)>, // TODO
    pub find_class: Option<fn(JniPtr<JniEnv>)>,            // TODO
    pub from_reflected_method: Option<*const ()>,          // TODO
    pub from_reflected_field: Option<*const ()>,           // TODO
    pub to_reflected_method: Option<*const ()>,            // TODO
    pub get_superclass: Option<*const ()>,                 // TODO
    pub is_assignable_from: Option<*const ()>,             // TODO
    pub to_reflected_field: Option<*const ()>,             // TODO
    pub throw: Option<*const ()>,                          // TODO
    pub throw_new: Option<*const ()>,                      // TODO
    pub exception_occurred: Option<*const ()>,             // TODO
    pub exception_describe: Option<*const ()>,             // TODO
    pub exception_clear: Option<*const ()>,                // TODO
    pub fatal_error: Option<*const ()>,                    // TODO
    pub push_local_frame: Option<*const ()>,               // TODO
    pub pop_local_frame: Option<*const ()>,                // TODO
    pub new_global_ref: Option<*const ()>,                 // TODO
    pub delete_global_ref: Option<*const ()>,              // TODO
    pub delete_local_ref: Option<*const ()>,               // TODO
    pub is_same_object: Option<*const ()>,                 // TODO
    pub new_local_ref: Option<*const ()>,                  // TODO
    pub ensure_local_capacity: Option<*const ()>,          // TODO
    pub alloc_object: Option<*const ()>,                   // TODO
    pub new_object: Option<*const ()>,                     // TODO
    pub new_object_v: Option<*const ()>,                   // TODO
    pub new_object_a: Option<*const ()>,                   // TODO
    pub get_object_class: Option<*const ()>,               // TODO
    pub is_instance_of: Option<*const ()>,                 // TODO
    pub get_method_id: Option<*const ()>,                  // TODO
    pub call_object_method: Option<*const ()>,             // TODO
    pub call_object_method_v: Option<*const ()>,           // TODO
    pub call_object_method_a: Option<*const ()>,           // TODO
    pub call_boolean_method: Option<*const ()>,            // TODO
    pub call_boolean_method_v: Option<*const ()>,          // TODO
    pub call_boolean_method_a: Option<*const ()>,          // TODO
    pub call_byte_method: Option<*const ()>,               // TODO
    pub call_byte_method_v: Option<*const ()>,             // TODO
    pub call_byte_method_a: Option<*const ()>,             // TODO
    pub call_char_method: Option<*const ()>,               // TODO
    pub call_char_method_v: Option<*const ()>,             // TODO
    pub call_char_method_a: Option<*const ()>,             // TODO
    pub call_short_method: Option<*const ()>,              // TODO
    pub call_short_method_v: Option<*const ()>,            // TODO
    pub call_short_method_a: Option<*const ()>,            // TODO
    pub call_int_method: Option<*const ()>,                // TODO
    pub call_int_method_v: Option<*const ()>,              // TODO
    pub call_int_method_a: Option<*const ()>,              // TODO
    pub call_long_method: Option<*const ()>,               // TODO
    pub call_long_method_v: Option<*const ()>,             // TODO
    pub call_long_method_a: Option<*const ()>,             // TODO
    pub call_float_method: Option<*const ()>,              // TODO
    pub call_float_method_v: Option<*const ()>,            // TODO
    pub call_float_method_a: Option<*const ()>,            // TODO
    pub call_double_method: Option<*const ()>,             // TODO
    pub call_double_method_v: Option<*const ()>,           // TODO
    pub call_double_method_a: Option<*const ()>,           // TODO
    pub call_void_method: Option<*const ()>,               // TODO
    pub call_void_method_v: Option<*const ()>,             // TODO
    pub call_void_method_a: Option<*const ()>,             // TODO
    pub call_nonvirtual_object_method: Option<*const ()>,  // TODO
    pub call_nonvirtual_object_method_v: Option<*const ()>, // TODO
    pub call_nonvirtual_object_method_a: Option<*const ()>, // TODO
    pub call_nonvirtual_boolean_method: Option<*const ()>, // TODO
    pub call_nonvirtual_boolean_method_v: Option<*const ()>, // TODO
    pub call_nonvirtual_boolean_method_a: Option<*const ()>, // TODO
    pub call_nonvirtual_byte_method: Option<*const ()>,    // TODO
    pub call_nonvirtual_byte_method_v: Option<*const ()>,  // TODO
    pub call_nonvirtual_byte_method_a: Option<*const ()>,  // TODO
    pub call_nonvirtual_char_method: Option<*const ()>,    // TODO
    pub call_nonvirtual_char_method_v: Option<*const ()>,  // TODO
    pub call_nonvirtual_char_method_a: Option<*const ()>,  // TODO
    pub call_nonvirtual_short_method: Option<*const ()>,   // TODO
    pub call_nonvirtual_short_method_v: Option<*const ()>, // TODO
    pub call_nonvirtual_short_method_a: Option<*const ()>, // TODO
    pub call_nonvirtual_int_method: Option<*const ()>,     // TODO
    pub call_nonvirtual_int_method_v: Option<*const ()>,   // TODO
    pub call_nonvirtual_int_method_a: Option<*const ()>,   // TODO
    pub call_nonvirtual_long_method: Option<*const ()>,    // TODO
    pub call_nonvirtual_long_method_v: Option<*const ()>,  // TODO
    pub call_nonvirtual_long_method_a: Option<*const ()>,  // TODO
    pub call_nonvirtual_float_method: Option<*const ()>,   // TODO
    pub call_nonvirtual_float_method_v: Option<*const ()>, // TODO
    pub call_nonvirtual_float_method_a: Option<*const ()>, // TODO
    pub call_nonvirtual_double_method: Option<*const ()>,  // TODO
    pub call_nonvirtual_double_method_v: Option<*const ()>, // TODO
    pub call_nonvirtual_double_method_a: Option<*const ()>, // TODO
    pub call_nonvirtual_void_method: Option<*const ()>,    // TODO
    pub call_nonvirtual_void_method_v: Option<*const ()>,  // TODO
    pub call_nonvirtual_void_method_a: Option<*const ()>,  // TODO
    pub get_field_id: Option<*const ()>,                   // TODO
    pub get_object_field: Option<*const ()>,               // TODO
    pub get_boolean_field: Option<*const ()>,              // TODO
    pub get_byte_field: Option<*const ()>,                 // TODO
    pub get_char_field: Option<*const ()>,                 // TODO
    pub get_short_field: Option<*const ()>,                // TODO
    pub get_int_field: Option<*const ()>,                  // TODO
    pub get_long_field: Option<*const ()>,                 // TODO
    pub get_float_field: Option<*const ()>,                // TODO
    pub get_double_field: Option<*const ()>,               // TODO
    pub set_object_field: Option<*const ()>,               // TODO
    pub set_boolean_field: Option<*const ()>,              // TODO
    pub set_byte_field: Option<*const ()>,                 // TODO
    pub set_char_field: Option<*const ()>,                 // TODO
    pub set_short_field: Option<*const ()>,                // TODO
    pub set_int_field: Option<*const ()>,                  // TODO
    pub set_long_field: Option<*const ()>,                 // TODO
    pub set_float_field: Option<*const ()>,                // TODO
    pub set_double_field: Option<*const ()>,               // TODO
    pub get_static_method_id: Option<*const ()>,           // TODO
    pub call_static_object_method: Option<*const ()>,      // TODO
    pub call_static_object_method_v: Option<*const ()>,    // TODO
    pub call_static_object_method_a: Option<*const ()>,    // TODO
    pub call_static_boolean_method: Option<*const ()>,     // TODO
    pub call_static_boolean_method_v: Option<*const ()>,   // TODO
    pub call_static_boolean_method_a: Option<*const ()>,   // TODO
    pub call_static_byte_method: Option<*const ()>,        // TODO
    pub call_static_byte_method_v: Option<*const ()>,      // TODO
    pub call_static_byte_method_a: Option<*const ()>,      // TODO
    pub call_static_char_method: Option<*const ()>,        // TODO
    pub call_static_char_method_v: Option<*const ()>,      // TODO
    pub call_static_char_method_a: Option<*const ()>,      // TODO
    pub call_static_short_method: Option<*const ()>,       // TODO
    pub call_static_short_method_v: Option<*const ()>,     // TODO
    pub call_static_short_method_a: Option<*const ()>,     // TODO
    pub call_static_int_method: Option<*const ()>,         // TODO
    pub call_static_int_method_v: Option<*const ()>,       // TODO
    pub call_static_int_method_a: Option<*const ()>,       // TODO
    pub call_static_long_method: Option<*const ()>,        // TODO
    pub call_static_long_method_v: Option<*const ()>,      // TODO
    pub call_static_long_method_a: Option<*const ()>,      // TODO
    pub call_static_float_method: Option<*const ()>,       // TODO
    pub call_static_float_method_v: Option<*const ()>,     // TODO
    pub call_static_float_method_a: Option<*const ()>,     // TODO
    pub call_static_double_method: Option<*const ()>,      // TODO
    pub call_static_double_method_v: Option<*const ()>,    // TODO
    pub call_static_double_method_a: Option<*const ()>,    // TODO
    pub call_static_void_method: Option<*const ()>,        // TODO
    pub call_static_void_method_v: Option<*const ()>,      // TODO
    pub call_static_void_method_a: Option<*const ()>,      // TODO
    pub get_static_field_id: Option<*const ()>,            // TODO
    pub get_static_object_field: Option<*const ()>,        // TODO
    pub get_static_boolean_field: Option<*const ()>,       // TODO
    pub get_static_byte_field: Option<*const ()>,          // TODO
    pub get_static_char_field: Option<*const ()>,          // TODO
    pub get_static_short_field: Option<*const ()>,         // TODO
    pub get_static_int_field: Option<*const ()>,           // TODO
    pub get_static_long_field: Option<*const ()>,          // TODO
    pub get_static_float_field: Option<*const ()>,         // TODO
    pub get_static_double_field: Option<*const ()>,        // TODO
    pub set_static_object_field: Option<*const ()>,        // TODO
    pub set_static_boolean_field: Option<*const ()>,       // TODO
    pub set_static_byte_field: Option<*const ()>,          // TODO
    pub set_static_char_field: Option<*const ()>,          // TODO
    pub set_static_short_field: Option<*const ()>,         // TODO
    pub set_static_int_field: Option<*const ()>,           // TODO
    pub set_static_long_field: Option<*const ()>,          // TODO
    pub set_static_float_field: Option<*const ()>,         // TODO
    pub set_static_double_field: Option<*const ()>,        // TODO
    pub new_string: Option<*const ()>,                     // TODO
    pub get_string_length: Option<*const ()>,              // TODO
    pub get_string_chars: Option<*const ()>,               // TODO
    pub release_string_chars: Option<*const ()>,           // TODO
    pub new_string_utf: Option<*const ()>,                 // TODO
    pub get_string_utf_length: Option<*const ()>,          // TODO
    pub get_string_utf_chars: Option<*const ()>,           // TODO
    pub release_string_utf_chars: Option<*const ()>,       // TODO
    pub get_array_length: Option<*const ()>,               // TODO
    pub new_object_array: Option<*const ()>,               // TODO
    pub get_object_array_element: Option<*const ()>,       // TODO
    pub set_object_array_element: Option<*const ()>,       // TODO
    pub new_boolean_array: Option<*const ()>,              // TODO
    pub new_byte_array: Option<*const ()>,                 // TODO
    pub new_char_array: Option<*const ()>,                 // TODO
    pub new_short_array: Option<*const ()>,                // TODO
    pub new_int_array: Option<*const ()>,                  // TODO
    pub new_long_array: Option<*const ()>,                 // TODO
    pub new_float_array: Option<*const ()>,                // TODO
    pub new_double_array: Option<*const ()>,               // TODO
    pub get_boolean_array_elements: Option<*const ()>,     // TODO
    pub get_byte_array_elements: Option<*const ()>,        // TODO
    pub get_char_array_elements: Option<*const ()>,        // TODO
    pub get_short_array_elements: Option<*const ()>,       // TODO
    pub get_int_array_elements: Option<*const ()>,         // TODO
    pub get_long_array_elements: Option<*const ()>,        // TODO
    pub get_float_array_elements: Option<*const ()>,       // TODO
    pub get_double_array_elements: Option<*const ()>,      // TODO
    pub release_boolean_array_elements: Option<*const ()>, // TODO
    pub release_byte_array_elements: Option<*const ()>,    // TODO
    pub release_char_array_elements: Option<*const ()>,    // TODO
    pub release_short_array_elements: Option<*const ()>,   // TODO
    pub release_int_array_elements: Option<*const ()>,     // TODO
    pub release_long_array_elements: Option<*const ()>,    // TODO
    pub release_float_array_elements: Option<*const ()>,   // TODO
    pub release_double_array_elements: Option<*const ()>,  // TODO
    pub get_boolean_array_region: Option<*const ()>,       // TODO
    pub get_byte_array_region: Option<*const ()>,          // TODO
    pub get_char_array_region: Option<*const ()>,          // TODO
    pub get_short_array_region: Option<*const ()>,         // TODO
    pub get_int_array_region: Option<*const ()>,           // TODO
    pub get_long_array_region: Option<*const ()>,          // TODO
    pub get_float_array_region: Option<*const ()>,         // TODO
    pub get_double_array_region: Option<*const ()>,        // TODO
    pub set_boolean_array_region: Option<*const ()>,       // TODO
    pub set_byte_array_region: Option<*const ()>,          // TODO
    pub set_char_array_region: Option<*const ()>,          // TODO
    pub set_short_array_region: Option<*const ()>,         // TODO
    pub set_int_array_region: Option<*const ()>,           // TODO
    pub set_long_array_region: Option<*const ()>,          // TODO
    pub set_float_array_region: Option<*const ()>,         // TODO
    pub set_double_array_region: Option<*const ()>,        // TODO
    pub register_natives: Option<*const ()>,               // TODO
    pub unregister_natives: Option<*const ()>,             // TODO
    pub monitor_enter: Option<*const ()>,                  // TODO
    pub monitor_exit: Option<*const ()>,                   // TODO
    pub get_java_vm: Option<*const ()>,                    // TODO
    pub get_string_region: Option<*const ()>,              // TODO
    pub get_string_utf_region: Option<*const ()>,          // TODO
    pub get_primitive_array_critical: Option<*const ()>,   // TODO
    pub release_primitive_array_critical: Option<*const ()>, // TODO
    pub get_string_critical: Option<*const ()>,            // TODO
    pub release_string_critical: Option<*const ()>,        // TODO
    pub new_weak_global_ref: Option<*const ()>,            // TODO
    pub delete_weak_global_ref: Option<*const ()>,         // TODO
    pub exception_check: Option<*const ()>,                // TODO
    pub new_direct_byte_buffer: Option<*const ()>,         // TODO
    pub get_direct_buffer_address: Option<*const ()>,      // TODO
    pub get_direct_buffer_capacity: Option<*const ()>,     // TODO
    pub get_object_ref_type: Option<*const ()>,            // TODO
    pub get_module: Option<*const ()>,                     // TODO
    pub is_virtual_thread: Option<*const ()>,              // TODO
    pub get_string_utf_length_as_long: Option<*const ()>,  // TODO
}

#[repr(i32)]
pub enum JniRetCode {
    Ok = 0,         /* success */
    Err = -1,       /* unknown error */
    Detached = -2,  /* thread detached from the VM */
    Version = (-3), /* JNI version error */
    Nomem = (-4),   /* not enough memory */
    Exist = (-5),   /* VM already created */
    Inval = (-6),   /* invalid arguments */
}

pub const JNI_VERSION_1_1: i32 = 0x00010001;
pub const JNI_VERSION_1_2: i32 = 0x00010002;
pub const JNI_VERSION_1_4: i32 = 0x00010004;
pub const JNI_VERSION_1_6: i32 = 0x00010006;
pub const JNI_VERSION_1_8: i32 = 0x00010008;
pub const JNI_VERSION_9: i32 = 0x00090000;
pub const JNI_VERSION_10: i32 = 0x000a0000;
pub const JNI_VERSION_19: i32 = 0x00130000;
pub const JNI_VERSION_20: i32 = 0x00140000;
pub const JNI_VERSION_21: i32 = 0x00150000;
pub const JNI_VERSION_24: i32 = 0x00180000;

// Find ways to enforce hierarchy

// pub type JniObject = JniPtr<()>;
// pub type JniClass = JniObject;
// pub type JniString = JniObject;
// pub type JniArray = JniObject;
// pub type JniObjectArray = JniArray;
// pub type JniBooleanArray = JniArray;
// pub type JniByteArray = JniArray;
// pub type JniCharArray = JniArray;
// pub type JniShortArray = JniArray;
// pub type JniIntArray = JniArray;
// pub type JniLongArray = JniArray;
// pub type JniFloatArray = JniArray;
// pub type JniDoubleArray = JniArray;
// pub type JniThrowable = JniObject;

// pub type JniFieldId = JniPtr<()>;
// pub type JniMethodId = JniPtr<()>;

/*
TO IMPLEMENT TO LOAD libjava.so:
- U jio_vfprintf@SUNWprivate_1.1                                TODO
- U jio_vsnprintf@SUNWprivate_1.1                               TODO
- U JVM_ActiveProcessorCount@SUNWprivate_1.1                    TODO
- U JVM_AddModuleExports@SUNWprivate_1.1                        TODO
- U JVM_AddModuleExportsToAll@SUNWprivate_1.1                   TODO
- U JVM_AddModuleExportsToAllUnnamed@SUNWprivate_1.1            TODO
- U JVM_AddReadsModule@SUNWprivate_1.1                          TODO
- U JVM_AreNestMates@SUNWprivate_1.1                            TODO
- U JVM_ArrayCopy@SUNWprivate_1.1                               TODO
- U JVM_AssertionStatusDirectives@SUNWprivate_1.1               TODO
- U JVM_BeforeHalt@SUNWprivate_1.1                              TODO
- U JVM_CallStackWalk@SUNWprivate_1.1                           TODO
- U JVM_Clone@SUNWprivate_1.1                                   TODO
- U JVM_ConstantPoolGetClassAtIfLoaded@SUNWprivate_1.1          TODO
- U JVM_ConstantPoolGetClassAt@SUNWprivate_1.1                  TODO
- U JVM_ConstantPoolGetClassRefIndexAt@SUNWprivate_1.1          TODO
- U JVM_ConstantPoolGetDoubleAt@SUNWprivate_1.1                 TODO
- U JVM_ConstantPoolGetFieldAtIfLoaded@SUNWprivate_1.1          TODO
- U JVM_ConstantPoolGetFieldAt@SUNWprivate_1.1                  TODO
- U JVM_ConstantPoolGetFloatAt@SUNWprivate_1.1                  TODO
- U JVM_ConstantPoolGetIntAt@SUNWprivate_1.1                    TODO
- U JVM_ConstantPoolGetLongAt@SUNWprivate_1.1                   TODO
- U JVM_ConstantPoolGetMemberRefInfoAt@SUNWprivate_1.1          TODO
- U JVM_ConstantPoolGetMethodAtIfLoaded@SUNWprivate_1.1         TODO
- U JVM_ConstantPoolGetMethodAt@SUNWprivate_1.1                 TODO
- U JVM_ConstantPoolGetNameAndTypeRefIndexAt@SUNWprivate_1.1    TODO
- U JVM_ConstantPoolGetNameAndTypeRefInfoAt@SUNWprivate_1.1     TODO
- U JVM_ConstantPoolGetSize@SUNWprivate_1.1                     TODO
- U JVM_ConstantPoolGetStringAt@SUNWprivate_1.1                 TODO
- U JVM_ConstantPoolGetTagAt@SUNWprivate_1.1                    TODO
- U JVM_ConstantPoolGetUTF8At@SUNWprivate_1.1                   TODO
- U JVM_CountStackFrames@SUNWprivate_1.1                        TODO
- U JVM_CurrentThread@SUNWprivate_1.1                           TODO
- U JVM_CurrentTimeMillis@SUNWprivate_1.1                       TODO
- U JVM_DefineClassWithSource@SUNWprivate_1.1                   TODO
- U JVM_DefineModule@SUNWprivate_1.1                            TODO
- U JVM_DesiredAssertionStatus@SUNWprivate_1.1                  TODO
- U JVM_DoPrivileged@SUNWprivate_1.1                            TODO
- U JVM_DumpThreads@SUNWprivate_1.1                             TODO
- U JVM_FillInStackTrace@SUNWprivate_1.1                        TODO
- U JVM_FindClassFromBootLoader@SUNWprivate_1.1                 TODO
- U JVM_FindClassFromCaller@SUNWprivate_1.1                     TODO
- U JVM_FindLibraryEntry@SUNWprivate_1.1                        TODO
- U JVM_FindLoadedClass@SUNWprivate_1.1                         TODO
- U JVM_FindPrimitiveClass@SUNWprivate_1.1                      TODO
- U JVM_FindSignal@SUNWprivate_1.1                              TODO
- U JVM_FreeMemory@SUNWprivate_1.1                              TODO
- U JVM_GC@SUNWprivate_1.1                                      TODO
- U JVM_GetAllThreads@SUNWprivate_1.1                           TODO
- U JVM_GetAndClearReferencePendingList@SUNWprivate_1.1         TODO
- U JVM_GetArrayElement@SUNWprivate_1.1                         TODO
- U JVM_GetArrayLength@SUNWprivate_1.1                          TODO
- U JVM_GetCallerClass@SUNWprivate_1.1                          TODO
- U JVM_GetClassAccessFlags@SUNWprivate_1.1                     TODO
- U JVM_GetClassAnnotations@SUNWprivate_1.1                     TODO
- U JVM_GetClassConstantPool@SUNWprivate_1.1                    TODO
- U JVM_GetClassContext@SUNWprivate_1.1                         TODO
- U JVM_GetClassDeclaredConstructors@SUNWprivate_1.1            TODO
- U JVM_GetClassDeclaredFields@SUNWprivate_1.1                  TODO
- U JVM_GetClassDeclaredMethods@SUNWprivate_1.1                 TODO
- U JVM_GetClassInterfaces@SUNWprivate_1.1                      TODO
- U JVM_GetClassModifiers@SUNWprivate_1.1                       TODO
- U JVM_GetClassSignature@SUNWprivate_1.1                       TODO
- U JVM_GetClassSigners@SUNWprivate_1.1                         TODO
- U JVM_GetClassTypeAnnotations@SUNWprivate_1.1                 TODO
- U JVM_GetDeclaredClasses@SUNWprivate_1.1                      TODO
- U JVM_GetDeclaringClass@SUNWprivate_1.1                       TODO
- U JVM_GetEnclosingMethodInfo@SUNWprivate_1.1                  TODO
- U JVM_GetFieldTypeAnnotations@SUNWprivate_1.1                 TODO
- U JVM_GetInheritedAccessControlContext@SUNWprivate_1.1        TODO
- U JVM_GetInterfaceVersion@SUNWprivate_1.1                     TODO
- U JVM_GetMethodParameters@SUNWprivate_1.1                     TODO
- U JVM_GetMethodTypeAnnotations@SUNWprivate_1.1                TODO
- U JVM_GetNanoTimeAdjustment@SUNWprivate_1.1                   TODO
- U JVM_GetNestHost@SUNWprivate_1.1                             TODO
- U JVM_GetNestMembers@SUNWprivate_1.1                          TODO
- U JVM_GetPrimitiveArrayElement@SUNWprivate_1.1                TODO
- U JVM_GetProtectionDomain@SUNWprivate_1.1                     TODO
- U JVM_GetSimpleBinaryName@SUNWprivate_1.1                     TODO
- U JVM_GetStackAccessControlContext@SUNWprivate_1.1            TODO
- U JVM_GetSystemPackages@SUNWprivate_1.1                       TODO
- U JVM_GetSystemPackage@SUNWprivate_1.1                        TODO
- U JVM_GetTemporaryDirectory@SUNWprivate_1.1                   TODO
- U JVM_GetVmArguments@SUNWprivate_1.1                          TODO
- U JVM_Halt@SUNWprivate_1.1                                    TODO
- U JVM_HasReferencePendingList@SUNWprivate_1.1                 TODO
- U JVM_HoldsLock@SUNWprivate_1.1                               TODO
- U JVM_IHashCode@SUNWprivate_1.1                               TODO
- U JVM_InitAgentProperties@SUNWprivate_1.1                     TODO
- U JVM_InitClassName@SUNWprivate_1.1                           TODO
- U JVM_InitializeFromArchive@SUNWprivate_1.1                   TODO
- U JVM_InitProperties@SUNWprivate_1.1                          TODO
- U JVM_InitStackTraceElementArray@SUNWprivate_1.1              TODO
- U JVM_InitStackTraceElement@SUNWprivate_1.1                   TODO
- U JVM_InternString@SUNWprivate_1.1                            TODO
- U JVM_Interrupt@SUNWprivate_1.1                               TODO
- U JVM_InvokeMethod@SUNWprivate_1.1                            TODO
- U JVM_IsArrayClass@SUNWprivate_1.1                            TODO
- U JVM_IsInterface@SUNWprivate_1.1                             TODO
- U JVM_IsInterrupted@SUNWprivate_1.1                           TODO
- U JVM_IsPrimitiveClass@SUNWprivate_1.1                        TODO
- U JVM_IsSupportedJNIVersion@SUNWprivate_1.1                   TODO
- U JVM_IsThreadAlive@SUNWprivate_1.1                           TODO
- U JVM_IsUseContainerSupport@SUNWprivate_1.1                   TODO
- U JVM_LatestUserDefinedLoader@SUNWprivate_1.1                 TODO
- U JVM_LoadLibrary@SUNWprivate_1.1                             TODO
- U JVM_MaxMemory@SUNWprivate_1.1                               TODO
- U JVM_MonitorNotifyAll@SUNWprivate_1.1                        TODO
- U JVM_MonitorNotify@SUNWprivate_1.1                           TODO
- U JVM_MonitorWait@SUNWprivate_1.1                             TODO
- U JVM_MoreStackWalk@SUNWprivate_1.1                           TODO
- U JVM_NanoTime@SUNWprivate_1.1                                TODO
- U JVM_NewArray@SUNWprivate_1.1                                TODO
- U JVM_NewInstanceFromConstructor@SUNWprivate_1.1              TODO
- U JVM_NewMultiArray@SUNWprivate_1.1                           TODO
- U JVM_RaiseSignal@SUNWprivate_1.1                             TODO
- U JVM_RegisterSignal@SUNWprivate_1.1                          TODO
- U JVM_ResumeThread@SUNWprivate_1.1                            TODO
- U JVM_SetArrayElement@SUNWprivate_1.1                         TODO
- U JVM_SetBootLoaderUnnamedModule@SUNWprivate_1.1              TODO
- U JVM_SetClassSigners@SUNWprivate_1.1                         TODO
- U JVM_SetNativeThreadName@SUNWprivate_1.1                     TODO
- U JVM_SetPrimitiveArrayElement@SUNWprivate_1.1                TODO
- U JVM_SetThreadPriority@SUNWprivate_1.1                       TODO
- U JVM_Sleep@SUNWprivate_1.1                                   TODO
- U JVM_StartThread@SUNWprivate_1.1                             TODO
- U JVM_StopThread@SUNWprivate_1.1                              TODO
- U JVM_SupportsCX8@SUNWprivate_1.1                             TODO
- U JVM_SuspendThread@SUNWprivate_1.1                           TODO
- U JVM_TotalMemory@SUNWprivate_1.1                             TODO
- U JVM_UnloadLibrary@SUNWprivate_1.1                           TODO
- U JVM_WaitForReferencePendingList@SUNWprivate_1.1             TODO
- U JVM_Yield@SUNWprivate_1.1                                   TODO
*/
