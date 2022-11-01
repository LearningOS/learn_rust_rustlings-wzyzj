// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints


fn main() {
    let optional_word = Some(String::from("rustlings"));

    // TODO: Make this an if let statement whose value is "Some" type
    let word = Some(String::from("rustlings"));
    if word == optional_word {
        println!("The word is: {}", word.is_some());
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    let integer:Option<i8> = Some(10);
    if let integer = optional_integers_vec.pop() {
        println!("current value: {}", integer.is_some());
    }
}
