const MAXIMUM_NUMBER: u8 = 20;

fn main() {

    // in Rust by default every variable created is immutable.
    // therefore, to change it, we use the "mut" keyword

    // you can define the datatype of a variable by using a colon and the type name "let x: i64"

    let mut x: i64 = 45;

    println!("The value of x is {}", x);

    x = 55;

    println!("\n\n=========== IF/ELSE ==========");
    if x < 50 {
        println!("The number is less than 50");
    } else {
        println!("The number is greater than 50")
    } 

    let mut n = 0;

    println!("\n\n=========== LOOP ==========");
    loop {
        n += 1;
        if n == 7 {
            continue;
        }
        if n > 10 {
            break;
        }
        println!("n --> {}", n);
    }

    println!("\n\n=========== WHILE LOOP ==========");
    while n > 0 {
        println!("n --> {}", n);
        n -= 1;
    }

    println!("\n\n=========== FOR LOOP ==========");
    let animals = vec!["Rabbit", "Dog", "Cat"];

    for a in animals.iter() {
        println!("{}", a);
    }

    println!("\n========= NOW WITH INDEX =========");
    for (index, a) in animals.iter().enumerate() {
        println!("Animal --> {} | Index --> {}", a, index);
    }

    println!("\n========== TUPLES =========");
    let tuple = ("Izzy", 25, 1.80);
    println!("0 ---> {}", tuple.0);
    println!("1 ---> {}", tuple.1);
    println!("2 ---> {}", tuple.2);

}
