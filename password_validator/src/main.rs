fn main() {
    println!("🔐 Password Strength Validator 🔐");
    
    let test_passwords = [
        "123456",
        "password", 
        "MyP@ssw0rd",
        "Tr0ub4dor&3",
        "correcthorsebatterystaple",
        "R4nd0m!P@ssW0rd#2023",
    ];
    
    for password in test_passwords {
        println!("\nAnalyzing: '{}'", password);
        let strength = analyze_password(password);
        display_analysis(password, strength);
    }
    
    // Interactive mode
    interactive_mode();
}

#[derive(Debug, PartialEq)]
enum PasswordStrength {
    VeryWeak,
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

fn analyze_password(password: &str) -> PasswordStrength {
    let mut score = 0;
    
    // TODO: Implement scoring logic based on:
    // - Length (bonus for 8+, 12+, 16+ characters)
    // - Character diversity (lowercase, uppercase, numbers, symbols)
    // - No common patterns (123, abc, repeated chars)
    // - No dictionary words
    
    // Convert score to strength level
    match score {
        0..=2 => PasswordStrength::VeryWeak,
        3..=4 => PasswordStrength::Weak,
        5..=6 => PasswordStrength::Medium,
        7..=8 => PasswordStrength::Strong,
        _ => PasswordStrength::VeryStrong,
    }
}

fn display_analysis(password: &str, strength: PasswordStrength) {
    println!("Length: {} characters", password.len());
    println!("Strength: {:?}", strength);
    
    // TODO: Display detailed analysis:
    // - Character type breakdown
    // - Identified weaknesses
    // - Suggestions for improvement
    // - Estimated crack time
    
    display_improvement_suggestions(password, &strength);
}

fn display_improvement_suggestions(password: &str, strength: &PasswordStrength) {
    // TODO: Provide specific suggestions based on analysis
    // - "Add uppercase letters"
    // - "Include special characters"
    // - "Increase length to at least 12 characters"
    // - "Avoid common patterns"
}

fn interactive_mode() {
   loop{

       let mut password = String::new();
       io::stdin().read_line(&mut password).expect("Failed to read input");
   
       println!("Analyzing: '{}'", password);
   }
    // TODO: Allow user to enter passwords for analysis
    // - Continuous input loop
    // - Option to generate secure passwords
    // - Comparison mode for multiple passwords
}

// Helper functions to implement:
fn has_lowercase(password: &str) -> bool { false }
fn has_uppercase(password: &str) -> bool { false }
fn has_digits(password: &str) -> bool { false }
fn has_special_chars(password: &str) -> bool { false }
fn has_repeated_chars(password: &str) -> bool { false }
fn has_sequential_chars(password: &str) -> bool { false }