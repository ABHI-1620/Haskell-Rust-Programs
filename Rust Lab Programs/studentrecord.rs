use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    grade: String,
}

impl Student {

    fn new(name: &str, age: u32, grade: &str) -> Student {
        Student {
            name: name.to_string(),
            age,
            grade: grade.to_string(),
        }
    }

    fn display_student_info(&self) {
        println!("\nStudent Information:");
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Grade: {}", self.grade);
    }
    fn update_grade(&mut self, new_grade: &str) {
        self.grade = new_grade.to_string();
    }
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    loop {
        let mut name = String::new();
        let mut age_input = String::new();
        let mut grade = String::new();

        println!("Enter student's name (or type 'exit' to stop): ");
        io::stdin().read_line(&mut name).expect("Failed to read line");

        let name = name.trim();
        if name == "exit" {
            break;
        }

        println!("Enter student's age: ");
        io::stdin().read_line(&mut age_input).expect("Failed to read line");

        let age: u32 = age_input.trim().parse().expect("Please input a valid number for age");

        println!("Enter student's grade: ");
        io::stdin().read_line(&mut grade).expect("Failed to read line");

        let student = Student::new(&name, age, &grade.trim());
        students.push(student);
    }

    println!("\nAll Students:");
    for student in &students {
        student.display_student_info();
    }

    println!("\nEnter the name of the student whose grade you want to update: ");
    let mut update_name = String::new();
    io::stdin().read_line(&mut update_name).expect("Failed to read line");
    let update_name = update_name.trim();

    if let Some(student) = students.iter_mut().find(|s| s.name == update_name) {
        println!("Enter new grade for {}: ", update_name);
        let mut new_grade = String::new();
        io::stdin().read_line(&mut new_grade).expect("Failed to read line");

        student.update_grade(&new_grade.trim());
        println!("Updated record for {}:", update_name);
        student.display_student_info();
    } else {
        println!("Student with name '{}' not found.", update_name);
    }
}
