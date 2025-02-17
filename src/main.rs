use ram::Ram;
mod ram;

fn main() {
    // Load RAM program code from a file given in the command line argument
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let code = std::fs::read_to_string(filename).unwrap();

    // Input for RAM execution are the next command line arguments
    let input: Vec<i64> = args[2..].iter().map(|s| s.parse().unwrap()).collect();
    
    // Compile and run the RAM program on given user input
    let mut ram = Ram::new(&code);
    let r0 = ram.execute(&input);
    println!("r0 = {}", r0);
}
