use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    loop {
        let mut choice = String::new();
        println!( "\n\n---------------------------------------------------------------\n|What do you want?\n|Type 1 for Guess Game;\n|Type 2 for variables;\n|Type 3 for functions;\n|Type 4 for fibonacci sequence;\n|Type 5 for structs;\n|Type 6 for rectangles;\n|Type 0 to exit;");

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
        } else if choice == 5 {
            structs();
        } else if choice == 6 {
            rectangles();
        } else if choice == 999 {
            let mut a = String::from("TEsteeeee");
            println!("{a}");
            a = test(a);
            println!("{a}");
        } else if choice == 0 {
            println!("Successfully finished program!");
            break;
        } else {
            println!("Invalid option, try again!")
        }
    }
}

fn test(str: String) -> String {
    println!("{str}");
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
    return str;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

fn rectangles() {
    println!("Rectangles\t");
    let rect = Rectangle {
        width: 14,
        heigth: 10,
    };
    println!("{:#?}", rect);
    println!(
        "The rectangle with heigth {} and width {} is {}.",
        rect.heigth,
        rect.width,
        calculate_rectangle_area(&rect)
    );
    dbg!(rect);
}

fn calculate_rectangle_area(rect: &Rectangle) -> u32 {
    rect.width * rect.heigth
}

struct User {
    active: bool,
    email: String,
    password_hash: String,
    username: String,
}

struct Color(u8, u8, u8);

fn structs() {
    println!("Structs lorem");
    let mut user1 = new_user(
        String::from("lorem@ypsum.com"),
        String::from("12312312312312312313123"),
        String::from("lorem"),
    );
    let mut user2 = User {
        email: String::from("ypsum_lorem@gmail.com"),
        ..user1
    };
    let user3 = User {
        active: user1.active,
        email: user1.email,
        password_hash: String::from("432432432423423423423"),
        username: String::from("testeee"),
    };
    //println!("{}", user1.email); não mais resusável
    println!("{}", user2.email);

    user1.email = String::from("lorem@gmail.com");
    println!("{}", user1.email);
    println!("{}", user2.email);

    let background_color = Color(255, 100, 200);
    println!(
        "R: {:#0x}, G: {:#0x}, B: {:#0x}",
        background_color.0, background_color.1, background_color.2
    );
}

fn new_user(email: String, password_hash: String, username: String) -> User {
    User {
        active: true,
        email,
        password_hash,
        username,
    }
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
        let counter = format!("{:0>3}", (i + 1).to_string());
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
