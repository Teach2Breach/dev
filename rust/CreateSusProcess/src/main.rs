use winapi::shared::minwindef::DWORD;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::processthreadsapi::{CreateProcessW, ResumeThread, STARTUPINFOW, PROCESS_INFORMATION};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::CREATE_SUSPENDED;
use std::ffi::OsString;
use std::os::windows::prelude::OsStrExt;
use std::{ptr, mem};

fn main() {
        let mut process_info = PROCESS_INFORMATION {
            hProcess: ptr::null_mut(),
            hThread: ptr::null_mut(),
            dwProcessId: 0,
            dwThreadId: 0,
        };
    
        let cmd = OsString::from("calc.exe");
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
    
        use winapi::um::securitybaseapi::{
            InitializeSecurityDescriptor,
            SetSecurityDescriptorDacl,
        };
        use winapi::um::winnt::{SECURITY_DESCRIPTOR};
        
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
        InitializeSecurityDescriptor(
            std::mem::transmute(&mut security_descriptor),
            1,
        );
    
    }
        
        // Set the DACL to allow full access to the thread handle
        unsafe {
            SetSecurityDescriptorDacl(
                &mut security_descriptor as *mut SECURITY_DESCRIPTOR as *mut winapi::ctypes::c_void,
                1,
                0 as *mut _,
                0,
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
                &mut process_info,
            )
        };
        
            
            
    
        if create_success == 0 {
            // handle error
            println!("Error creating process: {}", unsafe { GetLastError() });
        } else {
            // process was created successfully in a suspended state
            println!("Process created successfully");
        }

        // Resume the thread

        let resume_success = unsafe { ResumeThread(process_info.hThread) };
        if resume_success == 0xFFFFFFFF {
            // handle error
            println!("Error resuming thread: {}", unsafe { GetLastError() });
        } else {
            // thread was resumed successfully
            println!("Thread resumed successfully");
        }

        // Close the thread handle
        unsafe {
            winapi::um::handleapi::CloseHandle(process_info.hThread);
        }

        // Close the process handle
        unsafe {
            winapi::um::handleapi::CloseHandle(process_info.hProcess);
        }

        return;

}
