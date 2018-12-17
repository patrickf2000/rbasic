//ios.rs
//Handles the Input/output commands
use std::io;

use super::vars;
use super::string_utils;

//The PRINTLN command
pub fn println(line:String, vars:Vec<vars::Var>) {
	let fc = line.chars().nth(0).unwrap();
	let lc = line.chars().last().unwrap();
	let mut to_print = String::new();
	
	if fc == '"' && lc == '"' {		//We have hardcoded text to print
		to_print = string_utils::rm_quotes(&line);
	} else {						//We have a variable to print
		for v in vars.iter() {
			if v.name == line {
				if v.data_type == "str" {
					to_print = string_utils::rm_quotes(&v.value);
				} else {
					to_print = v.value.clone();
				}
			}
		}
	}
	
	//Print out our text
	println!("{}",to_print);
}

//The INPUT command
pub fn input(line:String, vars:Vec<vars::Var>) -> Vec<vars::Var> {
	let mut vrs:Vec<vars::Var> = Vec::new();

	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Unknown input");
	input = input.trim().to_string();

	for v in vars.iter() {
		if v.name == line {
			let vn = vars::Var {
				name: v.name.clone(),
				value: input.clone(),
				data_type: v.data_type.clone(),
			};
			
			vrs.push(vn.clone());
		} else {
			vrs.push(v.clone());
		}
	}

	vrs
}
