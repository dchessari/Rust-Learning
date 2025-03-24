use std::collections::HashMap;
use std::io;

fn add_employee(data: &mut HashMap<String, Vec<String>>, employees: &str, department: &str) {
    let new_name = String::from(employees);
    let dep = String::from(department.clone());

    match data.get_mut(&dep) {
        Some(existing_employees) => {
            existing_employees.push(new_name.clone());
            println!("Added Employee '{}' to Department: {}", new_name, dep);
        },
        None => {
            let mut new_employee_list = Vec::new();
            new_employee_list.push(new_name.clone());
            data.insert(dep, new_employee_list);
            println!("Added Employee: {} to NEW Department: {}", new_name, department);
        }
    }
}

fn list(data: &mut HashMap<String, Vec<String>>, employees: &str, department: &str) {
    let full_list = data.get(department);
    println!("{:?}", full_list);
}

fn main() {
    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please enter employee info in format [DEPARTMENT] [NAME]");
        println!("Type [list] for a full list of departments");
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
            add_employee(&mut data, employees, department);
        }

        else if words.len() == 1 && words[0] == "list" {

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
allow std io

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
