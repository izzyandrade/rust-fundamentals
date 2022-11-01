fn main() {
    let mut my_string: String = String::from("Hey there! This is Izzy.");

    println!("Length: {}", my_string.len());

    println!("Is empty? {}", my_string.is_empty());

    for str in my_string.split_whitespace(){
        println!("{}", str);
    }

    println!("Does it contain Izzy? {}", my_string.contains("Izzy"));

    my_string.push_str(" I'm learning Rust!");

    println!("{}", my_string)
}