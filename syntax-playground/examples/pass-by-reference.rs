struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let color = Color {
        red: 0,
        green: 0,
        blue: 255
    };

    print_color(&color);
}

fn print_color(c: &Color){
    println!("{}, {}, {}", c.red, c.green, c.blue);
}