use std::env;
use std::process;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::Write;

mod string_utils;
mod io_cmd;
mod vars;
mod utils;
mod loop_utils;
mod conditional;
mod interpreter;
mod lbl_utils;
mod string_cmd;

//Our main function
fn main() {
    let args:Vec<String> = env::args().collect();
    
    if args.len() == 1 {
    	shell_mode();
    } else {
    	file_mode(args);
    }
}

//Runs from a command shell
fn shell_mode() {
	println!("Welcome to rBasic v1.0");
	println!("You are in shell mode.");
	println!("");

	let mut data = interpreter::build_data();
	data.shell_mode = true;
	
	loop {
		print!("BAS> ");
		io::stdout().flush().unwrap();
		
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Unknown input");
		input = input.trim().to_string();
		
		if input.to_uppercase() == "EXIT" {
			process::exit(0);
		} else if input.len() == 0 {
			continue;
		} else if string_utils::get_first(&input).to_uppercase() == "REM" {
			continue;
		}
		
		data = interpreter::run(input, data.clone());
	}
}

//Runs from a file
fn file_mode(args:Vec<String>) {
    let path = args.iter().nth(1).unwrap().to_string();
    let mut contents:Vec<String> = Vec::new();
    
    let _file = match File::open(path.to_string()) {
    	Ok(file) => {
    		let reader = BufReader::new(&file);
    		
    		for ln in reader.lines() {
    			let line = ln.unwrap().trim().to_string();
    			if line.len() > 0 {
    				contents.push(line);
    			}
    		}
    	},
    	
    	Err(e) => {
    		println!("Fatal error.");
    		println!("{}",e);
    	}
    };
    
    contents = interpreter::trim_comments(contents);
    let labels:Vec<interpreter::Lbl> = interpreter::cache_labels(contents);
    
    let mut main_lbl = interpreter::Lbl {
    	name: "".to_string(),
    	contents: Vec::new(),
    	args: Vec::new(),
    };
    
    for lbl in labels.iter() {
    	if lbl.name == "MAIN" {
    		main_lbl = lbl.clone();
    	}
    }
    
    let mut data = interpreter::build_data();
    data.labels = labels;
    
    for ln in main_lbl.contents.iter() {
    	data = interpreter::run(ln.clone(), data.clone());
    	
    	if data.code == 1 {
    		break;
    	}
    }
}
