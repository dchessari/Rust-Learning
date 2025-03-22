use std::collections::HashMap;
use std::io;

fn main() {
    let mut data = HashMap::new();

    println!("Please enter employee info in format [DEPARTMENT] [NAME]")

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let employee = input.trim();

        input.push(employee);
}


/*Using a hash map and vectors, create a text interface to allow a user to add employee
names to a department in a company; for example, “Add Sally to Engineering” or
“Add Amir to Sales.” Then let the user retrieve a list of all people in a
department or all people in the company by department, sorted alphabetically.*/