fn update_salary(employee: (i32, String, f64)) -> (i32, String, f64) {
    let (id, name, salary) = employee;

    if salary < 50000.0 {
        (id, name, salary * 1.10)
    } else {
        employee
    }
}

fn main() {
    let employee = (1, "John Doe".to_string(), 40000.0);
    let updated_employee = update_salary(employee);
    println!("Updated Employee Data: {:?}", updated_employee);
}