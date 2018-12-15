use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod string_utils;
mod io;
mod vars;
mod interpreter;

fn main() {
    let args:Vec<String> = env::args().collect();
    
    if args.len() == 1 {
    	println!("Error: You must specify a file.");
    	process::exit(1);
    }
    
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
    };
    
    for lbl in labels.iter() {
    	if lbl.name == "MAIN" {
    		main_lbl = lbl.clone();
    	}
    }
    
    let mut data = interpreter::RunData {
    	code: 0,
    	labels: labels,
    	vars: Vec::new(),
    };
    
    for ln in main_lbl.contents.iter() {
    	data = interpreter::run(ln.clone(), data.clone());
    	
    	if data.code == 1 {
    		break;
    	}
    }
}
