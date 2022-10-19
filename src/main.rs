
extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    secondary_main();

    println!("Please enter the number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to readline");
    println!("You guessed: {}", guess);
    generating_new_number_rand();

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
