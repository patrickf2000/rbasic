use std::process;
use super::string_utils;
use super::io_cmd;
use super::vars;
use super::loop_utils;
use super::conditional;

#[derive(Clone)]
pub struct Lbl {
	pub name:String,
	pub contents:Vec<String>,
}

//This is sent and returned from each label as it runs
#[derive(Clone)]
pub struct RunData {
	//Generic data
	pub code: i32,
	pub labels: Vec<Lbl>,
	pub vars: Vec<vars::Var>,
	pub shell_mode: bool,
	
	//Loop data
	pub in_loop: bool,
	pub loop_bd: Vec<String>,
	
	//Data for conditional statements
	pub last_was_if: bool,
	pub if_solved: bool,
}

//Builds a blank data label
pub fn build_data() -> RunData {
	let data = RunData {
		code: 0,
		labels: Vec::new(),
		vars: Vec::new(),
		shell_mode: false,
		in_loop: false,
		loop_bd: Vec::new(),
		last_was_if: false,
		if_solved: false,
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
	let mut first = string_utils::get_first(&line);
	let second = string_utils::get_second(&line);
	let mut ret_code:i32 = 0;
	
	let fc = first.chars().nth(0).unwrap();
	let lc = first.chars().last().unwrap();
	if fc != '#' && fc != '.' && fc != '$' && lc != ':' {
		first = first.to_uppercase();
	}
	
	//Make a quick conditional control check
	if (first != "ELIF" && first != "ELSE") && data.last_was_if {
		data.last_was_if = false;
		data.if_solved = false;
	}
	
	//Make sure we are not in a loop
	if data.in_loop && first != "WHILE" {
		data.loop_bd.push(line.clone());
		return data.clone();
	}
	
	//The PRINTLN command
	if first == "PRINTLN" {
		io_cmd::println(second.clone(), data.vars.clone());
		
	//The LET command
	//This defines variables
	} else if first == "LET" {
		let var = vars::create_var(second.clone(), data.vars.clone());
		data.vars.push(var);
		
	//The GOSUB command
	//This command executes another function and returns from it
	} else if first == "GOSUB" {
		if data.shell_mode {
			println!("GOSUB is not valid for use in shell mode.");
			return data;
		}
		
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
		if data.shell_mode {
			println!("GOTO is not valid for use in shell mode.");
			return data;
		}
	
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
		if data.shell_mode {
			println!("RETURN is not valid for use in shell mode.");
			return data;
		}
		//TODO: Implement
		
	//The EXIT command
	//This simply exits the program
	} else if first == "EXIT" {
		process::exit(0);
		
	//The DO command-> Starts a do-while loop
	} else if first == "DO" {
		data.in_loop = true;
		
	//The WHILE command-> Ends a do-while loop
	} else if first == "WHILE" {
		data = loop_utils::run_while_loop(second.clone(), data.clone());
		
	//The IF command-> First part of a conditional
	} else if first == "IF" {
		data = conditional::check_conditional(second.clone(), data.clone());
		data.last_was_if = true;
		
	//The ELIF command-> Second part of a conditional
	} else if first == "ELIF" {
		if data.last_was_if {
			if data.if_solved {
				return data;
			} else {
				data = conditional::check_conditional(second.clone(), data.clone());
				data.last_was_if = true;
			}
		} else {
			println!("Error: You cannot have \"ELIF\" without a previous IF.");
			process::exit(1);
		}
		
	//The ELSE command
	} else if first == "ELSE" {
		if data.last_was_if {
			if data.if_solved {
				return data;
			} else {
				data = run(second.clone(), data.clone());
				data.last_was_if = false;
				data.if_solved = false;
			}
		} else {
			println!("Error: You cannot have \"ELSE\" without a previous IF.");
			process::exit(1);
		}
		
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
