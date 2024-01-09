// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    color: String,
}

fn print_name(name: &str) {
   println!("name of Person is {:?}", name);
}

fn print_color(color: &str) {
    println!("favorite color of person is {:?}", color);
}

fn main() {
    let persons = vec!{
        Person {
            age: 18,
            name: "Lucy".to_owned(),
            color: "Purple".to_owned(),
        },
        Person {
            age: 21,
            name: String::from("Michael"),
            color: String::from("Green"),
        },
        Person {
            age: 9,
            name: String::from("Misha"),
            color: String::from("Brown"),
        },
    };

    for person in &persons {
       if person.age <= 10 {
           print_name(&person.name);
           print_color(&person.color);
       } 
    }
}
