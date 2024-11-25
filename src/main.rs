use chrono::Local;
use core::time;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::{
        Clear,
        ClearType::{All, Purge},
        DisableLineWrap,
    },
};
use std::{collections::HashMap, thread::sleep};

const NUM_HEIGHT: usize = 5;

fn main() -> Result<(), std::io::Error> {
    let numbers: HashMap<String, Box<[&str]>> = HashMap::from([
        (
            "0".to_string(),
            Box::from([" -- ", "|  |", "|  |", "|  |", " -- "]),
        ),
        (
            "1".to_string(),
            Box::from(["    ", "   |", "   |", "   |", "    "]),
        ),
        (
            "2".to_string(),
            Box::from([" -- ", "   |", " -- ", "|   ", " -- "]),
        ),
        (
            "3".to_string(),
            Box::from([" -- ", "   |", " -- ", "   |", " -- "]),
        ),
        (
            "4".to_string(),
            Box::from(["    ", "|  |", " -- ", "   |", "    "]),
        ),
        (
            "5".to_string(),
            Box::from([" -- ", "|   ", " -- ", "   |", " -- "]),
        ),
        (
            "6".to_string(),
            Box::from([" -- ", "|   ", " -- ", "|  |", " -- "]),
        ),
        (
            "7".to_string(),
            Box::from([" -- ", "   |", "    ", "   |", "    "]),
        ),
        (
            "8".to_string(),
            Box::from([" -- ", "|  |", " -- ", "|  |", " -- "]),
        ),
        (
            "9".to_string(),
            Box::from([" -- ", "|  |", " -- ", "   |", "    "]),
        ),
    ]);

    'clock: loop {
        execute!(
            std::io::stdout(),
            Hide,
            Clear(Purge),
            Clear(All),
            DisableLineWrap,
            MoveTo(0, 0)
        )?;
        let local_now = Local::now().time().format("%H:%M:%S").to_string();

        let [hours, minutes, seconds] = local_now
            .split(":")
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();

        for i in 0..NUM_HEIGHT {
            let mut line = String::new();
            [hours, minutes, seconds].map(|t| {
                for char in t.chars() {
                    let num = numbers.get(&char.to_string()).unwrap()[i];
                    line.push_str(num);
                    line.push_str(" ");
                }

                if (i == 1 || i == 3) && line.len() <= 26 {
                    line.push_str(" * ");
                } else {
                    line.push_str("   ");
                }
            });
            println!("{}", line);
        }
        println!("Press Ctrl-C to quit.");
        sleep(time::Duration::from_secs(1));
    }
    Ok(())
}
