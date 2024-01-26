use std::env;
use std::process::exit;

fn main(){
    let options : &mut Vec<String> = &mut Vec::new();
    let values : &mut Vec<String> = &mut Vec::new();
    get_opt(options,values);
    assert!(options.len() == values.len());
    println!("options.len() = {}",options.len()); 
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
        let f: Vec<char> = my_field.chars().collect();
        if f[0] == '-' && (!f[1].is_ascii_digit()) && f[1] != '.' && f[1] != '\0'  {
            if last_was_opt {// two options in a row so there's an empty value
                values.push("".to_string());
            }
            if my_field.len() == 1 { // single minus is a value with no option
                 options.push("".to_string()); // empty option
                 values.push("-".to_string());
                 break;
            }
            println!("field {my_field} is an option");
            if f[1] == '-'{  // double minus means long option
                match my_field.find('='){
                    Some(i) => {
                        println!("found = at index {}",i);
                        options.push(f[2..i].iter().collect());
                        values.push(f[i+1..].iter().collect());
                        last_was_opt = false;
                    }
                    None => {
                        println!("no = found");
                        options.push(f[2..].iter().collect());
                        last_was_opt = true;
                    }
                }
/*
                for i in 2..f.len() {
                    if f[i] == b'='  { // embedded '=' in long option
                        options.push(String::from_utf8(f[2..i]));
                        values.push(String{f:f[i+1..]});
                        last_was_opt = false;
                        break;
                    }
                }
                options.push(String(f[2..]));
                 */
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
        }
    }
}
