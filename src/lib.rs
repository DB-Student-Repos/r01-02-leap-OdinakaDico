use std::io;

fn main() {
        println!("Enter a year:");

        let mut year_string = String::new();
        io::stdin().read_line(&mut year_string)
            .expect("Failed to read line");

        let year: i32 = year_string.trim().parse()
            .expect("Please type a number!");

        if is_leap_year(year) {
            println!("{} is a leap year.", year);
        } else {
            println!("{} is not a leap year.", year);
        }
}

fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
