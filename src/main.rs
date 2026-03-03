fn main() {

    println!("Rust Practice - Simple Grade Example");

    // Variables
    let student_name = "Kyle";
    let mut score = 85;

    println!("Student: {}", student_name);
    println!("Original Score: {}", score);

    // Mutability
    score += 5;  // bonus points
    println!("Updated Score: {}", score);

    // Function call
    let letter_grade = calculate_grade(score);
    println!("Letter Grade: {}", letter_grade);

    // If / Else
    if score >= 90 {
        println!("Great job!");
    } else {
        println!("Keep improving!");
    }

    // Match example
    match letter_grade {
        'A' => println!("Excellent work."),
        'B' => println!("Good job."),
        'C' => println!("You passed."),
        _ => println!("Needs improvement."),
    }

    // Loop example
    let scores = vec![70, 85, 90];

    println!("All Scores:");
    for s in scores {
        println!("{}", s);
    }
}

// Function to calculate grade
fn calculate_grade(score: i32) -> char {
    if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else {
        'F'
    }
}