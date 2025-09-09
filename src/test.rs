use std::cell::{Ref as CellRef, RefCell};

// 定义银行结构体
struct Bank {
    // 使用RefCell存储余额，因为余额是内部可变的
    balance: RefCell<i32>,
}

impl Bank {
    fn new() -> Bank {
        Bank {
            balance: RefCell::new(0),
        }
    }

    // 存款
    fn deposit(&self, amount: i32) {
        let mut balance = self.balance.borrow_mut();
        *balance += amount;
    }

    // 取款
    fn withdraw(&self, amount: i32) -> bool {
        // 获取内部可变引用
        let mut balance = self.balance.borrow_mut();
        if *balance >= amount {
            *balance -= amount;
            true
        } else {
            false
        }
    }
}

pub fn test() {
    let bank: Bank = Bank::new();
    bank.deposit(100);
    {
        let ss: CellRef<'_, i32> = bank.balance.borrow();
        println!("Current balance: {}={}", ss, *ss);
    }
    // 取款50元，余额应该是50元
    assert!(bank.withdraw(50));
    assert_eq!(*bank.balance.borrow(), 50);
}
