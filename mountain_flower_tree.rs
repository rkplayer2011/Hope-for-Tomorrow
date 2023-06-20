// This code is written in Rust 1.18.0

// Import the necessary modules
extern crate chrono;

use chrono::{Duration, NaiveDateTime, Utc};

// Main function
fn main() {
    // Declare the desired date as a NaiveDateTime 
    let date = NaiveDateTime::from_timestamp(1564184400, 0);
    // Get the current date
    let cur_date = Utc::now();
    // Calculate the difference between the two dates
    let diff = cur_date.signed_duration_since(date);
    // Print out the difference
    println!("Difference is {:?}", diff);
    
    // Loop through each day in the difference
    for days in 0..diff.num_days() {
        let tomorrow = date + Duration::days(days);
        // Print out a hopeful message for each day
        println!("Hope for tomorrow: {:?}", tomorrow);
    }
}