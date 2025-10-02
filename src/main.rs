use colored::Colorize;
use notify_rust::Notification;
use std::env;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};
fn main() {
    let args: Vec<String> = env::args().collect();
    let remaining_time: i64;
    let total_hours: f64;
    if args.len() == 2 {
        match parse_input(&args[1]) {
            Ok(num) => {
                total_hours = num;
                remaining_time = calculate_remaining_time(total_hours);
                display_output(remaining_time, total_hours);
            }
            Err(err) => {
                eprintln!("ERROR PARSING INPUT : {}", err.red());
            }
        }
    } else {
        total_hours = read_user_input();
        remaining_time = calculate_remaining_time(total_hours);
        display_output(remaining_time, total_hours);
    }
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
        match parse_input(&user_input) {
            Ok(num) => break num,
            Err(err) => {
                eprintln!("ERROR PARSING INPUT : {}", err.red());
            }
        }
    }
}

//start the timer
fn run_timer(mut remaining_time: i64) {
    while remaining_time >= 0 {
        print!(
            "\rTime Remaining : {}",
            format_time(&remaining_time).green()
        );
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        remaining_time -= 1;
    }
}

// display notification
fn display_notification(message: &str) {
    if let Err(e) = Notification::new().summary("Timer").body(message).show() {
        eprintln!("{} : {e}", "Failed to show notification".red());
    }
}

//calculate remaining time in seconds
fn calculate_remaining_time(input: f64) -> i64 {
    (input * 3600.0).round() as i64
}

//displaying the output on the commandline
fn display_output(remaining_time: i64, total_hours: f64) {
    let unit = if total_hours == 1.0 { "hour" } else { "hours" };
    println!("\nStarting timer for {total_hours} {unit}: ");
    display_notification(&format!("⏰ Starting timer for {total_hours} hours "));
    run_timer(remaining_time);
    println!("\n{}", "Time's Up!!!".red());
    display_notification("🔔 Time's Up!!!");
}

//parsing user input to float for inputs like 1.5hours etc
fn parse_input(input: &str) -> Result<f64, String> {
    match input.trim().parse::<f64>() {
        Ok(num) if num > 0.0 => Ok(num),
        Ok(_) => Err("Invalid input!!! Please enter a positive number".to_string()),
        Err(_) => Err("Invalid input!!! Please enter a valid number".to_string()),
    }
}
