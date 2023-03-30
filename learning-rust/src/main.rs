enum Message {
    Print(Box<str>),
    Warning(i32),
    Succes
}

fn main() {
    let curmes = Message::Warning(404);
    Mmanager(curmes);    
}

fn Mmanager(curmes : Message){
    match curmes {
        Message::Print(message) => println!("{}", message),
        Message::Succes => println!("succes"),
        Message::Warning(error_nbr) => println!("error {}", error_nbr)
    }
}
