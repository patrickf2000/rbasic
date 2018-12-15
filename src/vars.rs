use super::string_utils;

/* Our var structure
Possible data types:
int->	#
dec->	.
str->	$
*/
#[derive(Clone)]
pub struct Var {
	pub name:String,
	pub value:String,
	pub data_type:String,
}

//Clones a variable
pub fn clone_var(var:&Var) -> Var {
	let v = Var {
		name: var.name.clone(),
		value: var.value.clone(),
		data_type: var.data_type.clone(),
	};
	
	v
}

//Returns a variable name from a string
pub fn var_name(line:&String) -> String {
	let mut ret = String::new();
	
	let mut index:usize = 1;
	let len:usize = line.len();
	while index < len {
		ret.push(line.chars().nth(index).unwrap());
		index+=1;
	}
	
	ret
}

//Expands a variable declaration
pub fn expand_def(value:String, data_type:String, vars:Vec<Var>) -> String {
	let mut ret = value.clone();
	
	//First, see if we are referencing another variable
	for v in vars.iter() {
		if value == v.name {
			ret = v.value.clone();
			return ret;
		}
	}
	
	//Second, see if there are any math operators
	let mut found = false;
	let mut first = String::new();
	let mut second = String::new();
	let mut op:char = ' ';
	
	//Loop through our line
	for c in value.chars() {
		if c == '+' || c == '-' || c == '*' || c == '/' || c == '%' {
			found = true;
			op = c;
		} else if c == ' ' {
			continue;
		} else {
			if found {
				second.push(c);
			} else {
				first.push(c);
			}
		}
	}
	
	//If no operator was found, just return
	if !found {
		return ret;
	}
	
	//If an operator was found, use the rest of the function to expand it
	
	//First, see if we either parts reference another variable
	for v in vars.iter() {
		if v.name == first {
			first = v.value.clone();
		}
		
		if v.name == second {
			second = v.value.clone();
		}
	}
	
	//We can only add strings, so make sure no other operator is being used with strings
	if data_type == "str" && op != '+' {
		println!("Error: You can only add strings.");
		return ret;
	}
	
	//Now, determine our operation
	//Add
	if op == '+' {
		if data_type == "int" {
			let no1:i32 = first.parse().unwrap();
			let no2:i32 = second.parse().unwrap();
			let answer = no1+no2;
			ret = answer.to_string();
		} else if data_type == "dec" {
			let no1:f32 = first.parse().unwrap();
			let no2:f32 = second.parse().unwrap();
			let answer = no1+no2;
			ret = answer.to_string();
		} else if data_type == "str" {
			let str1 = string_utils::rm_quotes(&first);
			let str2 = string_utils::rm_quotes(&second);
			ret = str1 + &str2;
		}
		
	//Subtract
	} else if op == '-' {
		if data_type == "int" {
			let no1:i32 = first.parse().unwrap();
			let no2:i32 = second.parse().unwrap();
			let answer = no1-no2;
			ret = answer.to_string();
		} else if data_type == "dec" {
			let no1:f32 = first.parse().unwrap();
			let no2:f32 = second.parse().unwrap();
			let answer = no1-no2;
			ret = answer.to_string();
		}
		
	//Multiply
	} else if op == '*' {
		if data_type == "int" {
		
		} else if data_type == "dec" {
		
		}
		
	//Divide
	} else if op == '/' {
		if data_type == "int" {
		
		} else if data_type == "dec" {
		
		}
		
	//Modulus
	} else if op == '%' {
		if data_type == "int" {
		
		} else if data_type == "dec" {
		
		}
	}
	
	ret
}

//Defines a variable from a string
pub fn def_var(name:String, value:String, vars:Vec<Var>) -> Vec<Var> {
	let mut vrs:Vec<Var> = Vec::new();
	
	for v in vars.iter() {
		if v.name == name {
			let mut vn = Var {
				name: name.clone(),
				value: "".to_string(),
				data_type: v.data_type.clone(),
			};
			
			vn.value = expand_def(value.clone(), vn.data_type.clone(), vars.clone());
			
			vrs.push(vn);
		} else {
			vrs.push(clone_var(v));
		}
	}
	
	vrs
}

//Creates a var from a line of text
pub fn create_var(line:String, vars:Vec<Var>) -> Var {
	let mut v = Var {
		name: "".to_string(),
		value: "".to_string(),
		data_type: "int".to_string(),
	};
	
	let fc = line.chars().nth(0).unwrap();
	if fc == '#' {
		v.data_type = "int".to_string();
	} else if fc == '.' {
		v.data_type = "dec".to_string();
	} else if fc == '$' {
		v.data_type = "str".to_string();
	} else {
		println!("Error: No datatype or unknown datatype specified");
	}
	
	let name1 = string_utils::get_first(&line);
	let name = var_name(&name1);
	v.name = name.to_string();
	
	let s1 = string_utils::get_second(&line);
	let second = string_utils::get_second(&s1);
	
	v.value = expand_def(second, v.data_type.clone() ,vars.clone());
	
	v
}
