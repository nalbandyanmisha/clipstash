// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: u32,
}

impl Adult {
    fn new(name: String, age: u32) -> Result<Self, String> {
        if age > 21 {
           return  Ok(Self {name, age});
        }
        Err("User is not adult".to_owned())
    }
}
fn main() {
    let mut human = Adult::new("Misha".to_owned(), 26);
    match human {
        Ok(data) => println!("{:?}", data),
        Err(e) => println!("Error: {:?}", e),
    }

    human = Adult::new("Lucy".to_owned(), 20);
    match human {
        Ok(data) => println!("{:?}", data),
        Err(e) => println!("Error: {:?}", e),
    }
}