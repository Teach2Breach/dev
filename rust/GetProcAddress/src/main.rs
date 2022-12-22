use std::ffi::{CString, OsStr};
use std::mem::transmute;
use std::os::windows::ffi::{OsStrExt};
use winapi::shared::minwindef::{FARPROC, HMODULE};
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
use winapi::um::errhandlingapi::{GetLastError};

pub fn get_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

fn get_proc_address(module_name: &str, proc_name: &str) -> Result<FARPROC, String> {

    let module_name_wide = get_wide(module_name);
    println!("module_name_wide: {:?}", module_name_wide);

    let hmodule = unsafe { GetModuleHandleW(module_name_wide.as_ptr()) };
    if hmodule.is_null() {
        return Err(format!("Error: GetModuleHandleW failed with error code {}", unsafe { GetLastError() }));
    }
    println!("proc_name: {}", proc_name);
    let proc_name_lpcstr: Vec<i8> = CString::new(proc_name).unwrap().into_bytes_with_nul().into_iter().map(|x| x as i8).collect();

    let proc_address = unsafe {
        GetProcAddress(hmodule as HMODULE, proc_name_lpcstr.as_ptr())
    };
    if proc_address.is_null() {
        return Err(format!("Error: GetProcAddress failed with error code {}", unsafe { GetLastError() }));
    }

    Ok(proc_address as FARPROC)
}

fn main() {
    let kernel32_dll = "kernel32.dll";
    let get_current_process_id_proc = "GetCurrentProcessId";
    let get_current_process_id: extern "system" fn() -> u32 = match get_proc_address(kernel32_dll, get_current_process_id_proc) {
        Ok(proc_address) => unsafe { transmute(proc_address) },
        Err(error) => panic!("Error: {}", error),
    };
    let process_id = get_current_process_id();
    println!("Current process ID: {}", process_id);

    //wait for user input before exiting
    println!("Press any key to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();


}
