
extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    /*
    The next steps is to introduce the generating with a loop,
    inside of the main form .
    */
    println!("Hello, world!");
    secondary_main();

    println!("Please enter the number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to readline");
    println!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse()
          .expect("Please type a number");
    let generating_random = rand::thread_rng(). gen_range(1,134);
    println!("The random number is {}",generating_random);
    //Here we compare the latest version to the next
    match guess.cmp(&generating_random){
        Ordering::Less => {println!("Too small!");}
        Ordering::Greater => {println!("Too big!");}
        Ordering::Equal => {println!("You win!");}
    }
    //Finally we call the intermidate function
    //generating_new_number_rand();
    let number = five();
    println!("The number is {}", number);
}

//Here we define the rest of utility functions
// This is an utility function
fn secondary_main(){
    /*
    This an utility function!
    */
    print!("The new version of the game");
}

fn generating_new_number_rand(){
    /*
    Generating the new form of the main form of the current form
    */
    let generating_new_number = rand::thread_rng().gen_range(1,10);
    println!("The generatin number is: {} ",generating_new_number);
}


fn five() -> i32{
    /*
    This function is to generate the returning statment.
    */
    return 5;
}
