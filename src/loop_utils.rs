use std::process;
use super::interpreter;
use super::interpreter::RunData;
use super::utils;
use super::vars::Var;

//The code for a while loop
pub fn run_while_loop(second:String, mut data:RunData) -> RunData {
	data.in_loop = false;
		
	let condition = utils::get_condition(&second);
	let var = condition.part1;
	let op = condition.operator;
	let index = condition.part2;	
	
	let mut sub_data = interpreter::build_data();
	sub_data.labels = data.labels.clone();
	sub_data.vars = data.vars.clone();
		
	let mut last_vars:Vec<Var> = sub_data.vars.clone();

	loop {
		let mut vr = 0;
		let mut iv;
		
		match index.parse::<i32>() {
			Ok(_n) => iv = index.parse().unwrap(),
			Err(_n) => iv = 0,
		}
		
		for v in sub_data.vars.iter() {
			if v.name == var {
				if v.data_type == "int" {
					vr = v.value.clone().parse().unwrap();
				} else {
					println!("Fatal Error:");
					println!("Only integers are supported in do-while loops right now.");
					process::exit(1);
				}
			}
			
			if v.name == index {
				if v.data_type == "int" {
					iv = v.value.clone().parse().unwrap();
				} else {
					println!("Fatal Error:");
					println!("Only integers are supported in do-while loops right now.");
					process::exit(1);
				}
			}
		}
		
		if op == "<" {
			if vr < iv {
				for ln in data.loop_bd.iter() {
					sub_data = interpreter::run(ln.clone(), sub_data.clone());
					if sub_data.break_req {
						sub_data.break_req = false;
						return sub_data;
					}
				}
			} else {
				break;
			}
		} else if op == ">" {
			if vr > iv {
				for ln in data.loop_bd.iter() {
					sub_data = interpreter::run(ln.clone(), sub_data.clone());
					if sub_data.break_req {
						sub_data.break_req = false;
						return sub_data;
					}
				}
			} else {
				break;
			}
		} else if op == "==" {
			if vr == iv {
				for ln in data.loop_bd.iter() {
					sub_data = interpreter::run(ln.clone(), sub_data.clone());
					if sub_data.break_req {
						sub_data.break_req = false;
						return sub_data;
					}
				}
			} else {
				break;
			}
		} else if op == "!=" {
			if vr != iv {
				for ln in data.loop_bd.iter() {
					sub_data = interpreter::run(ln.clone(), sub_data.clone());
					if sub_data.break_req {
						sub_data.break_req = false;
						return sub_data;
					}
				}
			} else {
				break;
			}
		} else {
			println!("Fatal Error:");
			println!("Unknown operator: {}",op);
			process::exit(1);
		}

		//Remove any vars within the scope of the do-while
		let mut new_vars:Vec<Var> = Vec::new();

		for v1 in sub_data.vars.iter() {
			for v2 in last_vars.iter() {
				if v1.name == v2.name {
					new_vars.push(v1.clone());
				}
			}
		}

		sub_data.vars = new_vars;
		last_vars = sub_data.vars.clone();
	}
		
	sub_data
}

//The code for the LOOP command
pub fn run_loop(mut data:RunData) -> RunData {
	data.in_loop = false;

	let mut sub_data = interpreter::build_data();
	sub_data.labels = data.labels.clone();
	sub_data.vars = data.vars.clone();

	loop {
		for ln in data.loop_bd.iter() {
			sub_data = interpreter::run(ln.clone(), sub_data.clone());
			if sub_data.break_req {
				break;
			}
		}

		if sub_data.break_req {
			break;
		}
	}

	data.clone()
}
