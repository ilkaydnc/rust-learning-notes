// Constant Variables (Global scope)

const MAX_POINTS: u32 = 100_000;

//noinspection ALL
fn main() {
    println!("Constant value: {}", MAX_POINTS);

    // Mutable Variables

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Numeric Operations

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 6;

    // Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let (_x, y, _z) = (five_hundred, six_point_four, one);

    println!("The value of y is: {}", y);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let first = arr[0];
    let second = arr[1];

    println!("First: {}, Second: {}", first, second);

    // Function Call

    example_function(10, 20.3);

    let new_value: i32 = plus_one(30);
    println!("Plus One: {}", new_value);

    // If Statement

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // If Statement in a Let Statement

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // Loops

    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("Counter: {}", counter);

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // Loops with While

    let mut number = 3;

    while number != 0 {
        println!("While: {}!", number);

        number -= 1;
    }

    // Loops with For

    for number in (1..4).rev() {
        println!("For: {}!", number);
    }
}

// Function Definition

fn example_function(x: i32, y: f32) {
    println!("Example Function x: {}, y: {}", x, y);
}

fn plus_one(value: i32) -> i32 {
    value + 1
}
