use std::process;
use super::string_utils;
use super::io;
use super::vars;

#[derive(Clone)]
pub struct Lbl {
	pub name:String,
	pub contents:Vec<String>,
}

//This is sent and returned from each label as it runs
#[derive(Clone)]
pub struct RunData {
	pub code: i32,
	pub labels: Vec<Lbl>,
	pub vars: Vec<vars::Var>,
}

//Builds a blank data label
pub fn build_data() -> RunData {
	let data = RunData {
		code: 0,
		labels: Vec::new(),
		vars: Vec::new(),
	};
	
	data
}

//Removes any comments from contents
pub fn trim_comments(contents:Vec<String>) -> Vec<String> {
	let mut code:Vec<String> = Vec::new();
	
	for ln in contents {
		let first = string_utils::get_first(&ln);
		if first != "REM" {
			code.push(ln);
		}
	}
	
	code
}

//Converts the file into a vector of functions
pub fn cache_labels(contents:Vec<String>) -> Vec<Lbl> {
	let mut labels:Vec<Lbl> = Vec::new();
	let mut first = false;
	
	let mut current = Lbl {
		name: "".to_string(),
		contents: Vec::new(),
	};

	for ln in contents {
		let lc:char = ln.chars().last().unwrap();
		
		if lc == ':' {
			if first {
				labels.push(current.clone());
			} else {
				first = true;
			}
			
			current = Lbl {
				name: string_utils::lbl_name(&ln),
				contents: Vec::new(),
			}
		} else {
			current.contents.push(ln.clone());
		}
	}
	
	labels.push(current.clone());
	
	labels
}

//Returns a particular label from a vector
pub fn find_label(labels:Vec<Lbl>, name:String) -> Lbl {
	let mut lbl = Lbl {
		name: "".to_string(),
		contents: Vec::new(),
	};
	
	for l in labels.iter() {
		if name == l.name {
			lbl = l.clone();
		}
	}
	
	lbl
}

//The main interpreter-> Interprets the individual layers
pub fn run(line:String, mut data:RunData) -> RunData {
	let first = string_utils::get_first(&line);
	let second = string_utils::get_second(&line);
	let mut ret_code:i32 = 0;
	
	//The PRINTLN command
	if first == "PRINTLN" {
		io::println(second.clone(), data.vars.clone());
		
	//The LET command
	//This defines variables
	} else if first == "LET" {
		let var = vars::create_var(second.clone(), data.vars.clone());
		data.vars.push(var);
		
	//The GOSUB command
	//This command executes another function and returns from it
	} else if first == "GOSUB" {
		let lbl = find_label(data.labels.clone(), second);
		let mut sub_data = build_data();
		sub_data.labels = data.labels.clone();
			
		for ln in lbl.contents.iter() {
			sub_data = run(ln.clone() ,sub_data.clone());
			
			if sub_data.code == 1 {
				ret_code = sub_data.code;
				break;
			}
		}
		
	//The GOTO command
	//This command goes to another function and stops the current function
	} else if first == "GOTO" {
		let lbl = find_label(data.labels.clone(), second);
		
		for ln in lbl.contents.iter() {
			let mut sub_data = build_data();
			sub_data.labels = data.labels.clone();
			sub_data = run(ln.clone() ,sub_data.clone());
			
			if sub_data.code == 1 {
				break;
			}
		}
		
		ret_code = 1;
		
	//The RETURN command
	} else if first == "RETURN" {
		//TODO: Implement
		
	//The EXIT command
	//This simply exits the program
	} else if first == "EXIT" {
		process::exit(0);
	
	} else {
		let fc = first.chars().nth(0).unwrap();
		
		//Check to see if we have a variable as a command
		if fc == '#' || fc == '.' || fc == '$' {
			let vname = vars::var_name(&first);
			let sec = string_utils::get_second(&second);
			data.vars = vars::def_var(vname, sec, data.vars.clone());
		
		//Unknown command
		} else {
			println!("Error: Unknown command.");
		}
	}
	
	data.code = ret_code;
	data.clone()
}
