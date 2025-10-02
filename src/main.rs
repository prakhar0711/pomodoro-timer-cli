use colored::Colorize;
use notify_rust::Notification;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};
fn main() {
    let total_hours: f64 = read_user_input();
    let remaining_time: i64 = (total_hours * 3600.0).round() as i64;
    println!("\nStarting timer for {total_hours} hours: ");
    display_notification(&format!("⏰ Starting timer for {} hours ", total_hours));
    run_timer(remaining_time);
    println!("\n{}", "Time's Up!!!".red());
    display_notification(&format!("🔔 Time's Up!!!"));
}

// format time to get all its derivatives(h:m:s)
fn format_time(time_in_seconds: &i64) -> String {
    let hours = time_in_seconds / 3600;
    let minutes = (time_in_seconds % 3600) / 60;
    let remaining_time = time_in_seconds % 60;
    format!("{hours:02} : {minutes:02} : {remaining_time:02}")
}

//read timer input from user
fn read_user_input() -> f64 {
    loop {
        let mut user_input = String::new();
        print!("Enter time in hours : ");
        io::stdout().flush().unwrap();
        if io::stdin().read_line(&mut user_input).is_err() {
            eprintln!("\n{}", "Failed to read input. Try again!!!".red());
            continue;
        }
        match user_input.trim().parse::<f64>() {
            Ok(num) if num > 0.0 => break num,
            Ok(num) if num < 0.0 => eprintln!("{}\n", "lease enter a positive integer!!!".red()),
            Ok(_) => eprintln!("{}\n", "Invalid input!!! Please enter a valid number".red()),
            Err(_) => {
                eprintln!("{}\n", "Invalid input!!! Please enter a valid number".red());
            }
        }
    }
}

//start the timer
fn run_timer(mut remaining_time: i64) {
    while remaining_time > 0 {
        print!(
            "\rTime Remaining : {}",
            format_time(&remaining_time).green()
        );
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        if remaining_time == 0 {
            break;
        }
        remaining_time -= 1;
    }
}
fn display_notification(message: &str) {
    Notification::new()
        .summary("Timer")
        .body(message)
        .show()
        .unwrap();
}
