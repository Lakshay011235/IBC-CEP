struct Student {
    name: String,
    email: String,
    phone: String,
    id: u32,
}

fn main() {
    let students = vec![
        Student {
            name: String::from("John Doe"),
            email: String::from("john@example.com"),
            phone: String::from("123-456-7890"),
            id: 1,
        },
        Student {
            name: String::from("Jane Smith"),
            email: String::from("jane@example.com"),
            phone: String::from("987-654-3210"),
            id: 2,
        },
        // Add more students here
    ];

    for index in 0..=2 {
        match students.get(index) {
            Some(student) => {
                println!("Student Name: {}", student.name);
                println!("Student Email: {}", student.email);
                println!("Student Phone: {}", student.phone);
                println!("Student ID: {}", student.id);
            }
            None => {
                println!("Invalid index. Student not found.");
            }
        }
    }
}