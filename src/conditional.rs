use std::process;

use super::string_utils;
use super::utils;
use super::interpreter;
use super::interpreter::RunData;

//Checks a conditional statement
pub fn check_conditional(second:String, mut data:RunData) -> RunData {
	//Break down the string
	let mut condition = String::new();
	let mut rm_second = String::new();
	let mut last_bracket = false;
		
	for c in second.chars() {
		if c == '[' {
			continue;
		} else if c == ']' {
			last_bracket = true;
		} else if c == ' ' && !last_bracket {
			continue;
		} else {
			if last_bracket {
				rm_second.push(c);
			} else {
				condition.push(c);
			}
		}
	}
		
	condition = condition.trim().to_string();
	rm_second = rm_second.trim().to_string();
	let result = string_utils::get_second(&rm_second);
	
	//Parse the condition
	//1) Break down the string
	let condition = utils::get_condition(&condition);
	
	//2) Check to see if each part is a variable, and if
	//	so, get the value
	let mut var1 = condition.part1;
	let mut var2 = condition.part2;
	
	for v in data.vars.clone() {
		if v.name == var1 {
			var1 = v.value.clone();
		}
		
		if v.name == var2 {
			var2 = v.value.clone();
		}
	}
		
	//3) Get the datatype and compare
	//Integer datatypes
	if utils::is_int(var1.clone(), var2.clone()) {
		let no1 = var1.parse::<i32>().unwrap();
		let no2 = var2.parse::<i32>().unwrap();
		
		if compare_ints(no1,no2,&condition.operator) {
			data = interpreter::run(result,data.clone());
			data.if_solved = true;
		}
		
	//Floating-point datatypes
	} else if utils::is_double(var1.clone(), var2.clone()) {
		let no1 = var1.parse::<f32>().unwrap();
		let no2 = var2.parse::<f32>().unwrap();
		
		if compare_doubles(no1,no2,&condition.operator) {
			data = interpreter::run(result,data.clone());
			data.if_solved = true;
		}
		
	//String datatypes
	} else {
		if compare_strings(&var1, &var2, &condition.operator) {
			data = interpreter::run(result,data.clone());
			data.if_solved = true;
		}
	}

	data.clone()
}

//Compares two integer variables
pub fn compare_ints(no1:i32, no2:i32, operator:&String) -> bool {
	if operator == "==" {
		if no1 == no2 {
			return true;
		}
	} else if operator == "!=" {
		if no1 != no2 {
			return true;
		}
	} else if operator == ">" {
		if no1 > no2 {
			return true;
		}
	} else if operator == "<" {
		if no1 < no2 {
			return true;
		}
	}
	
	false
}

//Compare two double variables
pub fn compare_doubles(no1:f32, no2:f32, operator:&String) -> bool {
	if operator == "==" {
		if no1 == no2 {
			return true;
		}
	} else if operator == "!=" {
		if no1 != no2 {
			return true;
		}
	} else if operator == ">" {
		if no1 > no2 {
			return true;
		}
	} else if operator == "<" {
		if no1 < no2 {
			return true;
		}
	}
	
	false
}

//Compares two strings
pub fn compare_strings(str1:&String, str2:&String, op:&String) -> bool {
	if op != "==" && op != "!=" {
		println!("Error: Only the \'==\' and the \'!=\' are valid for use with strings.");
		process::exit(1);
	}
	
	let s1 = string_utils::rm_quotes(&str1);
	let s2 = string_utils::rm_quotes(&str2);
	
	if s1 == s2 {
		return true;
	}
	
	false
}
