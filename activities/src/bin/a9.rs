// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

struct CartesianCoordinate {
    x: i32,
    y: i32,
}

fn get_c_coordinate(point: CartesianCoordinate) -> (i32, i32) {
    (point.x, point.y)
}

fn main() {
    let point = CartesianCoordinate {
        x: 4,
        y: 5,
    };
    let (_, y) = get_c_coordinate(point);

    if y > 5 {
        println!("y is gt 5");
    } else if y < 5 {
        println!("y is lt 5");
    } else {
        println!("y is eq 5");
    }
}
