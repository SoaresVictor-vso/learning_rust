use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    loop {
        let mut choice = String::new();
        println!( "\n\n---------------------------------------------------------------\n|What do you want?\n|Type 1 for Guess Game;\n|Type 2 for variables;\n|Type 3 for functions;\n|Type 4 for fibonacci sequence4;\n|Type 0 to exit;");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line :/");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your choice was >>> {choice}");

        if choice == 1 {
            guess_game();
        } else if choice == 2 {
            variables();
        } else if choice == 3 {
            functions();
        } else if choice == 4 {
            fibonacci();
        } else if choice == 999 {
            test();
        } else if choice == 0 {
            println!("Successfully finished program!");
            break;
        } else {
            println!("Invalid option, try again!")
        }
    }
}

fn test() {
    let condition = true;
    let number: i8 = if condition {
        let a = 3;
        if a % 2 == 0 {
            0
        } else {
            1
        }
    } else {
        6
    };

    println!("{number}");
}

fn fibonacci() {
    let n = read_line("Digite o número de termos a ser gerado: ");
    let mut penult: u128 = 1;
    let mut last: u128 = 1;

    if n > 0 {
        println!("001º => {penult}");
    }
    if n > 1 {
        println!("002º => {last}");
    }

    for i in 2..n {
        let element: u128 = last + penult;
        let counter =  format!("{:0>3}", (i + 1).to_string());
        println!("{counter}º => {element}");
        penult = last;
        last = element;
    }
}

fn read_line(text: &str) -> u32 {
    loop {
        let mut enter = String::new();
        println!("\n{text}");

        io::stdin()
            .read_line(&mut enter)
            .expect("Failed to read line");

        let enter: u32 = match enter.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break enter;
    }
}

fn functions() {
    //Error, statement (let [...]) does not returns value, so you can't set another let with his value
    //let x = (let y = 5);
    //println!("The y value is {y}");
    //println("The x value is {x}");

    let prt = println!("Pato de agasalho!!!");
    println!("{:?}", prt);

    let block = {
        let x = 5;
        x + 12
    };

    println!("The block value {:?}", block);

    let a = 10;
    let b = 15;
    let sum_a_b = sum(a, b);
    println!("The sum between {a} and {b} is equals to {sum_a_b}");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y //;
}

fn variables() {
    // first
    let mut w = 5;
    println!("\n\n_________________________________________\nThe value of w is: {w}");
    w = 6;
    println!("The value of w is: {w}");

    //second
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("\n\n_________________________________________\nThe value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    //third
    let mut y = 3;
    println!("\n\n_________________________________________\nThe first value of y is: {y}");
    {
        y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    //fourth
    let spaces = "   ";
    println!("\n\n'{spaces}'");
    let spaces = spaces.len();
    println!("'{spaces}'");
}

fn guess_game() {
    let secret_number = rand::thread_rng().gen_range(1..=128);
    //println!("The secret number is: {secret_number}");

    //      # Chapter 2 finished!!!
    loop {
        let mut guess = String::new();
        println!("\nGuess a number: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("--------------\n#            #\n# YOU WIN!!! #\n#            #\n--------------");
                break;
            }
        }
    }
}
