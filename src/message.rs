pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

pub fn handle_message(msg: Message) {
    match msg {
        Message::Move { x, y } => {
            println!("asking to move to ({}, {})", x, y);
        }
        Message::ChangeColor(r, g, b) => {
            println!("asking to change color to rgb({}, {}, {})", r, g, b);
        }
        Message::Quit => {
            println!("asking to quit");
        }
    }
}
