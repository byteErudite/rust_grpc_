fn main() {
    let user = String::from("vaibhav");
    let mut test_vector = Vec::new();
    test_vector.push("string1");

    test_vector.iter().for_each(|value| println!("{}", value));
    println!("Hello, {}", user);
}
