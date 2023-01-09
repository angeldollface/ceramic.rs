/*
CERAMIC.RS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// This file contains my implementation of the Luhn algorithm.
// The library from GitHub (also mine) does work with non-WASM
// binaries but not with WASM binaries.

// Gets the index of an item in a vector 
// with items of type "String".
pub fn get_index(subject: &Vec<String>, element: &String) -> usize {
    let index = subject.iter().position(|r| r == element).unwrap();
    return index;
}

// Removes the last item of a vector with integers in
// them ("usize").
pub fn remove_last(subject: &mut Vec<usize>) -> Vec<usize> {
    let vec_last_index: usize = &subject.len() - 1;
    subject.remove(vec_last_index);
    return subject.to_vec();
}

// Returns a vector of strings from a character split for a string.
// Both the string and split character have to be strings.
pub fn clean_split(subject: &String, split_char: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in subject.split(split_char) {
        if &item == &"" {
            // Do nothing.
        }
        else {
            let new_item: String = item.to_string();
            result.push(new_item);
        }
    }
    return result;
}

// Checks whether the supplied character is
// an integer. Returns a boolean depending on
// whether this is the case.
pub fn is_int(subject: &String) -> bool{
    let mut result: bool = false;
    let match_op = subject.parse::<usize>();
    match match_op {
        Ok(_n) => {
            result = true
        },
        Err(_e) => {
            // Do nothing.
        }
    };
    return result;
}

// We check if "subject" is an integer and return
// it as such if it is. If not, 0 is returned.
pub fn parse_int(subject: &String) -> usize {
    let mut result: usize = 0;
    if is_int(&subject) {
        result = subject.parse::<usize>().unwrap();
    }
    else {
        // Do nothing.
    }
    return result;
}

// Checks whether the supplied IMEI string
// only contains integers.
// Returns a boolean depending on this. 
pub fn is_number_sequence(imei: &String) -> bool {
    let mut result = true;
    let char_list: Vec<String> = clean_split(imei, &String::from(""));
    for item in char_list {
        if is_int(&item) == false {
            result = false;
        }
        else {
            // Do nothing.
        }        
    }
    return result;
}

// Gets every second number starting from the left.
pub fn get_important_numbers(imei: &String) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let char_list: Vec<String> = clean_split(imei, &String::from(""));
    for (index, item) in char_list.iter().enumerate() {
        if index%2 != 0 {
            result.push(parse_int(item));
        }
        else {
            // Do nothing.
        }
    }
    return result;
}

// Gets all the numbers that remain.
// Removes the check digit because this is not allowed when
// adding all the remaining numbers together.
pub fn get_trash_numbers(imei: &String) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let char_list: Vec<String> = clean_split(imei, &String::from(""));
    for (index, item) in char_list.iter().enumerate() {
        if index%2 != 0 {
            // Do nothing.
        }
        else {
            result.push(parse_int(item));
        }
    }
    remove_last(&mut result);
    return result;
}

// Converts all the "important" numbers, doubles them, and returns them
// in an array.
pub fn double_important_numbers(imei: &String) -> Vec<usize> {
    let imp_nums: Vec<usize> = get_important_numbers(&imei);
    let mut result: Vec<usize> = Vec::new();
    for item in imp_nums {
        result.push(item * 2);
    }
    return result;
}

// Adds all the remaining numbers in a lump sum.
pub fn add_trash_numbers(imei: &String) -> usize{
    let imp_nums: Vec<usize> = get_trash_numbers(&imei);
    let mut result: usize = 0;
    for item in imp_nums {
        result = result + item;
    }
    return result;
}

// Because this is neccessary and we can't play fast and loose with types
// we need to convert between the arrays' types. 
pub fn convert_number_array_to_string_array(arr: &Vec<usize>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in arr {
        let num: String = item.to_string();
        let nums = clean_split(&(num.to_string()), &String::from(""));
        for x in nums {
            result.push(x);
        }
    }
    return result;
}

// Splits all the characters in the "important" numbers and adds them all
// together in a lump sum.
pub fn add_important_double_digits(imei: &String) -> usize {
    let mut result: usize = 0;
    let doubles = double_important_numbers(&imei);
    let digit_arr = convert_number_array_to_string_array(&doubles);
    for item in digit_arr {
        result = result + parse_int(&item);
    }
    return result;
}

// Gets the last item of a string array and returns it.
pub fn get_last_item(arr: &Vec<String>) -> String {
    let array_length: usize = arr.len();
    let last_item_index: usize = array_length -1;
    return arr[last_item_index].clone();
}

// Gets the check digit of your IMEI, adds the "important" and the
// "other" numbers together, subtracts the "mod 10" from 10 of that sum, makes
// a type conversion, compares the check digit and the calculated check digit,
// and returns true or false depending on whether they are equal or not.
pub fn validate_IMEI(imei: &String) -> bool {
    let mut result = false;
    let std_length: usize = 15;
    let imei_chars: Vec<String> = clean_split(&imei, &String::from(""));
    let check_digit = get_last_item(&imei_chars);
    let sum: usize = add_important_double_digits(&imei) + add_trash_numbers(&imei);
    let computed_check_digit: usize = 10 - (sum%10);
    let computed_converted_check_digit: String = computed_check_digit.to_string();
    if check_digit == computed_converted_check_digit && 
       imei_chars.len() == std_length && 
       is_number_sequence(&imei) 
    {
        result = true;
    }
    else {
        // Do nothing.
    }
    return result;
}
