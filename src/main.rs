use std::{env::args, fs, io::stdin, path::Path};

use chrono::{DateTime, Local};

fn main() {
    let arguments: Vec<String> = args().collect();

    match arguments.as_slice() {
        [_, file_name] => {
            let buf = std::fs::read_to_string(file_name)
                .unwrap_or_else(|_| panic!("{file_name} was not readable"));
            let lines: Vec<&str> = buf.lines().map(|l| l.trim()).collect();
            repl(&lines);
            println!("You have successfully completed your exercises.");
            println!("Writing completion to file");
            let current_local: DateTime<Local> = Local::now();
            let mut completed_exercises = String::default();
            for line in lines {
                completed_exercises.push_str("✅: ");
                completed_exercises.push_str(line);
                completed_exercises.push('\n');
            }
            let date = current_local.date_naive();
            match Path::exists(Path::new(&date.to_string())) {
                true => {
                    eprintln!("You've already completed your exercises for today.");
                }
                false => {
                    fs::write(&format!("{}.txt", date), completed_exercises)
                        .expect("Failed to write completion to file");
                }
            }
        }
        _ => {
            eprintln!("usage: exercise <exercise_file>.");
        }
    }
}

fn repl(exercises: &[&str]) {
    let non_empty_exercises = exercises.iter().filter(|x| !x.is_empty());
    let mut exercises: Vec<_> = non_empty_exercises.collect();
    let mut input = String::new();
    while !exercises.is_empty() {
        input.clear();
        for (i, exercise) in exercises.iter().enumerate() {
            println!("{}: {}", i + 1, exercise);
        }
        stdin()
            .read_line(&mut input)
            .expect("Could not read stdin.");
        if let Ok(index) = input.trim_end().parse::<usize>() {
            if index == 0 || index > exercises.len() {
                println!("{index} was out of bounds, try again.");
            } else {
                exercises.remove(index - 1);
            }
        }
    }
}
