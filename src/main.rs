use std::{
    io::{self, Write},
    thread,
    time::Duration,
};
fn main() {
    let total_hours: f64 = loop {
        let mut user_input = String::new();
        print!("Enter time in hours : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();
        match user_input.trim().parse::<f64>() {
            Ok(num) if num > 0.0 => break num,
            Ok(num) if num < 0.0 => eprintln!("Please enter a positive integer!!!\n"),
            Ok(_) => eprintln!("Invalid input!!! Please enter a valid number\n"),
            Err(_) => {
                eprintln!("Invalid input!!! Please enter a valid number\n");
            }
        }
    };
    let mut total_secs: i64 = (total_hours * 3600.0).round() as i64;
    println!("\nStarting timer for {total_hours} hours: ");
    while total_secs > 0 {
        print!("\rTime Remaining : {}", format_time(&total_secs));
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        total_secs -= 1;
        if total_secs == 0 {
            break;
        }
    }
    println!("Timer Finished!!!");
}
fn format_time(time_in_seconds: &i64) -> String {
    let hours = time_in_seconds / 3600;
    let minutes = (time_in_seconds % 3600) / 60;
    let seconds = time_in_seconds % 60;
    format!("{hours:02} : {minutes:02} : {seconds:02}")
}
