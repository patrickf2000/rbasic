use std::process;
use super::vars;
use super::vars::Var;
use super::interpreter::Lbl;

//Returns a list of arguments from a function name
pub fn args_list(name:&String) -> Vec<String> {
	let mut args_list = String::new();
	let mut in_args = false;
	
	for c in name.chars() {
		if c == '[' {
			in_args = true;
		} else if c == ']' {
			in_args = false;
		} else {
			if in_args {
				args_list.push(c);
			}
		}
	}
	
	if args_list.len() == 0 {
		return Vec::new();
	}
	
	let mut args_obj:Vec<String> = Vec::new();
	let mut current = String::new();
	
	for c in args_list.chars() {
		if c == ',' {
			args_obj.push(current.clone());
			current = "".to_string();
		} else if c == ' ' {
			continue;
		} else {
			current.push(c);
		}
	}
	args_obj.push(current);
	
	args_obj
}

//Returns a vector of vars as args for a label
pub fn build_args(name:&String) -> Vec<Var> {
	let mut args:Vec<Var> = Vec::new();
	let args_obj = args_list(&name);
	
	if args_obj.len() == 0 {
		return args;
	}
	
	for obj in args_obj {
		let mut var = Var {
			name: vars::var_name(&obj),
			value: "".to_string(),
			data_type: "".to_string(),
		};
		
		let dt:char = obj.chars().nth(0).unwrap();
		
		if dt == '#' {
			var.data_type = "int".to_string();
		} else if dt == '.' {
			var.data_type = "dec".to_string();
		} else if dt == '$' {
			var.data_type = "str".to_string();
		}
		
		args.push(var);
	}
	
	args
}

//Assigns argument values for a label
pub fn assign_args(label:Lbl, name:&String, parent_vars:&Vec<Var>) -> Vec<Var> {
	let mut vars:Vec<Var> = label.args.clone();
	let args_obj = args_list(&name);
	
	let v_len = vars.len();
	let obj_len = args_obj.len();
	
	if v_len != obj_len {
		println!("Error: Insufficient or excessive number of label arguments.");
		process::exit(1);
	}
	
	let mut index = 0;
	while index < v_len {
		let mut value = args_obj.iter().nth(index).unwrap().to_string();
		for vr in parent_vars.iter() {
			if vr.name == value {
				value = vr.value.clone();
			}
		}
	
		let v = vars.iter().nth(index).unwrap();
		vars = vars::def_var(v.name.clone(), value, vars.clone());
		
		index += 1;
	}
	
	vars
}
