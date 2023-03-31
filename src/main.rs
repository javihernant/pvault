use pvault::account::Account;
fn main() {
    let account = Account::access().unwrap();
    
    println!("{}",account.data());
}
