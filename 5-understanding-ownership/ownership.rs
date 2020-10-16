fn main() {
    // Memory and Allocation

    let x = 5; // The stack data
    let y = x;

    println!("x: {}, y: {}", x, y);

    let s1 = String::from("hello"); // The heap data
    let s2 = s1;
    // println!("{}, world!", s1);  -> This line is not working. Because when we make s2 = s1, s1 is no longer valid.

    // Ownership and Functions

    let str = String::from("Hello"); // str comes into scope

    takes_ownership(str); // s's value moves into the function...
                          // ... and so is no longer valid here

    let num = 5; // num comes into scope

    makes_copy(num); // num would move into the function,
                     // but i32 is Copy, so it’s okay to still
                     // use num afterward

    // Return Values and Scope

    let str1 = gives_ownership(); // gives_ownership moves its return
                                  // value into str1

    let str2 = String::from("hello"); // str2 comes into scope

    let str3 = takes_and_gives_back(s2); // str2 is moved into
                                         // takes_and_gives_back, which also
                                         // moves its return value into str3

    let str4 = String::from("hello");

    let (str5, len) = calculate_length(str4);

    println!("The length of '{}' is {}.", str5, len);
}

// some_string comes into scope
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

// some_integer comes into scope
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// gives_ownership will move its
// return value into the function
// that calls it
fn gives_ownership() -> String {
    let str = String::from("hello"); // str comes into scope

    str // str is returned and
        // moves out to the calling
        // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
