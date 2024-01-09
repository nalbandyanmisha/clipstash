// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

#[derive(Debug)]
struct Box {
    dimensions: Dimensions,
    color: Color,
    weight: f32,
}

#[derive(Debug)]
struct Dimensions {
    length: i32,
    width: i32,
    height: i32,
}

#[derive(Debug)]
enum Color {
    Red,
    Yellow,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("The color is Red"),
            Color::Yellow => println!("The color is Yellow"),
        }
    }
}

impl Dimensions {
    fn print(&self) {
        println!("Length: {:?}, Width: {:?}, Height: {:?}", self.length, self.width, self.height);
    }
}

impl Box {
    fn new(dimensions: Dimensions, color: Color, weight: f32) -> Self {
        Self {
            dimensions,
            color,
            weight,
        }
    }

    fn print(&self) {
        self.dimensions.print();
        self.color.print();
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {
    let box1 = Box::new(Dimensions { length: 3, width: 4, height: 5}, Color::Red, 4.0);
    let box2 = Box::new(Dimensions { length: 6, width: 7, height: 8}, Color::Yellow, 3.0);

    box1.print();
    box2.print();
}
