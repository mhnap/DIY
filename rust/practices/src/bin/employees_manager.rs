use employees_manager::EmployeesManager;

fn main() {
    let mut manager = EmployeesManager::new();
    manager.run();
}

mod employees_manager {
    use std::io;
    use std::io::Write;
    use std::collections::{HashMap, HashSet};

    // Each `Employee` and `Department` are unique.
    // So, one `Employee` can be added only to one `Department`.

    type Employee = String;
    type Department = String;
    type Employees = HashSet<Employee>;
    type Departments = HashMap<Department, Employees>;

    pub struct EmployeesManager {
        departments: Departments,
    }

    impl EmployeesManager {
        pub fn new() -> Self {
            EmployeesManager { departments: Departments::new() }
        }

        pub fn run(&mut self) {
            EmployeesManager::print_intro();
            loop {
                print!("Please input your command: ");
                io::stdout().flush().expect("Unable to flush stdout");

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line.");

                let command = input.trim();
                if command.is_empty() {
                    continue;
                }

                if !self.process_command(command) {
                    break;
                }
            }
        }

        fn print_intro() {
            println!("This is a basic employees management tool.");
            println!("You can add or remove employees and their departments in a company.");
            println!("Use this list of commands:");
            println!(" - \"Add `employee_name` to `department_name`\" to add employee to department.");
            println!(" - \"Remove `department_name` department\" to remove department.");
            println!(" - \"Remove `employee_name` employee\" to remove employee from department.");
            println!(" - \"List `department_name` department\" to list department with its employees.");
            println!(" - \"List `employee_name` employee\" to list employee with its department.");
            println!(" - \"List departments\" to list all departments.");
            println!(" - \"List employees\" to list all employees.");
            println!(" - \"List departments with employees\" to list all departments with employees.");
            println!(" - \"List employees with departments\" to list all employees with departments.");
            println!(" - \"Quit\" to quit.");
            println!();
        }

        fn process_command(&mut self, command: &str) -> bool {
            let command_parts: Vec<&str> = command.split_whitespace().collect();
            match command_parts[..] {
                ["Add", employee, "to", department] => {
                    match self.add_employee_to_department(employee, department) {
                        None => println!("{employee} was successfully added to {department} department."),
                        Some(old_department) => {
                            if old_department == department {
                                println!("{employee} is already added to {department} department!");
                            } else {
                                println!("{employee} is already added to another {old_department} department! Consider removing {employee} from {old_department} first!");
                            }
                        }
                    }
                }
                ["Remove", department, "department"] => {
                    match self.remove_department(department) {
                        true => println!("{department} department was successfully removed."),
                        false => println!("Cannot find {department} department!")
                    }
                }
                ["Remove", employee, "employee"] => {
                    match self.remove_employee(employee) {
                        true => println!("{employee} was successfully removed."),
                        false => println!("Cannot find {employee}!")
                    }
                }
                ["List", department, "department"] => {
                    match self.list_department(department) {
                        Some(employees) => println!("List of employees in {department} department is {:?}.", employees),
                        None => println!("Cannot find {department} department!")
                    }
                }
                ["List", employee, "employee"] => {
                    match self.list_employee(employee) {
                        Some(department) => println!("{employee}'s department is {department}."),
                        None => println!("Cannot find {employee}!")
                    }
                }
                ["List", "departments"] => {
                    let departments = self.list_departments();
                    println!("List of departments is {:?}.", departments);
                }
                ["List", "employees"] => {
                    let employees = self.list_employees();
                    println!("List of all employees is {:?}.", employees);
                }
                ["List", "departments", "with", "employees"] => {
                    let departments = self.list_departments_with_employees();
                    println!("List of departments with employees is {:?}.", departments);
                }
                ["List", "employees", "with", "departments"] => {
                    let employees = self.list_employees_with_departments();
                    println!("List of employees with departments is {:?}.", employees);
                }
                ["Quit"] => {
                    return false;
                }
                _ => println!("Unknown command..."),
            }
            true
        }

        // TODO: Investigate why Option<&Department> don't work.
        fn add_employee_to_department(&mut self, employee: &str, department: &str) -> Option<Department> {
            // Check whether this employee is added to some department.
            for (old_department, employees) in &self.departments {
                if employees.get(employee).is_some() {
                    return Some(old_department.clone());
                }
            }
            // Employee is not added to any department, so let's add it.
            let employees = self.departments.entry(department.to_owned()).or_default();
            employees.insert(employee.to_owned());
            None
        }

        fn remove_department(&mut self, department: &str) -> bool {
            self.departments.remove(department).is_some()
        }

        fn remove_employee(&mut self, employee: &str) -> bool {
            for employees in self.departments.values_mut() {
                if employees.get(employee).is_some() {
                    employees.remove(employee);
                    return true;
                }
            }
            false
        }

        fn list_department(&self, department: &str) -> Option<&Employees> {
            self.departments.get(department)
        }

        fn list_employee(&self, employee: &str) -> Option<&Department> {
            for (department, employees) in &self.departments {
                if employees.get(employee).is_some() {
                    return Some(department);
                }
            }
            None
        }

        fn list_departments(&self) -> Vec<&Department> {
            let mut departments = Vec::new();
            for department in self.departments.keys() {
                departments.push(department);
            }
            departments
        }

        fn list_employees(&self) -> Vec<&Employee> {
            let mut all_employees = Vec::new();
            for employees in self.departments.values() {
                for employee in employees {
                    all_employees.push(employee);
                }
            }
            all_employees
        }

        fn list_departments_with_employees(&self) -> &Departments {
            &self.departments
        }

        fn list_employees_with_departments(&self) -> HashMap<&Employee, &Department> {
            let mut employees_with_departments = HashMap::new();
            for (department, employees) in &self.departments {
                for employee in employees {
                    employees_with_departments.insert(employee, department);
                }
            }
            employees_with_departments
        }
    }
}
