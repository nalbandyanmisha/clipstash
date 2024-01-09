// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student = Student {
        name: "Misha".to_owned(),
        locker: Some(45),
    };
    let student1 = Student {
        name: "Lucy".to_owned(),
        locker: None,
    };
    
    println!("name: {:?}", student.name);
    match student.locker {
        Some(number) => println!("{:?}", number),
        None => println!("number was not provided"),
    }

    println!("name: {:?}", student1.name);
    match student1.locker {
        Some(number) => println!("{:?}", number),
        None => println!("number was not provided"),
    }
}
