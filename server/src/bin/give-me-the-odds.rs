use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let milenium_falcon_file = env::args().nth(1).expect("Missing Milenium Falcon file");
    let empire_file = env::args().nth(2).expect("Missing Empire file");
    let probability = server::run_for_bin(milenium_falcon_file.to_string(), empire_file.to_string());
    println!("probability {}", probability?);
    Ok(())
}