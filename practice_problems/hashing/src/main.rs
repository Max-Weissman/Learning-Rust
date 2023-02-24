use std::io;
use std::collections::HashMap;

fn main() {

    let mut company = HashMap::new();
    
    println!("The company!");

    home(&mut company);
}

fn home(company: &mut HashMap<String,Vec<String>>) {
    loop {
        println!("Would you like to add or check?");

        let mut directory = String::new();

        io::stdin()
            .read_line(&mut directory)
            .expect("Failed to read line");

        match directory.trim() {
            "add" => add(company),
            "check" => check(&company),
            _ => println!("You must type 'add' or 'check'.")
        }
    }
}

fn add(company: &mut HashMap<String,Vec<String>>) {
    let mut employee = String::new();
    let mut department = String::new();

    println!("Employee's name?");

    io::stdin()
        .read_line(&mut employee)
        .expect("Failed to read line");

    println!("Employee's department?");

    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");

    let employee = employee.trim().to_string();
    let department = department.trim().to_string();
        
    let key = company.entry(department).or_insert(vec![]);
    key.push(employee)
}

fn check(company: &HashMap<String,Vec<String>>) {
    println!("Which department would you like to check?");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice = choice.trim();


    if let Some(value) = company.get(choice) {
        println!("{:?}", value);
    } else if choice == "all" {
        println!("{:?}", company)
    } else {
        println!("No one is in that department")
    }
}
