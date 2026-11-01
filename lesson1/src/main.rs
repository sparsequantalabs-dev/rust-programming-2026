fn main() {
    //println ends with exclammatory mark since its macro, not a function. 
    //Macros generate code during runtime
    println!("Welcome, Rust Programming!");
    
    /*Variables in Rust immutable by default*/
    let x = 5;
    /*{} is placeholder for the variable to print*/
    println!("Value of x is {}", x);

}
