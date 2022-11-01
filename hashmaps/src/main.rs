use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new(); // creating hashMap

    // inserting new values
    marks.insert("Rust Programming", 79);
    marks.insert("Web Development", 96);
    marks.insert("Smart Contracts", 80);
    marks.insert("Leadership", 87);

    //getting length of hashMap
    println!("The length of the HashMap is: {}", marks.len());

    //getting a single value from the hashmap
    match marks.get("Web Development"){
        Some(mark) => println!("You got {} for Web Development", mark),
        None => println!("You didn't study Web Development!")
    }

    //remove a value
    marks.remove("Leadership");

    //looping through all values
    for (key, value) in &marks {
        println!("For {} you got {}", key, value);
    }

    //check if key exists
    println!("Did you study C++? {}", marks.contains_key("C++"));
}
