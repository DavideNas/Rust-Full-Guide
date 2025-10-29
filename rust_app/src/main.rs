#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

use std::thread;
use std::time::Duration;

fn main() {
    // creo una struttura 'Bank'
    pub struct Bank {
        // ... con un solo tipo di dato
        balance: f32,
    }
    // metodo per il prelievo
    fn withdraw(the_bank: &mut Bank, amt: f32) {
        the_bank.balance -= amt;
        println!("Balance : {}", the_bank.balance);
    }
    let mut bank = Bank { balance: 100.0 };
    withdraw(&mut bank, 5.00);
    println!("Balance : {}", bank.balance);
    fn customer(the_bank: &mut Bank) {
        withdraw(the_bank, 5.00);
    }
    thread::spawn(|| {
        let mut bank = Bank { balance: 100.0 };
        customer(&mut bank)
    })
    .join()
    .unwrap();
}
