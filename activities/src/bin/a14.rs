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

enum Color {
    Red,
    Green,
    Blue,
}

struct Person {
    age: i32,
    name: String,
    favorite_color: Color,
}

fn print_name(p: &Person){
    println!("Name: {:?} and age: {:?}", p.name, p.age)  
}

fn print_color(c: &Color){
    match c{
        Color::Red => println!("Her/His favorite color is: red"),
        Color::Green => println!("Her/His favorite color is: green"),
        Color::Blue => println!("Her/His favorite color is: blue"),
    }
}

fn main() {
    let persons = vec![
        Person{
            age: 10,
            name: String::from("Jacob"),
            favorite_color: Color::Red
        }, 
        Person{
            age: 24,
            name: String::from("Marry"),
            favorite_color: Color::Green
        },
        Person{
            age: 58,
            name: "Mark".to_owned(),
            favorite_color: Color::Blue
        }
    ];

    for p in &persons{
        if p.age > 10 {
            print_name(&p);
            print_color(&p.favorite_color);
        } else {
            println!("Invalid age!")
        }
        
    }
}
