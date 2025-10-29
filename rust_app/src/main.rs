#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    // creo l'oggetto 'heroes'
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent"); // aggiungo la I° coppia Key-Value
    heroes.insert("Batman", "Bruce Wayne"); // aggiungo la II°
    heroes.insert("The Flash", "Barry Allen"); // aggiungo la III°

    // inizio loop
    for (k, v) in heroes.iter() {
        // itera prendendo la coppia 'k' (key), v (value)
        println!("{} = {}", k, v); // e la stampa come ad esempio "Superman = Clark Kent"
    }

    // stampa la lunghezza dell'oggetto hashmap (che ha 3 coppie valore)
    println!("Length : {}", heroes.len()); // stampa : "Length : 3"

    let mut the_batman = None;
    // check sulla key che contiene "Batman" → prende &"Batman" perché viene fatta la get sul riferimento
    if heroes.contains_key(&"Batman") {
        the_batman = heroes.get(&"Batman"); // salvo il valore della chiave in 'the_batman'
    }

    // switch/case sulla variabile 'the_batman'
    match the_batman {
        // se presente la chiave : &"Batman"
        Some(x) => println!("Batman is on the hero list"),
        // se non presente la chiave : &"Batman"
        None => println!("Batman is not in the hero list"),
    }
}
