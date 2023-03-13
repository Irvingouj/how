use std::env;


fn main() {
    let args: Vec<_> = env::args().collect();
    let iter = args.iter().next();

    let mut full_line = "how".to_owned();
    for arg in iter {

        println!("{} ", arg);
        full_line = full_line.to_string() + " " + arg;
    }


    println!("{}", full_line);
}