fn main() {
    //let go = Direction::Left;

    show_direction(Direction::Left);
    show_direction(Direction::Right);
    show_direction(Direction::Up);
    show_direction(Direction::Down);
}
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn show_direction ( my_direction: Direction) {
    match my_direction {
        Direction::Left => println!("go Left"),
        Direction::Right => println!("go Right"),
        Direction::Up => println!("go Up"),
        Direction::Down => println!("go Down"),
    }
}
