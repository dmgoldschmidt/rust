use std::env;
//use std::process::exit;

fn main(){
    let options : &mut Vec<String> = &mut Vec::new();
    let values : &mut Vec<String> = &mut Vec::new();
    get_opt(options,values);
    //assert!(options.len() == values.len());
    println!("options.len() = {}, values.len() = {}",options.len(), values.len()); 
    for i in  1..options.len(){
        println!("option[{i}] = {:?}, value[{i}] = {:?} ",options[i], values[i]);
    }
//    println!("options: {:?} values: {:?}",options,values);
}
fn get_opt(options : &mut Vec<String>, values : &mut Vec<String>) {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let mut last_was_opt:bool = false; // parser state
    for f0 in args {
        let  my_field = &f0;
        println!("processing {my_field}");
        let f: Vec<char> = my_field.chars().collect();
        if f[0] == '-' {
            if my_field.len() == 1 { // single minus is a value with no option
                options.push("".to_string()); // empty option
                values.push("-".to_string());
                last_was_opt=false;
                println!("field {my_field} is a value");
                continue;
            }
            if f[1].is_ascii_digit() || f[1] == '.' && f[1] == '\0' { // it's a negative number
                values.push(my_field.to_string());
                if !last_was_opt { options.push("".to_string()) }; // two values in a row
                last_was_opt = false;
                println!("field {my_field} is a value");
                continue;
            }
            println!("field {my_field} is an option");
 
            if last_was_opt {// two options in a row so there's an empty value
                values.push("".to_string());
                last_was_opt = false;
            }
            if f[1] == '-'{  // double minus means long option
                match my_field.find('='){
                    Some(i) => { // field is option=value
                        println!("found = at index {}",i);
                        options.push(f[2..i].iter().collect());
                        values.push(f[i+1..].iter().collect());
                        last_was_opt = false;
                    }
                    None => { // its a long option
                        println!("no = found");
                        options.push(f[2..].iter().collect());
                        values.push("".to_string());
                        last_was_opt = true;
                    }
                }
            }
            else {options.push(f[1..].iter().collect());
                  last_was_opt = true;
            }
        }
        else { // it's a value
            if !last_was_opt { // two values in a row
                options.push("".to_string()); // empty option 
            }
            values.push(my_field.to_string());
            last_was_opt = false;
            println!("field {my_field} is a value");
        }
    }
}
