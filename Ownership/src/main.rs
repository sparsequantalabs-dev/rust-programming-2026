fn main() {
    //This creates a heap growable string different from &str
    let str1 = String::from("Hello");
    /*help: consider cloning the value if the performance cost is acceptable
        |
      6 |     let str2 = str1.clone();
        |                    ++++++++ */

    /*To mitigate the above error clone the str1 value*/
    //let str2 = str1;
    let mut str2 = str1.clone();
    str2 = str2 + ", World!";
    println!("String 1 >> {}, String 2 >> {}",str1, str2);
}
