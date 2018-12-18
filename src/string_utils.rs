pub fn get_first(line:&String) -> String {
	let mut ret = String::new();
	
	for c in line.chars() {
		if c == ' ' {
			break;
		} else {
			ret.push(c);
		}
	}
	
	ret
}

pub fn get_second(line:&String) -> String {
	let mut ret = String::new();
	let mut found = false;
	
	for c in line.chars() {
		if c == ' ' {
			if found {
				ret.push(c);
			} else {
				found = true;
			}
		} else {
			if found {
				ret.push(c);
			}
		}
	}
	
	ret
}

//Returns a label name (no arguments)
pub fn lbl_name(name:&String) -> String {
	let mut ret = String::new();
	let mut in_args = false;
	
	for c in name.chars() {
		if c == ':' {
			break;
		} else if c == '[' {
			in_args = true;
		} else if c == ']' {
			in_args = false;
		} else {
			if !in_args {
				ret.push(c);
			}
		}
	}
	
	ret
}

pub fn rm_quotes(line:&String) -> String {
	let mut ret = String::new();
	
	for c in line.chars() {
		if c != '"' {
			ret.push(c);
		}
	}
		
	ret
}
