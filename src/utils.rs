extern crate rand;
use rand::Rng;
use std::process;

use super::interpreter::RunData;

#[derive(Clone)]
pub struct Condition {
	pub part1:String,
	pub part2:String,
	pub operator:String,
}

//Breaks down a conditional statement
pub fn get_condition(line:&String) -> Condition {
	let mut condition = Condition {
		part1: String::new(),
		part2: String::new(),
		operator: String::new(),
	};
	
	let mut var = String::new();
	let mut op = String::new();
	let mut index = String::new();
	let mut found_first = false;
	let mut found_op = false;
	
	for c in line.chars() {
		if c == '<' || c == '>' || c == '=' || c == '!' {
			if !found_op {
				op.push(c);
				if !found_first {
					found_first = true;
				}
			}
		} else if c == ' ' {
			continue;
		} else {
			if found_first {
				if !found_op {
					found_op = true;
				}
				index.push(c);
			} else {
				var.push(c);
			}
		}
	}
	
	condition.part1 = var;
	condition.part2 = index;
	condition.operator = op;
	
	condition
}

//Checks to see if two numbers are integers
pub fn is_int(_no1:String, _no2:String) -> bool {
	let no1_is:bool;
	let no2_is:bool;
	
	match _no1.parse::<i32>() {
		Ok(_n) => no1_is = true,
		Err(_n) => no1_is = false,
	}
	
	match _no2.parse::<i32>() {
		Ok(_n) => no2_is = true,
		Err(_n) => no2_is = false,
	}
	
	if no1_is && no2_is {
		return true;
	}
	
	false
}

//Checks to see if two numbers are doubles
pub fn is_double(_no1:String, _no2:String) -> bool {
	let no1_is:bool;
	let no2_is:bool;
	
	match _no1.parse::<f32>() {
		Ok(_n) => no1_is = true,
		Err(_n) => no1_is = false,
	}
	
	match _no2.parse::<f32>() {
		Ok(_n) => no2_is = true,
		Err(_n) => no2_is = false,
	}
	
	if no1_is && no2_is {
		return true;
	}
	
	false
}

//Handles return commands
pub fn handle_return(line:String, mut data:RunData) -> RunData {
	data.memory = line.clone();
	
	for v in data.vars.iter() {
		if v.name == line {
			data.memory = v.value.clone();
		}
	}

	data.clone()
}

//Checks to see if a string is a function call
pub fn is_cmd(line:String) -> bool {
	let fc:char = line.chars().nth(0).unwrap();
	let lc:char = line.chars().last().unwrap();
	
	if fc == '(' || lc == ')' {
		return true;
	}
	
	false
}

//Generates a random number and saves it to memory
pub fn gen_random_int(max:String, data:&mut RunData) {
	let seed:i32;
	let mut to_parse = max.clone();
		
	for v in data.vars.iter() {
		if v.name == max {
			to_parse = v.value.clone();
		}
	}
		
	match to_parse.parse::<i32>() {
		Ok(n) => seed = n,
		Err(_n) => {
			println!("Error: Invalid integer specified as seed.");
			process::exit(1);
		},
	}
		
	let num:i32 = rand::thread_rng().gen_range(0,seed+1);
	data.memory = num.to_string();
}
