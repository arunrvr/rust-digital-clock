// Using crate chrono that we added to our dependecies
use chrono::{Local};     // using local time

// Character string that we are going tp use for character display
const DIGITS : [[&str; 11]; 7] = [
    ["╓━╖ ","  ╻  "," ╓━╖ ", " ╓━╖ "," ╻ ╻ "," ╓━╖ "," ╓   "," ╓━╖ "," ╓━╖ "," ╓━╖ ","   "],
    ["║ ║ ","  ║  ","   ║ ", "   ║ "," ║ ║ "," ║   "," ║   ","   ║ "," ║ ║ "," ┃ ┃ "," ╻ "],
    ["║ ║ ","  ║  ","   ║ ", "   ║ "," ║ ║ "," ║   "," ║   ","   ║ "," ║ ║ "," ┃ ┃ ","   "],
    ["║ ║ ","  ║  "," ╓━┛ ", " ┣━┫ "," ╘━┫ "," ╘━┓ "," ┣━┓ ","   ║ "," ┣━┫ "," ╘━┫ ","   "],
    ["║ ║ ","  ║  "," ║   ", "   ║ ","   ║ ","   ║ "," ║ ║ ","   ║ "," ║ ║ ","   ┃ ","   "],
    ["║ ║ ","  ║  "," ║   ", "   ║ ","   ║ ","   ║ "," ║ ║ ","   ║ "," ║ ║ ","   ┃ "," ╹ "],
    ["╘━┛ ","  ╹  "," ╘━━ ", " ╘━┛ ","   ╹ "," ╘━┛ "," ╘━┛ ","   ╹ "," ╘━┛ "," ╘━┛ ","   "],
];



fn main() {
    print!("\x1b[2J");      // clearing the entire screen
    print!("\x1b[?25l");    // hiding the cursor
    
    // looping so the program will run forever(until terminated manually)
    loop {
        
        // putting local time in variable 't'.
        let t = Local::now();
        
        // formating time in hours:minutes:seconds format and converting it into string
        let time = t.format("%H:%M:%S").to_string();
        
        // iterating over DIGITS to convert time string into characters

        // iterating over rows of DIGITS 
        for row in &DIGITS {
            
            // iterating over every charcter of time string
            for c in time.chars() {
                
                // translate those charecters, matching from string
                let col = match c {
                    '0'..='9' => c as usize - '0' as usize,
                    ':' => 10,
                    _ => 10,
                };
                
                // printing relavent column from row
                print!("{} ", row[col]);
            }
            println!();  // printing empty line just to be cautious
        }
        
        // print time after 999 milliseconds 
        std::thread::sleep(std::time::Duration::from_millis(999));
        
        // moves cursor to the top
        print!("\x1b[7A");
    }
}
