struct Man {
    name : String,
    age : u16,
    job : String
}

impl Man {
    fn get_in_string(&self) -> String {
      format!("{} {} {}", self.name, self.age, self.job) 
}}

fn main() {
    let man1 = Man { 
        name: String::from("james"), 
        age: 122, 
        job: String::from("ingenieur")
    };
    println!("{}", man1.get_in_string());
}
