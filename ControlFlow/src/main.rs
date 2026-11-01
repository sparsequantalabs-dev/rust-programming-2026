fn main() {
    let number: i32 = 100;

    //If block
    if number % 2 == 0 {
        println!("Even Number");
    } else {
        println!("Odd Number");
    }

    //Loop
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 5 {
            break counter;
        }
    };

    println!("Loop Counter >> {}", result);

    //While-loop
    println!("Assigning result {} to whilecounter", result);
    let mut while_counter: i32 = result;
    while while_counter != 0 {
        println!("whileCounter --> {}", while_counter);
        while_counter -= 1;
    }

    println!("After while Counter >> {}", result);

    //For-Loop
    for loop_cnt in 1..result {
        println!("For Loop iteration...{}", loop_cnt);
    }

    //Array-forloop

    let marks: [i32; 3] = [100, 90, 80];
    for mark in marks {
        println!("Mark is {}", mark);
    }
}
