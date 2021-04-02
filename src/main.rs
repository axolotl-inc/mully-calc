use std::io;

fn main() {
    let mut numb = String::new();
    let mut sound = String::new();
    let mut loves_bum = String::new();
    let mut love_billy = String::new();
    
    println!("Mully Calculater 1.0!");
    
    println!("\nWhat is Mully's favorite number? (i32)");
    io::stdin()
        .read_line(&mut numb)
        .expect("Failed to read line");
        
        
    println!("What is the sound Mully makes? (String)");
    io::stdin()
        .read_line(&mut sound)
        .expect("Failed to read line");
        
        
    println!("Does Mully love bum? (bool)");
    io::stdin()
        .read_line(&mut loves_bum)
        .expect("Failed to read line");
        
        
    println!("Do you love Mully? (bool)");
    io::stdin()
        .read_line(&mut love_billy)
        .expect("Failed to read line");
    
    if numb.trim() == "22" && sound.trim() == "swoodle" && loves_bum.trim() == "true" && love_billy.trim() == "true" {
        println!("\nYou got it right!");
        println!("You may sniff the Mully.");
    } else {
        println!("\nu looz")
    }
}
