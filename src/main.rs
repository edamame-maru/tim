use chrono::{Datelike, Local};
use std::io::{self, Write};

const MONDAY: [&str; 9] = [
    "Chemistry",
    "Chemistry",
    "Biology",
    "Biology",
    "Computer Science",
    "Computer Science",
    "Physics",
    "Music",
    "Music",
];

const TUESDAY: [&str; 9] = [
    "Latin",
    "Latin",
    "English",
    "English",
    "Biology",
    "Biology",
    "Chemistry",
    "Italian",
    "Italian",
];

const WEDNESDAY: [&str; 9] = [
    "Music",
    "Music",
    "Maths",
    "Maths",
    "English",
    "English",
    "Games Option",
    "Games Option",
    "Games Option",
];

const THURSDAY: [&str; 9] = [
    "Italian",
    "Italian",
    "Physics",
    "Maths",
    "Latin",
    "Latin",
    "PSHE",
    "Computer Science",
    "Computer Science",
];

const FRIDAY: [&str; 9] = [
    "Maths",
    "Maths",
    "English",
    "PE",
    "PE",
    "PE",
    "Physics",
    "Physics",
    "Chemistry",
];

fn main() {
    println!(
        "Soichi Ueda Upper Remove: https://ucs.myschoolportal.co.uk/showme/pupil-timetable..."
    );
    day();
}

fn day() {
    let today = Local::now();
    let weekday = today.weekday();

    match weekday {
        chrono::Weekday::Mon => {
            print!("MONDAY\n");
            io::stdout().flush().unwrap();

            for i in 0..MONDAY.len() {
                println!("{} {}", i, MONDAY[i]);
            }
        }
        chrono::Weekday::Tue => {
            print!("TUESDAY\n");
            io::stdout().flush().unwrap();

            for i in 0..TUESDAY.len() {
                println!("{} {}", i, TUESDAY[i]);
            }
        }
        chrono::Weekday::Wed => {
            print!("WEDNESDAY\n");
            io::stdout().flush().unwrap();

            for i in 0..WEDNESDAY.len() {
                println!("{} {}", i, WEDNESDAY[i]);
            }
        }
        chrono::Weekday::Thu => {
            print!("THURSDAY\n");
            io::stdout().flush().unwrap();

            for i in 0..THURSDAY.len() {
                println!("{} {}", i, THURSDAY[i]);
            }
        }
        chrono::Weekday::Fri => {
            print!("FRIDAY\n");
            io::stdout().flush().unwrap();

            for i in 0..FRIDAY.len() {
                println!("{} {}", i, FRIDAY[i]);
            }
        }
        _ => {
            println!("No school!");
        }
    }
}
