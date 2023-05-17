use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use self::bank::Bank;
pub mod bank;
struct Ledger {
	from: i32,
	to: i32,
	amount: i32,
  	mode: i32,
	ledger_id: i32,
}

/// Creates new bank and ledger object
/// Then removes items from ledger and executes each instruction
/// Joins all threads at the end
///
/// # Arguments
///
/// * 'num_workers' - number of worker threads
/// * `filename` - the name of the file to be parsed
pub fn init_bank_workers(num_workers: i32, filename: String){
    //create new vector that can be shared with all threads
    let ledger: Arc<Mutex<Vec<_>>> = Arc::new(Mutex::new(Vec::new()));
    //function call to load the vector with ledgers
	load_ledger(&mut ledger.lock().unwrap(), filename);
    //create new bank object that is thread-safe
    let bank = Arc::new(Mutex::new(Bank{
        num:10,
        num_succ:Arc::new(Mutex::new(0)),
        num_fail:Arc::new(Mutex::new(0)),
        accounts:Vec::new(),
    }));
    //initialize the accounts within the bank object
    bank.lock().unwrap().bank(10);
    //show details of the bak object in current state
    bank.lock().unwrap().print_account();
    //ID of the worker to pass into functions
    let mut worker_id = 0;
    //vector to store the join handles for the created threads
    let mut handles: Vec<_> = Vec::new();
    //loop until ledger is empty
    while !ledger.lock().unwrap().is_empty(){
        //Create clone of bank object to send into closure and share with threads
        let bank_clone = bank.clone();
        //pop a ledger off vector
        let temp = ledger.lock().unwrap().pop().unwrap();
        let handle = thread::spawn(move || {
            //deposit
            if temp.mode == 0 {
                bank_clone.lock().unwrap().deposit(worker_id, temp.ledger_id, temp.from, temp.amount);
            }
            //withdraw
            else if temp.mode == 1 {
                bank_clone.lock().unwrap().withdraw(worker_id, temp.ledger_id, temp.from, temp.amount);
            } 
            //transfer
            else {
                bank_clone.lock().unwrap().transfer(worker_id, temp.ledger_id, temp.from, temp.to, temp.amount);
            }
        });
        //increment worker id for next thread
        worker_id+=1;
        //check if worker id exceeds specified number, reset the id if true
        if worker_id >= num_workers{
            worker_id = 0;
        }
        handles.push(handle);
    }
    //join all threads
    for handle in handles {
        handle.join().unwrap();
    }
    bank.lock().unwrap().print_account();
}

/// Reads from file line by line
///
/// # Arguments
///
/// * 'filename' - the name of the file to be parsed
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Parse a ledger file and store each line into a vector
///
/// # Arguments
///
/// * 'ledger' - the vector storing all the ledgers
/// * 'filename' - the name of the file to be parsed
fn load_ledger(ledger: &mut Vec<Ledger>, filename: String){
    let mut ledger_id = 0;
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            //get string
            if let Ok(ip) = line {
                //tokenize string
                let parts = ip.split(" ");
                //store tokenized strings in vector
                let collection = parts.collect::<Vec<&str>>();
                //create ledger
                let l = Ledger {
                    from:collection[0].parse().unwrap(),
                    to:collection[1].parse().unwrap(),
                    amount:collection[2].parse().unwrap(),
                    mode:collection[3].parse().unwrap(),
                    ledger_id,
                };
                //push onto ledger vector
                ledger.push(l);
                ledger_id+=1;
            }
        }
    }
}
