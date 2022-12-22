#![allow(dead_code)]
use std::fs::File;
use std::io::Error;
use winapi::um::errhandlingapi::GetLastError;

use crate::humanerror::ERROR_MAP;
mod humanerror;
//use humanerror::ERROR_MAP;

fn main() -> Result<(), Error> {
  // Open a file that does not exist
  let _file_handle = File::open("does_not_exist.txt")?;

  // Get the error code from the return value of File::open
  let error_code = unsafe {GetLastError()};

  // Look up the error message in the error map
  let error_message = ERROR_MAP.iter().find(|x| x.error_code == error_code).map(|x| x.error_message).unwrap_or("Unknown error.");

  // Print the error code and error message
  println!("Windows error code {}: {}", error_code, error_message);

  Ok(())
}
