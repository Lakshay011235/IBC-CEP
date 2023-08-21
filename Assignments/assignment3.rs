use std::io;

// Define the Employee struct
#[derive(Debug)]
struct Employee {
    employee_id: i32,
    employee_name: String,
    email: String,
    age: i32,
    phone_number: String,
}

impl Employee {
    // Function to create a new Employee instance
    fn new(
        employee_id: i32,
        employee_name: String,
        email: String,
        age: i32,
        phone_number: String,
    ) -> Employee {
        Employee {
            employee_id,
            employee_name,
            email,
            age,
            phone_number,
        }
    }

    // Function to get employee details by ID
    fn get_employee_by_id(employees: &Vec<Employee>, employee_id: i32) -> Option<&Employee> {
        employees.iter().find(|employee| employee.employee_id == employee_id)
    }

    // Function to get employees by age
    fn get_employees_by_age(employees: &Vec<Employee>, age: i32) -> Vec<&Employee> {
        employees.iter().filter(|employee| employee.age == age).collect()
    }
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    loop {
        println!("Enter employee details:");

        let mut employee_id = String::new();
        let mut employee_name = String::new();
        let mut email = String::new();
        let mut age = String::new();
        let mut phone_number = String::new();

        println!("Employee ID:");
        io::stdin().read_line(&mut employee_id).expect("Failed to read line");
        let employee_id: i32 = employee_id.trim().parse().expect("Invalid employee ID");

        println!("Employee Name:");
        io::stdin().read_line(&mut employee_name).expect("Failed to read line");

        println!("Email:");
        io::stdin().read_line(&mut email).expect("Failed to read line");

        println!("Age:");
        io::stdin().read_line(&mut age).expect("Failed to read line");
        let age: i32 = age.trim().parse().expect("Invalid age");

        println!("Phone Number:");
        io::stdin().read_line(&mut phone_number).expect("Failed to read line");

        let employee = Employee::new(
            employee_id,
            employee_name.trim().to_string(),
            email.trim().to_string(),
            age,
            phone_number.trim().to_string(),
        );

        employees.push(employee);

        println!("Do you want to add another employee? (yes/no)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        if choice.trim().to_lowercase() != "yes" {
            break;
        }
    }

    // Fetch employee details by ID
    println!("Enter employee ID to fetch details:");
    let mut fetch_id = String::new();
    io::stdin().read_line(&mut fetch_id).expect("Failed to read line");
    let fetch_id: i32 = fetch_id.trim().parse().expect("Invalid employee ID");
    if let Some(employee) = Employee::get_employee_by_id(&employees, fetch_id) {
        println!("Employee with ID {}:\n{:?}", fetch_id, employee);
    } else {
        println!("Employee with ID {} not found.", fetch_id);
    }

    // Fetch employees by age
    println!("Enter age to fetch employees:");
    let mut fetch_age = String::new();
    io::stdin().read_line(&mut fetch_age).expect("Failed to read line");
    let fetch_age: i32 = fetch_age.trim().parse().expect("Invalid age");
    let same_age_employees = Employee::get_employees_by_age(&employees, fetch_age);
    println!("Employees with age {}:\n{:?}", fetch_age, same_age_employees);
}
