
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let player_direction:Direction = Direction::Up;
    handle_player_direction(player_direction);
}

fn handle_player_direction(dir: Direction){
    match dir {
        Direction::Up => println!("We are going Up!"),
        Direction::Down => println!("We are going Down!"),
        Direction::Left => println!("We are going Left!"),
        Direction::Right => println!("We are going Right!"),
    }
}