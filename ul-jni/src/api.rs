use crate::ptr::JniPtr;

// Find ways to enforce hierarchy

pub type JniObject = JniPtr<()>;
pub type JniClass = JniObject;
pub type JniString = JniObject;
pub type JniArray = JniObject;
pub type JniObjectArray = JniArray;
pub type JniBooleanArray = JniArray;
pub type JniByteArray = JniArray;
pub type JniCharArray = JniArray;
pub type JniShortArray = JniArray;
pub type JniIntArray = JniArray;
pub type JniLongArray = JniArray;
pub type JniFloatArray = JniArray;
pub type JniDoubleArray = JniArray;
pub type JniThrowable = JniObject;

/*
TO IMPLEMENT TO LOAD libjava.so:
- U jio_vfprintf@SUNWprivate_1.1
- U jio_vsnprintf@SUNWprivate_1.1
- U JVM_ActiveProcessorCount@SUNWprivate_1.1
- U JVM_AddModuleExports@SUNWprivate_1.1
- U JVM_AddModuleExportsToAll@SUNWprivate_1.1
- U JVM_AddModuleExportsToAllUnnamed@SUNWprivate_1.1
- U JVM_AddReadsModule@SUNWprivate_1.1
- U JVM_AreNestMates@SUNWprivate_1.1
- U JVM_ArrayCopy@SUNWprivate_1.1
- U JVM_AssertionStatusDirectives@SUNWprivate_1.1
- U JVM_BeforeHalt@SUNWprivate_1.1
- U JVM_CallStackWalk@SUNWprivate_1.1
- U JVM_Clone@SUNWprivate_1.1
- U JVM_ConstantPoolGetClassAtIfLoaded@SUNWprivate_1.1
- U JVM_ConstantPoolGetClassAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetClassRefIndexAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetDoubleAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetFieldAtIfLoaded@SUNWprivate_1.1
- U JVM_ConstantPoolGetFieldAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetFloatAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetIntAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetLongAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetMemberRefInfoAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetMethodAtIfLoaded@SUNWprivate_1.1
- U JVM_ConstantPoolGetMethodAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetNameAndTypeRefIndexAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetNameAndTypeRefInfoAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetSize@SUNWprivate_1.1
- U JVM_ConstantPoolGetStringAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetTagAt@SUNWprivate_1.1
- U JVM_ConstantPoolGetUTF8At@SUNWprivate_1.1
- U JVM_CountStackFrames@SUNWprivate_1.1
- U JVM_CurrentThread@SUNWprivate_1.1
- U JVM_CurrentTimeMillis@SUNWprivate_1.1
- U JVM_DefineClassWithSource@SUNWprivate_1.1
- U JVM_DefineModule@SUNWprivate_1.1
- U JVM_DesiredAssertionStatus@SUNWprivate_1.1
- U JVM_DoPrivileged@SUNWprivate_1.1
- U JVM_DumpThreads@SUNWprivate_1.1
- U JVM_FillInStackTrace@SUNWprivate_1.1
- U JVM_FindClassFromBootLoader@SUNWprivate_1.1
- U JVM_FindClassFromCaller@SUNWprivate_1.1
- U JVM_FindLibraryEntry@SUNWprivate_1.1
- U JVM_FindLoadedClass@SUNWprivate_1.1
- U JVM_FindPrimitiveClass@SUNWprivate_1.1
- U JVM_FindSignal@SUNWprivate_1.1
- U JVM_FreeMemory@SUNWprivate_1.1
- U JVM_GC@SUNWprivate_1.1
- U JVM_GetAllThreads@SUNWprivate_1.1
- U JVM_GetAndClearReferencePendingList@SUNWprivate_1.1
- U JVM_GetArrayElement@SUNWprivate_1.1
- U JVM_GetArrayLength@SUNWprivate_1.1
- U JVM_GetCallerClass@SUNWprivate_1.1
- U JVM_GetClassAccessFlags@SUNWprivate_1.1
- U JVM_GetClassAnnotations@SUNWprivate_1.1
- U JVM_GetClassConstantPool@SUNWprivate_1.1
- U JVM_GetClassContext@SUNWprivate_1.1
- U JVM_GetClassDeclaredConstructors@SUNWprivate_1.1
- U JVM_GetClassDeclaredFields@SUNWprivate_1.1
- U JVM_GetClassDeclaredMethods@SUNWprivate_1.1
- U JVM_GetClassInterfaces@SUNWprivate_1.1
- U JVM_GetClassModifiers@SUNWprivate_1.1
- U JVM_GetClassSignature@SUNWprivate_1.1
- U JVM_GetClassSigners@SUNWprivate_1.1
- U JVM_GetClassTypeAnnotations@SUNWprivate_1.1
- U JVM_GetDeclaredClasses@SUNWprivate_1.1
- U JVM_GetDeclaringClass@SUNWprivate_1.1
- U JVM_GetEnclosingMethodInfo@SUNWprivate_1.1
- U JVM_GetFieldTypeAnnotations@SUNWprivate_1.1
- U JVM_GetInheritedAccessControlContext@SUNWprivate_1.1
- U JVM_GetInterfaceVersion@SUNWprivate_1.1
- U JVM_GetMethodParameters@SUNWprivate_1.1
- U JVM_GetMethodTypeAnnotations@SUNWprivate_1.1
- U JVM_GetNanoTimeAdjustment@SUNWprivate_1.1
- U JVM_GetNestHost@SUNWprivate_1.1
- U JVM_GetNestMembers@SUNWprivate_1.1
- U JVM_GetPrimitiveArrayElement@SUNWprivate_1.1
- U JVM_GetProtectionDomain@SUNWprivate_1.1
- U JVM_GetSimpleBinaryName@SUNWprivate_1.1
- U JVM_GetStackAccessControlContext@SUNWprivate_1.1
- U JVM_GetSystemPackages@SUNWprivate_1.1
- U JVM_GetSystemPackage@SUNWprivate_1.1
- U JVM_GetTemporaryDirectory@SUNWprivate_1.1
- U JVM_GetVmArguments@SUNWprivate_1.1
- U JVM_Halt@SUNWprivate_1.1
- U JVM_HasReferencePendingList@SUNWprivate_1.1
- U JVM_HoldsLock@SUNWprivate_1.1
- U JVM_IHashCode@SUNWprivate_1.1
- U JVM_InitAgentProperties@SUNWprivate_1.1
- U JVM_InitClassName@SUNWprivate_1.1
- U JVM_InitializeFromArchive@SUNWprivate_1.1
- U JVM_InitProperties@SUNWprivate_1.1
- U JVM_InitStackTraceElementArray@SUNWprivate_1.1
- U JVM_InitStackTraceElement@SUNWprivate_1.1
- U JVM_InternString@SUNWprivate_1.1
- U JVM_Interrupt@SUNWprivate_1.1
- U JVM_InvokeMethod@SUNWprivate_1.1
- U JVM_IsArrayClass@SUNWprivate_1.1
- U JVM_IsInterface@SUNWprivate_1.1
- U JVM_IsInterrupted@SUNWprivate_1.1
- U JVM_IsPrimitiveClass@SUNWprivate_1.1
- U JVM_IsSupportedJNIVersion@SUNWprivate_1.1
- U JVM_IsThreadAlive@SUNWprivate_1.1
- U JVM_IsUseContainerSupport@SUNWprivate_1.1
- U JVM_LatestUserDefinedLoader@SUNWprivate_1.1
- U JVM_LoadLibrary@SUNWprivate_1.1
- U JVM_MaxMemory@SUNWprivate_1.1
- U JVM_MonitorNotifyAll@SUNWprivate_1.1
- U JVM_MonitorNotify@SUNWprivate_1.1
- U JVM_MonitorWait@SUNWprivate_1.1
- U JVM_MoreStackWalk@SUNWprivate_1.1
- U JVM_NanoTime@SUNWprivate_1.1
- U JVM_NewArray@SUNWprivate_1.1
- U JVM_NewInstanceFromConstructor@SUNWprivate_1.1
- U JVM_NewMultiArray@SUNWprivate_1.1
- U JVM_RaiseSignal@SUNWprivate_1.1
- U JVM_RegisterSignal@SUNWprivate_1.1
- U JVM_ResumeThread@SUNWprivate_1.1
- U JVM_SetArrayElement@SUNWprivate_1.1
- U JVM_SetBootLoaderUnnamedModule@SUNWprivate_1.1
- U JVM_SetClassSigners@SUNWprivate_1.1
- U JVM_SetNativeThreadName@SUNWprivate_1.1
- U JVM_SetPrimitiveArrayElement@SUNWprivate_1.1
- U JVM_SetThreadPriority@SUNWprivate_1.1
- U JVM_Sleep@SUNWprivate_1.1
- U JVM_StartThread@SUNWprivate_1.1
- U JVM_StopThread@SUNWprivate_1.1
- U JVM_SupportsCX8@SUNWprivate_1.1
- U JVM_SuspendThread@SUNWprivate_1.1
- U JVM_TotalMemory@SUNWprivate_1.1
- U JVM_UnloadLibrary@SUNWprivate_1.1
- U JVM_WaitForReferencePendingList@SUNWprivate_1.1
- U JVM_Yield@SUNWprivate_1.1
*/
