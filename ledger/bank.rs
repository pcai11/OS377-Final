use std::{sync::{Arc, Mutex}};
pub struct Account {
    account_id: Arc<Mutex<i32>>,
    balance: Arc<Mutex<i32>>,
}

pub struct Bank{
    pub num:i32,
    pub num_succ:Arc<Mutex<i32>>,
    pub num_fail:Arc<Mutex<i32>>,
    pub accounts: Vec<Account>,
}

impl Bank{
    /// Creates n individual accounts with thread-safe fields 
    /// that are then pushed onto the accounts Vector
    ///
    /// # Arguments
    ///
    /// * 'self' - reference to this bank
    /// * `n` - the number of accounts to create
    pub fn bank(&mut self, n:i32){
        //create n accounts
        for i in 0..n{
            let a = Account{
                account_id: Arc::new(Mutex::new(i)),
                balance: Arc::new(Mutex::new(0)),
            };
            //store in accounts vector
            self.accounts.push(a);
        }
    }

    /// Deposits money into the specified account and 
    /// returns a 0 indicating success or -1 for failure
    ///
    /// # Arguments
    ///
    /// * 'self' - reference to this bank
    /// * 'workerID' - the ID of the worker (thread)
    /// * 'ledgerID' - the ID of the ledger entry
    /// * 'accountID' - the account ID to deposit 
    /// * 'amount' - the amount to deposit
    pub fn deposit(&mut self, worker_id:i32, ledger_id:i32, account_id:i32, amount:i32)-> i32{
        //increase account balance by ammount
        *self.accounts[account_id as usize].balance.lock().unwrap() += amount;
        //output message and record success
        self.record_succ(format!("Worker {} completed ledger {}: deposit {} into account {}", worker_id, ledger_id, amount, account_id));
        0
    }

    /// Withdraws money from the specified account and 
    /// returns a 0 indicating success or -1 for failure
    ///
    /// # Arguments
    ///
    /// * 'self' - reference to this bank
    /// * 'workerID' - the ID of the worker (thread)
    /// * 'ledgerID' - the ID of the ledger entry
    /// * 'accountID' - the account ID to withdraw 
    /// * 'amount' - the amount to withdraw
    pub fn withdraw(&mut self, worker_id:i32, ledger_id:i32, account_id:i32, amount:i32)-> i32{
        let mut success = -1;
        //check if balance > amount
        if amount <= *self.accounts[account_id as usize].balance.lock().unwrap(){
            *self.accounts[account_id as usize].balance.lock().unwrap() -= amount;
            self.record_succ(format!("Worker {} completed ledger {}: withdraw {} from account {}", worker_id, ledger_id, amount, account_id));
            success = 0;
        } else{
            self.record_fail(format!("Worker {} failed to complete ledger {}: withdraw {} from account {}", worker_id, ledger_id, amount, account_id));
        }
        success
    }

    /// Transfers money from the specified account to another and 
    /// returns a 0 indicating success or -1 for failure
    ///
    /// # Arguments
    ///
    /// * 'self' - reference to this bank
    /// * 'workerID' - the ID of the worker (thread)
    /// * 'ledgerID' - the ID of the ledger entry
    /// * 'srcID' - the account ID to transfer out of
    /// * 'destID' - the account ID to transfer into  
    /// * 'amount' - the amount to transfer
    pub fn transfer(&mut self, worker_id:i32, ledger_id:i32, src_id:i32, dest_id:i32, amount:i32)-> i32{
        let mut success = -1;
        //check if balance > amount
        if amount <= *self.accounts[src_id as usize].balance.lock().unwrap(){
            *self.accounts[src_id as usize].balance.lock().unwrap() -= amount;
            *self.accounts[dest_id as usize].balance.lock().unwrap() += amount;
            self.record_succ(format!("Worker {} completed ledger {}: transfer {} from account {} to account {}", worker_id, ledger_id, amount, src_id, dest_id));
            success = 0;
        }else{
            self.record_fail(format!("Worker {} failed to complete ledger {}: transfer {} from account {} to account {}", worker_id, ledger_id, amount, src_id, dest_id));
        }
        success
    }

    /// Prints the account information for this Bank and
    /// the number of success and failure
    ///
    /// # Arguments
    ///
    /// * 'self' - reference to this bank
    pub fn print_account(&mut self){
        //print each account
        for i in self.accounts.iter(){
            println!("ID# {} | {}", *i.account_id.lock().unwrap(), *i.balance.lock().unwrap());
        }
        println!("Success: {} Fails: {}", *self.num_succ.lock().unwrap(), *self.num_fail.lock().unwrap());
    }

    /// Helper function to increment 'num_succ' and print
    /// the message
    ///
    /// # Arguments
    ///
    /// * 'self' - reference to this bank
    /// * 'message' - message to be printed
    pub fn record_succ(&mut self, message: String){
        println!("{}", message);
        *self.num_succ.lock().unwrap()+=1;
    }

    /// Helper function to increment 'num_fail' and print
    /// the message
    ///
    /// # Arguments
    ///
    /// * 'self' - reference to this bank
    /// * 'message' - message to be printed
    pub fn record_fail(&mut self, message: String){
        println!("{}", message);
        *self.num_fail.lock().unwrap()+=1;
    }
}