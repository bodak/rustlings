// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    if let value = Some(String::from("rustlings")) {
        println!("the value of optional value is: {}", value.unwrap());
    } else {
        println!("The optional value doesn't contain anything!");
    }

    let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_values_vec.push(Some(x));
    }

    while let Some(Some(value)) = optional_values_vec.pop() {
        println!("current value: {}", value);
    }
}
