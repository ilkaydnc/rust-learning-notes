fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // ---------------------------

    // Mutable References

    let mut s2 = String::from("Hello");

    change(&mut s2);

    println!("Changed value: {}.", s2);

    // ---------------------------

    let mut s3 = String::from("Hello");

    // This code will fail. Because you can have only one mutable reference to a
    // particular piece of data in a particular scope.

    // let r1 = &mut s3;
    // let r2 = &mut s3;

    // println!("{}, {}", r1, r2);

    // ---------------------------

    // let r1 = &s3;  no problem
    // let r2 = &s3;  no problem
    // let r3 = &mut s3;  BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    // ---------------------------

    let r1 = &s3; // no problem
    let r2 = &s3; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s3; // no problem
    println!("{}", r3);

    // ---------------------------

    // Dangling References

    // this function's return type contains a borrowed value, but there is no value
    // for it to be borrowed from.

    // let reference_to_nothing = dangle();

    let new_str = no_dangle();
}

// str is a reference to a String
fn calculate_length(str: &String) -> usize {
    str.len()
} // Here, str goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// dangle returns a reference to a String
// fn dangle() -> &String {
//     let str = String::from("hello"); // str is a new String
//
//     &str // we return a reference to the String, str
// }
// Here, str goes out of scope, and is dropped. Its memory goes away.
// Danger! Because str is created inside dangle, when the code of dangle is finished,
// str will be deallocated. But we tried to return a reference to it.
// That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
