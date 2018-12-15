//ios.rs
//Handles the Input/output commands
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
