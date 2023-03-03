//use std::ptr::null_mut;
use winapi::um::processthreadsapi::{GetCurrentProcessId, OpenProcess};
use winapi::um::processsnapshot::{
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
};
use winapi::um::winnt::CONTEXT_ALL;
//use winapi::ctypes::c_void;

fn main() {
    // Declare variables
    let mut dw_pid: u32;

    //check for command-line arguments and print help if there are none
    if std::env::args().len() < 2 {
        //strip the path from the executable name
        let p = std::env::args().next().unwrap();
        let p = p.rsplit('\\').next().unwrap_or(&p);
        //print the usage with just the executable name
        println!("Usage: {} <PID>", p);
        println!("    or {} 0 (to capture the snapshot of the current process)", p);
        std::process::exit(1);
    }

    //convert the command-line argument to a DWORD
    dw_pid = std::env::args().nth(1).unwrap().parse().unwrap();

    //if PID is set to 0, get PID of current process
    if dw_pid == 0 {
        dw_pid = unsafe { GetCurrentProcessId() };
    }

    //get the PID from the command-line argument
    println!("PID: {}", dw_pid);

    //get handle to process
    let h_process = unsafe { OpenProcess(winapi::um::winnt::PROCESS_ALL_ACCESS, 0, dw_pid) };
    //print handle to process
    println!("Process handle: {:?}", h_process);

    let flags: PSS_CAPTURE_FLAGS = PSS_CAPTURE_VA_CLONE | PSS_CAPTURE_HANDLES | PSS_CAPTURE_HANDLE_NAME_INFORMATION | PSS_CAPTURE_HANDLE_BASIC_INFORMATION | PSS_CAPTURE_HANDLE_TYPE_SPECIFIC_INFORMATION | PSS_CAPTURE_HANDLE_TRACE | PSS_CAPTURE_THREADS | PSS_CAPTURE_THREAD_CONTEXT | PSS_CAPTURE_THREAD_CONTEXT_EXTENDED | PSS_CREATE_BREAKAWAY | PSS_CREATE_BREAKAWAY_OPTIONAL | PSS_CREATE_USE_VM_ALLOCATIONS | PSS_CREATE_RELEASE_SECTION;
    let mut snapshot_handle: HPSS = std::ptr::null_mut();
    let result = unsafe { PssCaptureSnapshot(h_process, flags, CONTEXT_ALL, &mut snapshot_handle) };

    //print the result of PssCaptureSnapshot
    println!("PssCaptureSnapshot result: {}", result);

    //print the snapshot handle
    println!("Snapshot handle: {:?}", snapshot_handle);

    //deref and print the snapshot handle
    //println!("Snapshot handle deref: {:?}", *snapshot_handle);
    //let mut snapshot_handle_c_void: *mut c_void = unsafe { std::mem::transmute(snapshot_handle) };
    //println!("Snapshot handle deref: {:?}", snapshot_handle_c_void);

    //free the snapshot
    let result = unsafe { PssFreeSnapshot(h_process, snapshot_handle) };

    //print the result of PssFreeSnapshot
    println!("PssFreeSnapshot result: {}", result);

    //close the process handle
    let result = unsafe { winapi::um::handleapi::CloseHandle(h_process) };

    //print the result of CloseHandle
    println!("CloseHandle result: {}", result);
}