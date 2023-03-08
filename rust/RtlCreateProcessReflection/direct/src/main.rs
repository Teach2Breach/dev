#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(non_camel_case_types)]

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use ntapi::{winapi::shared::{ntdef::HANDLE, minwindef::DWORD}, ntapi_base::CLIENT_ID};
use winapi::{um::{
    libloaderapi::{GetProcAddress, LoadLibraryW}, // resolve these dynamically
    processthreadsapi::{GetCurrentProcessId, GetCurrentProcess} // use these directly
}, shared::{minwindef::{HMODULE, FARPROC, ULONG}, ntdef::{PVOID, NTSTATUS}}};

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

pub fn get_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

fn main() {

//the purpose of this program is to perform a proof of concept for the RtlCreateProcessReflection API, using Rust and dynamically resolving the API call.

//first we need to dynamically resolve the RtlCreateProcessReflection API call, which is located in ntdll.dll.
let ntdll_handle = get_dll("ntdll.dll");
let rcp_func = get_fn(ntdll_handle, "RtlCreateProcessReflection\0");

// define functions, structs, and enums needed for the API call

//define PRTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION
type PRTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION = *mut RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION;
//define RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION
#[repr(C)]
    pub struct RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION {
    pub ReflectionProcessHandle: HANDLE,
    pub ReflectionThreadHandle: HANDLE,
    pub ReflectionClientId: CLIENT_ID,
    }
    
//adding RtlCreateProcessReflection function
    let RtlCreateProcessReflection: unsafe fn(
        HANDLE, 
        ULONG, 
        PVOID, 
        PVOID, 
        HANDLE, 
        PRTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION,
    ) -> NTSTATUS = unsafe { std::mem::transmute(rcp_func as FARPROC) };

//now we need a target process to clone, so we will use the current process as the target.

//we need to get the current process handle

let mut process_handle: HANDLE = 0 as HANDLE;

let mut process_id: DWORD = 0 as DWORD;

unsafe { process_handle = GetCurrentProcess(); }

unsafe { process_id = GetCurrentProcessId(); }

println!("Getting current process handle and ID...");
println!("Process Handle: {:x?}", process_handle);
println!("Process ID: {}", process_id);

//wait for user input so user has a chance to see the output and check their process handle and ID.

println!("Press any key to continue and clone process...");
let mut input = String::new();
std::io::stdin().read_line(&mut input).expect("Failed to read line");

//now that we have a handle, let's set up our API call to RtlCreateProcessReflection, which will clone the target process.

//we need to set up the flags, which are as follows:

//RTL_CREATE_PROCESS_PARAMS_FLAG_NO_DEBUG_INHERIT: 0x00000002

let refl_flags = 0x00000002;

//we need to set up the RTL_USER_PROCESS_PARAMETERS structure, which is as follows:

let mut info: RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION = RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION {
    ReflectionProcessHandle: 0 as _,
    ReflectionThreadHandle: 0 as _,
    ReflectionClientId: CLIENT_ID {
        UniqueProcess: 0 as _,
        UniqueThread: 0 as _,
    },
};

//rest of the values are optional, so we can leave them as 0.

//call RtlCreateProcessReflection
    let status = unsafe { RtlCreateProcessReflection (
        process_handle,
        refl_flags,
        0 as _,
        0 as _,
        0 as _,
        &mut info,
    )};
//setup some vars to hold the new handle and pid

let mut new_handle: HANDLE = 0 as _;
let mut new_pid: DWORD = 0 as _;

//check the result of the API call and handle any errors.

if status == 0 {
    println!("Reflection successful");
    new_handle = info.ReflectionProcessHandle;
    //convert info.ReflectionClientId.UniqueProcess to DWORD
    new_pid = info.ReflectionClientId.UniqueProcess as DWORD;
    println!("New handle: {:x?}", new_handle);
    println!("New PID: {}", new_pid);
} else {
    println!("Process Cloning Failed!");
    println!("Error Code: {}", status);
}

//wait for user input so user has a chance to see the output and check their process handle and ID.

println!("Press any key to close handles and exit...");
let mut input = String::new();
std::io::stdin().read_line(&mut input).expect("Failed to read line");

//close the new process handle

let mut close_result: i32 = 0;

unsafe { close_result = winapi::um::handleapi::CloseHandle(new_handle); }

if close_result == 0 {
    println!("Failed to close handle!");
} else {
    println!("Handle closed successfully!");
}

}
