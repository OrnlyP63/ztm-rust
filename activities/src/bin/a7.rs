// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Color {
    Red,
    Yellow,
    Blue,
}

fn print_color(my_color: Color){
    match my_color {
        Color::Red => println!("My Red"),
        Color::Blue => println!("My Blue"),
        Color::Yellow => println!("My Yellow"),
    }
}

fn main() {
    print_color(Color::Red);
}
