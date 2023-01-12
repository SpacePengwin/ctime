use std::time::Instant;
use std::process::{Command, exit};
use std::env;

fn main() {
    let usage = String::from("Output the total run-time of a program. \nAppend --silence-output or -s to silence the output of the child program.\nExample: `ctime echo 'HELLO'");
    let mut silence_output = false;
    let starting_time = Instant::now();
    let mut args: Vec<String> = env::args().collect();
    Some(args.remove(0)); // Remove rust binary from argument Vec
    if args.is_empty() {
        println!("{}", &usage);
        exit(255);
    }
    if args.contains(&String::from("-h")) {
        println!("{}", &usage);
        exit(0);
    }
    if args.contains(&String::from("--silence-output")) || args.contains(&String::from("-s")) {
        silence_output = true;
        // Remove any existing silence output flags to ensure we don't pass it to the child program as arguments.
        args.retain(|value| *value != String::from("--silence-output"));
        args.retain(|value| *value != String::from("-s"));
    };
    let program = Some(args.remove(0)).expect("Failed to get program argument");
    let result = Command::new(program)
    .args(&args)
    .output();
    match result {
        Ok(result) => {
        let elapsed = starting_time.elapsed();
        if silence_output == false {
            let program_output = String::from_utf8(result.stdout).expect("Failed to convert output.");
            println!("{}", &program_output);
        }
        println!("Time elapsed: {:?}", elapsed);
        },
        Err(err) => {
            println!("Failed to run sub-command: {}", err);
            exit(255);
        }
    }
}
