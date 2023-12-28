// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}



fn main() {
    let tickets = vec![
        Tickets::Backstage(50.0, String::from("Billy")),
        Tickets::Standard(15.0),
        Tickets::Vip(30.0, String::from("Amy")),
    ];

    for t in &tickets {
        match t {
            Tickets::Backstage(price, placeholder) => println!("Price: {:?}, Name: {:?}", price, placeholder),
            Tickets::Vip(price, placeholder) => println!("Price: {:?}, Name: {:?}", price, placeholder),
            Tickets::Standard(price) => println!("Price: {:?}", price),
        }
    }
}
