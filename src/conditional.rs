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
	if utils::is_int(var1.clone(), var2.clone()) {
		let no1 = var1.parse::<i32>().unwrap();
		let no2 = var2.parse::<i32>().unwrap();
		
		if utils::compare_ints(no1,no2,&condition.operator) {
			data = interpreter::run(result,data.clone());
		}
	} else if utils::is_double(var1.clone(), var2.clone()) {
	
	} else {
	
	}

	data.clone()
}
