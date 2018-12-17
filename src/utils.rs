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
