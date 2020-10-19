fn main() {
    let str = String::from("Hello World");
    let len = first_word(&str);

    println!("First word: {}", len);

    // ---------------------------

    // String Slices

    let firstTwo = &str[0..2];

    println!("First two characters of string: {}", firstTwo);

    let len2 = str.len();

    // Same things
    let slice = &str[0..len2];
    let slice = &str[..];

    // ---------------------------

    // let mut str2 = String::from("Hello World 2");
    //
    // let word = first_word(&str2);
    //
    // str2.clear(); // error!
    //
    // println!("This first word is: {}", word);

    // ---------------------------
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     println!("Bytes: {:?}", bytes.iter());
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
