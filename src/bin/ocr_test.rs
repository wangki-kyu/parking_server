use std::{env, process::Command};

fn main() {
    // py script call!
    let args: Vec<String> = env::args().collect();

    println!("총 인자 개수: {}", args.len());

    if args.len() > 2 {
        println!("first args: {}", args[1]);
        println!("second args: {}", args[2]);
    }

    let output = Command::new("python")
        .arg(&args[1])
        .arg(&args[2])
        .output().unwrap();

    let res_str = String::from_utf8(output.stdout).unwrap();
    println!("{}", res_str);
}
