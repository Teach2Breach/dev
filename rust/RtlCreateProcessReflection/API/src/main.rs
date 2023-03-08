#![allow(non_snake_case)]
#![allow(unused_assignments)]

use ntapi::{ntrtl::{RtlCreateProcessReflection, RTLP_PROCESS_REFLECTION_REFLECTION_INFORMATION}, winapi::shared::{ntdef::HANDLE, minwindef::DWORD}, ntapi_base::CLIENT_ID};
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::processthreadsapi::GetCurrentProcessId;

fn main() {

//the purpose of this program is to perform a proof of concept for the RtlCreateProcessReflection API, using Rust and API calls for this version.
//other version will make either direct calls to the function pointer, or use dinvoke.

//first we need a target process to clone, so we will use the current process as the target.

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

let result = unsafe { RtlCreateProcessReflection(process_handle, refl_flags, 0 as _, 0 as _, 0 as _, &mut info) };

//setup some vars to hold the new handle and pid

let mut new_handle: HANDLE = 0 as _;
let mut new_pid: DWORD = 0 as _;

//check the result of the API call and handle any errors.

if result == 0 {
    println!("Reflection successful");
    new_handle = info.ReflectionProcessHandle;
    //convert info.ReflectionClientId.UniqueProcess to DWORD
    new_pid = info.ReflectionClientId.UniqueProcess as DWORD;
    println!("New handle: {:x?}", new_handle);
    println!("New PID: {}", new_pid);
} else {
    println!("Process Cloning Failed!");
    println!("Error Code: {}", result);
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