fn main() {
    greet("Sai");
    println!("Addition >> {}", add(10,30));
}

fn greet(name: &str){
    println!("My name is {}",name);
}

fn add(a: i32, b:i32) -> i32 {
   a + b //its a expression, its value automatically becomes return value
}
