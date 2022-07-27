use std::io::{self, Write};

fn main() {
    loop {
        let mut bill_amount = String::new();
        print!("Please enter the bill amount: ");
        io::stdout().flush().expect("Unable to flush");
        io::stdin()
            .read_line(&mut bill_amount)
            .expect("Failed to read line from stdin");

        let bill_amount: f64 = match bill_amount.trim().parse() {
            Ok(num) => num,
            // error handling needs some work
            // revisit once I learn how to error handle
            Err(errmsg) => {
                println!("{errmsg}");
                continue;
            }
        };
        println!("You gave {bill_amount}");
    }

    loop {
        let mut tip_rate = String::new();
        print!("Please enter the tip rate: ");
        io::stdout().flush().expect("Unable to flush");
        io::stdin()
            .read_line(&mut tip_rate)
            .expect("Failed to read from stdin");
        let tip_rate: f64 = match tip_rate.trim().parse() {
            Ok(num) => num,
            Err(errmsg) => {
                println!("{errmsg}");
                continue;
            }
        };
    }
}
