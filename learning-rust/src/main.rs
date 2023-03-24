struct Man {
    name : String,
    age : u16,
    job : String
}

impl Man {
    fn get_in_string(&self) -> String {
        String::from(&self.name + " " + &self.age.to_str() + " " + &self.job)
}}

fn main() {
    let man1 = Man { 
        name: String::from("james"), 
        age: 122, 
        job: String::from("ingenieur")
    };
    println!("{}", man1.get_in_string());
}
