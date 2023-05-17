pub mod ledger;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() != 3 {
        print!("Pass in the number of workers and ledger name\n");
        process::exit(1);
    }
    ledger::init_bank_workers(args[1].parse().unwrap(), args[2].clone());
    
}
