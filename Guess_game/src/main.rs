use std::io;
use rand::Rng;

fn main() {
    println!("🎯 Enhanced Guessing Game 🎯");
    
    // Select difficulty
    let difficulty = select_difficulty();
    let (min, max, max_attempts) = get_difficulty_settings(difficulty);
    
    println!("I'm thinking of a number between {} and {}!", min, max);
    println!("You have {} attempts to guess it!", max_attempts);
    
    // Generate secret number
    let secret_number = rand::thread_rng().gen_range(min..=max);
    
    // Game loop
    let mut attempts = 0;
    let mut won = false;
    
    while attempts < max_attempts && !won {
        attempts += 1;
        println!("\nAttempt {}/{}", attempts, max_attempts);
        print!("Enter your guess: ");
        
        let guess = get_user_guess();
        
        match guess {
            Some(num) if num == secret_number => {
                println!("🎉 Congratulations! You guessed it in {} attempts!", attempts);
                won = true;
            },
            Some(num) => {
                give_feedback(num, secret_number, attempts, max_attempts);
            },
            None => {
                println!("❌ Invalid input! Please enter a number.");
                attempts -= 1; // Don't count invalid attempts
            }
        }
    }
    
    if !won {
        println!("💀 Game over! The number was {}.", secret_number);
    }
    
    calculate_score(won, attempts, max_attempts, difficulty);
}

fn select_difficulty() -> u32 {
    // TODO: Display difficulty menu and get user choice

      println!("Select difficulty:");
      println!("1. Easy (1-50, 10 attempts)");
      println!("2. Medium (1-100, 8 attempts)");
      println!("3. Hard (1-200, 6 attempts)");
      println!("4. Expert (1-500, 5 attempts)");
      let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
   
       return input.trim().parse::<u32>().expect("Invalid input");
    // 1. Easy (1-50, 10 attempts)
    // 2. Medium (1-100, 8 attempts)  
    // 3. Hard (1-200, 6 attempts)
    // 4. Expert (1-500, 5 attempts)
  
}

fn get_difficulty_settings(difficulty: u32) -> (i32, i32, u32) {
    // TODO: Return (min, max, max_attempts) based on difficulty
    match difficulty {
        1 => (1, 50, 10),   // Easy
        2 => (1, 100, 8),   // Medium
        3 => (1, 200, 6),   // Hard
        4 => (1, 500, 5),   // Expert
        _ => (1, 50, 10),   // Default to easy
    }
}

fn get_user_guess() -> Option<i32> {
    // TODO: Get and parse user input, return None for invalid input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    match input.trim().parse(){
        Ok(num)=>Some(num),
        Err(_)=>None,
    }
}

fn give_feedback(guess: i32, secret: i32, attempts: u32, max_attempts: u32) {
    // TODO: Provide helpful feedback:
    // - Too high/low
    // - How close they are
    // - Remaining attempts
    // - Hints based on attempts remaining

    if guess <secret {
        println!("Too low! The difference  was {}.", secret-guess);
        println!("You have {} attempts remaining.", max_attempts-attempts);
    }

    if guess >secret {
        println!("Too high! The difference was {}.", guess-secret);
        println!("You have {} attempts remaining.", max_attempts-attempts);
    }

}

fn calculate_score(won: bool, attempts: u32, max_attempts: u32, difficulty: u32) {
    // TODO: Calculate and display score based on:
    // - Whether they won
    // - Number of attempts used
    // - Difficulty level

    println!("Score: {} points", if won { attempts } else { 0 });
    println!("Difficulty: {}", match difficulty {
        1 => "Easy",
        2 => "Medium",
        3 => "Hard",
        4 => "Expert",
        _ => "Unknown",
    });
    println!("Max attempts: {}", max_attempts);
    println!("Attempts used: {}", attempts);
    println!("Won: {}", if won { "Yes" } else { "No" });

}