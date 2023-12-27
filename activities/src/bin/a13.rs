// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

struct Number {
    value: i32,
}

fn main() {
    let numbers = vec![
        Number{ value: 10 },
        Number{ value: 20 },
        Number{ value: 30 },
        Number{ value: 40 },
    ];

    for num in &numbers{
        if num.value != 30{
            println!("Number: {:?}", num.value);
        } else {
            println!("Number: 'thirty'");
        }
    }

    println!("Length of vector: {:?}", numbers.len());
}
