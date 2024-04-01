fn main(){
    println!("hello world");
    let mut input = String::new();
    let bites = std::io::stdin().read_line(&mut input).unwrap();
    println!("hi {}, {} bites",input, bites);
}