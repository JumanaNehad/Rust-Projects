#![deny(clippy::all)]

fn concatenate_strings(n1: &str, n2: &str) -> String {
    // let result = format!("{}{}", n1, n2);
    //or
    let mut result = String::from(n1);
    result.push_str(n2);
    result
}

fn main() {
    let string1 = String::from("Jumana ");
    let string2 = String::from("Nehad");
    let concatenate_string = concatenate_strings(&string1, &string2);

    println!("{}", concatenate_string);
}
