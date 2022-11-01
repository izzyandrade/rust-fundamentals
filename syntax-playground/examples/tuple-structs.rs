struct RGBColor(u8, u8, u8);

fn main() {
    let new_color = RGBColor(255, 0, 0);
    println!("{}, {}, {}", new_color.0, new_color.1, new_color.2);
}
