use std::process;

use super::interpreter::RunData;
use super::string_utils;

//The code for the LEN command
pub fn len(second:String, data:&mut RunData) -> bool {
	let mut found = false;
	let fc:char = second.chars().nth(0).unwrap();
	let lc:char = second.chars().last().unwrap();
	let mut string = String::new();
	
	if (fc == '\"') && (fc == lc) {
		string = string_utils::rm_quotes(&second);
		found = true;
	} else {
		for v in data.vars.iter() {
			if v.name == second {
				if v.data_type == "str" {
					string = v.value.clone();
					found = true;
				} else {
					println!("Error: LEN is only valid with strings or string variables.");
					process::exit(1);
				}
			}
		}
	}
	
	string = string_utils::rm_quotes(&string);
	
	let len = string.len();
	data.memory = len.to_string();

	found
}

//The code for the CHAR command
pub fn char_cmd(second:String, data:&mut RunData) {
	let mut part1 = string_utils::get_first(&second);
	let mut part2 = string_utils::get_second(&string_utils::get_second(&second));
	
	//First, see if we reference any vars
	for v in data.vars.iter() {
		if v.name == part1 {
			part1 = v.value.clone();
		}
		
		if v.name == part2 {
			part2 = v.value.clone();
			part2 = string_utils::rm_quotes(&part2);
		}
	}

	//Convert the first part to an index
	let mut index = 0;
	match part1.parse::<usize>() {
		Ok(n) => index = n,
		Err(_n) => println!("Error: You must specify an integer with the CHAR command."),
	}
	
	//Get the character at the index and set it to memory
	let c:char = part2.chars().nth(index).unwrap();
	data.memory = c.to_string();
}
