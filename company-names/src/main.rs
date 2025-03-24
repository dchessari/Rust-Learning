use std::collections::HashMap;
use std::io;

fn add_employee(data: &mut HashMap<String, Vec<String>>, employees: &str, department: &str) {
    let mut new_name = String::from(employees);
    let mut department = String::from(department);

    match data.get_mut(&department) {
        Some(employees) => {
            employees.push(String::from(new_name));
            println!("Added Employee '{}' to Department: {}", employees, department);
        },
        None => {
            let mut new_name = Vec::new();
            employees.push(&new_name);
            data.insert(department.to_string(), new_name);
            println!("Added Employee '{}' to NEW Department: {}", new_name, department);
        }
    }
}

fn main() {
    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please enter employee info in format [DEPARTMENT] [NAME]");
        println!("Type [quit] to exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "quit" {
            break;
        }

        let words: Vec<&str> = input.split_whitespace().collect();
        if words.len() == 2 {
            let department = words[0];
            let employees = words[1];
        }

        else if words.len() == 1 && words[0] == "list" {
            // CALL FUNCTION TO PRINT DEPT LIST
        }

        else if words.len() == 2 && words[1] == "department" {
            // CALL FUNCTION TO PRINT DEPARTMENT SPECIFIC LIST
        }

        else {
            println!("Invalid input: {}", input);
        }
    }
}


/*Using a hash map and vectors, create a text interface to allow a user to add employee
names to a department in a company; for example, “Add Sally to Engineering” or
“Add Amir to Sales.” Then let the user retrieve a list of all people in a
department or all people in the company by department, sorted alphabetically.*/

/*allow hashmaps
allow std input

*function to handle NAME*
take vector containing name list
push names to department hashmap

*function to handle department*
if existing department take the employee name
if new department make new hashmap for it and add employee name

*main function*
handle user input
split whitespace
parse and sort user input

sort alphabetically
handle user commands (report full list)*/
