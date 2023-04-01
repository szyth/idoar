use std::{env, process::Command, thread};

fn main() {
    let args: Vec<String> = env::args().collect();

    let url = args[1].clone();
    let mut ids = vec![];
    let start_range: i32 = args[2].parse().expect("Invalid integer");
    let end_range: i32 = args[3].parse().expect("Invalid integer");

    for i in start_range..=end_range {
        ids.push(i)
    }

    let mut handles = vec![];

    // create number of threads same as total number of ids
    for id in 1..=ids.len() {
        // let mut ids = ids.clone();
        let url = url.clone();
        let handle = thread::spawn(move || {

            let command = format!(
                "curl {}{} -o output_{}.txt --silent",
                url, id, id
            );
            let _output = Command::new("bash")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("Failed to execute command");

            println!("{:#?}", command)
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
}
