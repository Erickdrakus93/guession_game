use std::io;

fn main() {
    println!("Hello, world!");
    secondary_main();

    println!("Please enter the number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to readline");
    println!("You guessed: {}", guess);
}

// This is an utility function
fn secondary_main(){
    /*
    This an utility function!
    */
    print!("The new version of the game");
}

fn alternativity_function(){
    /*
    Function to create the main form
    */
    
}
