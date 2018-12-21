use std::process;

use super::interpreter::RunData;
use super::string_utils;

/* Our var structure
Possible data types:
int->	#
dec->	.
str->	$
*/
#[derive(Clone)]
pub struct Array {
	pub name:String,
	pub value:Vec<String>,
	pub data_type:String,
}

//Creates a new array variable
pub fn create_array(line:&String, data:&mut RunData) {
    //Create an array variable
    let mut array = Array {
        name: "".to_string(),
        value: Vec::new(),
        data_type: "".to_string(),
    };

    let first = string_utils::get_first(&line);
    let second = string_utils::get_second(&string_utils::get_second(&line));

    //Get the data type
    let fc:char = first.chars().nth(0).unwrap();
    if fc == '#' {
        array.data_type = "int".to_string();
    } else if fc == '.' {
        array.data_type = "dec".to_string();
    } else if fc == '$' {
        array.data_type = "str".to_string();
    } else {
        println!("Error: Invalid datatype.");
        process::exit(1);
    }

    //Get the name
    let mut index = 1;
    let len = first.len();
    let mut name = String::new();

    while index < len {
        name.push(first.chars().nth(index).unwrap());
        index+=1;
    }

    array.name = name;

    //If we have a second part, load the default values
    if second.len() >= 1 {
        let mut current = String::new();

        for c in second.chars() {
            if c == ',' {
                array.value.push(current.clone());
                current = String::new();
            } else if c == ' ' {
                continue;
            } else {
                current.push(c);
            }
        }

        array.value.push(current.clone());
    }

    //Push the new object to our runtime data
    data.arrays.push(array);
}

//Push a value to an array
pub fn push(line:&String, data:&mut RunData) {
    let mut to_push = string_utils::get_first(&line);
    let part2 = string_utils::get_second(&line);
    let middle = string_utils::get_first(&part2);
    let arr_name = string_utils::get_second(&part2);

    //Check our middle keyword
    if middle != "to" {
        println!("Error: Invalid syntax for the PUSH command");
        process::exit(1);
    }

    //See if what we are pushing is a variable
    for v in data.vars.iter() {
        if v.name == to_push {
            to_push = v.value.clone();
        }
    }

    //Find the array and push it
    let mut arrays:Vec<Array> = Vec::new();
    
    for arr in data.arrays.iter() {
        if arr.name == arr_name {
            let mut arr2 = arr.clone();
            arr2.value.push(to_push.clone());
            arrays.push(arr2);
        } else {
            arrays.push(arr.clone());
        }
    }

    data.arrays = arrays;
}