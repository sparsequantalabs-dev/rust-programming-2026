fn main() {
    //Scalar Types
    let age: u32 = 80; //non-negative 32 bit integer 
    let num: i32 = -100; //default for numbers if you don't specify type
    let temperature: f64 = 36.6; //64 bit floating point number for decimals
    let isit_hot: bool = true; //Boolean with true or false
    let v_char: char = 'A'; //A single character

    println!(
        "Age --> {}, Number --> {}, Temperature --> {}, Hot --> {}, Letter --> {} ",
        age, num, temperature, isit_hot, v_char
    );

    //Compound types
    //tuple
    let person: (&str, u32, u32) = ("Sai", 4, 2022);
    println!(
        "My Name is {} and age is {}, my born year is {}",
        person.0, person.1, person.2
    );

    //Arrays
    let marks: [i32; 3] = [80, 75, 90];
    println!(
        "My Marks , Maths >> {}, Chemistry >> {}, Physics >> {}",
        marks[0], marks[1], marks[2]
    );
}
