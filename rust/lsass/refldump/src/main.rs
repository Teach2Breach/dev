#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use safetydump::in_memory_dump;
use std::fs;
use std::io::Write;
use random_string::generate;

fn main() {

	let args: Vec<String> = std::env::args().collect();

	let args_clone = args.clone();

fn check_args(args: Vec<String>) -> Vec<String> {

	if args.len() == 1 {
		return vec!["safetydump".to_string(), "0".to_string()];
	}
	else {
		if args[1].parse::<i32>().is_ok() {
			return vec!["safetydump".to_string(), args[1].to_string()];
		}
		else {
			return vec!["safetydump".to_string(), "0".to_string()];
		}
	}

} 

	let safety_args = check_args(args);	
	let safety_args: Vec<&str> = safety_args.iter().map(|x| x.as_str()).collect();

	//println!("{:?}", safety_args);

    let buf_b64 = in_memory_dump(safety_args);
	    let buf = buf_b64.clone();

	//println!("{:?}", buf);

fn filename (args: Vec<String>) -> Vec<String> {
	if args.contains(&"classic".to_string()) {
		return vec!["classic".to_string()];
	}
	else {
		let charset = "abcdefghijklmnopqrstuvwxyz";	
		return vec![format!("{}", generate(8, charset))];
	}
}
	let file_name_vec: Vec<String> = filename(args_clone);	
	let file_name = file_name_vec[0].to_string();

	let file_name_bin = format!("{}.bin", file_name.clone());

	let mut output_file = fs::File::create(&file_name_bin.clone()).expect("could not write f");
	output_file.write_all(buf.as_bytes()).expect("could not write contents");

	println!("filename: {:?}", file_name_bin.clone());
	println!("done");

	//for debugging, decode the base64 dump and write to a file
	//let buf_decoded = base64::decode(&buf).unwrap();
	//let mut output_file = fs::File::create("dump.bin").expect("could not write f");
	//output_file.write_all(&buf_decoded).expect("could not write contents");

	//println!("debugging message. dump.bin created (base64 decoded dump.bin)")

}
