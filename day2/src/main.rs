use std::collections::HashMap;
struct Token {
    name: String,
    symbol: String,
    decimals: i32,
    balances: HashMap<String, i32>,
    allowances: HashMap<(String,String), i32>,
}

impl Token {
    // 构造函数
    fn new(name: String, symbol: String, decimals: i32) -> Token {
        Token {
            name,
            symbol,
            decimals,
            HashMap,
            HashMap::new(),
        }
    }

    fn name(&self) -> String {
        self.name
    }

    fn symbol(&self) -> String {
        self.symbol
    }

    fn decimals(&self) -> i32 {
        self.decimals
    }

    fn mint(&self,to: String, amount: i32) {
        self.balances.insert(to, amount);
    }

    fn burn(from: String, amount: i32) {
        self.balances.(to, amount);
    }

    fn transfer(to: String, amount: i32) {

    }

    fn transferFrom(from: String, to: String, amount: i32) {

    }

    fn approve(&self,owner: String, spender: String, amount: i32) {
        self.allowances.insert((owner, spender), amount);
    }

    fn allowance(&self,from: String, to: String) -> i32 {
        self.allowances.get((from, to))
    }
}

fn main() {
    let mut _balance = 1;
    let mut _approve = 0;
    _balance += 1;
    print!("balance: {}", _balance);
}

fn balanceOf(address: String) -> i32 {
    if owners.contains_key(address) {
        let owner = owners[address];
        return owner.balance;
    }
    return 0;
}

fn balanceOf(address: String) -> i32 {
    if owners.contains_key(address) {
        let owner = owners[address];
        return owner.balance;
    }
    return 0;
}
