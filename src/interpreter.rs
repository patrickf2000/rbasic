use std::process;
use super::string_utils;
use super::io_cmd;
use super::vars;
use super::vars::Var;
use super::loop_utils;
use super::conditional;
use super::utils;
use super::lbl_utils;
use super::string_cmd;

#[derive(Clone)]
pub struct Lbl {
	pub name:String,
	pub contents:Vec<String>,
	pub args:Vec<Var>,
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
	pub lp_layer: i32,
	pub break_req: bool,
	
	//Data for conditional statements
	pub last_was_if: bool,
	pub if_solved: bool,
	
	//Memory control
	pub memory: String,
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
		lp_layer: 0,
		break_req: false,
		last_was_if: false,
		if_solved: false,
		memory: String::new(),
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
		args: Vec::new(),
	};

	//Load all contents into the labels
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
				args: lbl_utils::build_args(&ln),
			};
		} else {
			current.contents.push(ln.clone());
		}
	}
	
	labels.push(current.clone());
	
	labels
}

//Returns a particular label from a vector
pub fn find_label(labels:Vec<Lbl>, name:String) -> Lbl {
	let lbl_name = string_utils::lbl_name(&name);

	let mut lbl = Lbl {
		name: "".to_string(),
		contents: Vec::new(),
		args: Vec::new(),
	};
	
	for l in labels.iter() {
		if lbl_name == l.name {
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
	if data.in_loop {
		if first == "DO" {
			data.lp_layer += 1;
			data.loop_bd.push(line.clone());
			return data.clone();
		} else if first == "WHILE" && data.lp_layer > 1 {
			data.loop_bd.push(line.clone());
			data.lp_layer -= 1;
			return data.clone();
		} else if first != "WHILE" {
			data.loop_bd.push(line.clone());
			return data.clone();
		}
	}
	
	//The PRINTLN command
	if first == "PRINTLN" {
		data = io_cmd::print(second.clone(), data.clone(), true);
		
	//The PRINT command
	} else if first == "PRINT" {
		data = io_cmd::print(second.clone(), data.clone(), false);
		
	//The CLS command
	} else if first == "CLS" {
		println!("\x1b[2J\x1b[1;1H");
		
	//The INPUT command
	} else if first == "INPUT" {
		data.vars = io_cmd::input(second.clone(), data.vars.clone(), &mut data);
		
	//The LET command
	//This defines variables
	} else if first == "LET" {
		let var = vars::create_var(second.clone(), data.vars.clone(), &mut data);
		data.vars.push(var);
		
	//The BREAK command
	} else if first == "BREAK" {
		data.break_req = true;
		
	//The LEN command
	//Returns the length of a string
	} else if first == "LEN" {
		string_cmd::len(second.clone(), &mut data);
		
	//The CHAR command
	//Returns the character at a particular index
	} else if first == "CHAR" {
		string_cmd::char_cmd(second.clone(), &mut data);
		
	//The RAND command
	//Generates a random number up to a certain max (Syntax: RAND 10)
	} else if first == "RAND" {
		utils::gen_random_int(second.clone(), &mut data);
		
	//The GOSUB command
	//This command executes another function and returns from it
	} else if first == "GOSUB" {
		if data.shell_mode {
			println!("GOSUB is not valid for use in shell mode.");
			return data;
		}
		
		let mut sub_data = build_data();
		sub_data.labels = data.labels.clone();
		
		let mut lbl = find_label(data.labels.clone(), second.clone());
		lbl.args = lbl_utils::assign_args(lbl.clone(), &second, &mut data);
		sub_data.vars = vars::merge_vars(lbl.args.clone(), sub_data.vars.clone());
		
		for ln in lbl.contents.iter() {
			sub_data = run(ln.clone() ,sub_data.clone());
			
			if sub_data.code == 1 {
				ret_code = sub_data.code;
				break;
			}
		}
		
		data.memory = sub_data.memory.clone();
		
	//The GOTO command
	//This command goes to another function and stops the current function
	} else if first == "GOTO" {
		if data.shell_mode {
			println!("GOTO is not valid for use in shell mode.");
			return data;
		}
	
		let mut sub_data = build_data();
		sub_data.labels = data.labels.clone();
		
		let mut lbl = find_label(data.labels.clone(), second.clone());
		lbl.args = lbl_utils::assign_args(lbl.clone(), &second, &mut data);
		sub_data.vars = vars::merge_vars(lbl.args.clone(), sub_data.vars.clone());
		
		for ln in lbl.contents.iter() {
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
		data = utils::handle_return(second.clone(), data.clone());
		
	//The MEMSET command
	} else if first == "MEMSET" {
		data.vars = vars::def_var(second.clone(), data.memory.clone(), data.vars.clone(), &mut data);
		
	//The EXIT command
	//This simply exits the program
	} else if first == "EXIT" {
		process::exit(0);
		
	//The DO command-> Starts a do-while loop
	} else if first == "DO" {
		data.in_loop = true;
		data.lp_layer += 1;
		
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
			data.vars = vars::def_var(vname, sec, data.vars.clone(), &mut data);
		
		//Unknown command
		} else {
			println!("Error: Unknown command.");
		}
	}
	
	data.code = ret_code;
	data.clone()
}
