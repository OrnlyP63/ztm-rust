// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

fn main() {
    let mut stocks = HashMap::new();
    stocks.insert("Chair", 5);
    stocks.insert("Bed", 3);
    stocks.insert("Table", 2);
    stocks.insert("Couch", 0);

    let mut total_socks = 0;

    for (item, qty) in stocks.iter() {
        total_socks = total_socks + qty;
        let stock_count = if qty == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", qty)
        };

        println!("item={:?}, stock={:?}", item, stock_count);
    }
    println!("total stock={:?}", total_socks);
}
