use std::io;

// Function to calculate average
fn calculate_average(total_marks: f32, num_subjects: u32) -> f32 {
    if num_subjects == 0 {
        0.0
    } else {
        total_marks / num_subjects as f32
    }
}

// Function to assign grade
fn assign_grade(average: f32) -> char {
    if average >= 90.0 {
        'A'
    } else if average >= 75.0 {
        'B'
    } else if average >= 60.0 {
        'C'
    } else {
        'D'
    }
}

fn main() {
    println!("📘 Student Grade Calculator");

    // Get student name
    println!("Enter student name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");

    // Get total marks
    println!("Enter total marks:");
    let mut total_marks_str = String::new();
    io::stdin().read_line(&mut total_marks_str).expect("Failed to read marks");
    let total_marks: f32 = total_marks_str.trim().parse().expect("Please enter a valid number");

    // Get number of subjects
    println!("Enter number of subjects:");
    let mut num_subjects_str = String::new();
    io::stdin().read_line(&mut num_subjects_str).expect("Failed to read subjects");
    let num_subjects: u32 = num_subjects_str.trim().parse().expect("Please enter a valid number");

    // Calculate average and grade
    let average = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(average);

    // Output
    println!("\n🎓 Student: {}", name.trim());
    println!("📊 Average Marks: {:.2}", average);
    println!("🏅 Grade: {}", grade);
}

