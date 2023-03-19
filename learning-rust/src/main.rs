fn main() {
    let mut string = String::from("Hello World");
    println!("{}", getdaword(&mut string));
}
fn getdaword(s : &mut String) -> String{
    let mut result: String = String::from("");
    for char in s.chars() {
        if char == ' '{
            break;
        } else {
            result.push(char);
        }
    }
    return result
}