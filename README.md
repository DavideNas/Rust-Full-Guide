# Rust-Full-Guide

## Intro

Perché scegliere Rust?
Rust è un linguaggio di programmazione che è studiato per offrire:

- Alte prestazioni
- Assenza si Garbage Collection
- Programmazione Concurrente
- Facile debug degli errori

Per iniziare visita il [sito ufficiale](https://rust-lang.org/learn/get-started/) di Rust e segui le istruzioni di installazione.

quindi verifica l'installazione da prompt

```sh
cargo --version
```

---

## Creare un progetto Rust

Puoi creare una nuova app in Rust scrivendo da bash

```sh
cargo new rust_app
```

Appena creata l'app verrà aggiunta una cartella col nome assegnatogli ed al suo interno si avranno:

- un file `Cargo.toml` per la gestione dei packages di progetto
- una cartella **/src** con un file `main.rs`

> Se mi servisse un package particolare posso ottenerlo dal sito https://crates.io/

Alcuni comandi utili

```sh
cargo update            # per installare le dipendenze quando necessarie
cargo add package_name  # per aggiungere un pacchetto
```

apri quindi il file main.rs ed aggiungi il codice di esempio

```rs
#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

// qua apparirà un pulsante Run|Debug
fn main() {
    println!("Enter your name");                    // CTA per scrivere il nome
    let mut name = String::new();                   // variabile che salva l'input (il nome)
    let greeting = "Nice to meet you";               // messaggio di benvenuto
    io::stdin().read_line(&mut name)                // comando di cattura stringa (legge il nome quando INVIO)
        .expect("Didn't Receive Input");            // messaggio di errore se non scivo nulla
    println!("Hello, {}! {}", name.trim_end(), greeting);       // messaggio di benvenuto (stampa nome e messaggio)
}
```

Quindi avvia il progetto semplicemente cliccando sul piccolo bottone play che appare sulla funzione main

In questo esempio:

- Si avvia mostrando un messaggio
- prende in input una stringa "name"
- Quando preme invio usa la libreria che
  - Legge il nome inserito
  - Mostra un messaggio di errore se la stringa rimane vuota
- chiama la macro "println" che stampa 2 occorrenze indicate con **{}**
  - la prima associata al nome utente (inserito prima e salvato in name)
  - la seconda un messaggio di benvenuto

Alcune precisazioni sintattiche:

- la libreria `rand` va installata con `cargo add rand`
- `println` è una macro, le macro in Rust vengono chiamate col ! alla fine
- `let mut name` indica che una variabile può cambiare in runtime , `let greeting` la lascia inizializzare una volta
- le **mutable** sono lette con & davanti al tipo del nome quindi `&mut name` o `&mut age`, etc...
- le stringhe con valori parametrizzati sono concatenate con placeholder definiti con **{}** e poi associati ai valori di seguito (in ordine cronologico)

---

## Variabili

In rust puoi definire una variabile in diversi modi

```rs
// costanti
const ONE_MIL: u32 = 1_000_000;     // intero 32 bit unsigned (un milione scritto con _ ogni 3 cifre)
const PI: f32 = 3.141592;           // float 32 che indica numero con virgola (PI Greco)

// variabili
let age = "42";                     // variabile NON mutabile (si assegna una volta)
let mut year = 2024;                // mutabile in runtime (posso aggiornare l'anno al 2025 ad  esempio)

// interi positivi
// variabili Unsigned Integer: u8, u16, u32, u64, u128, usize
let pointer:u8 = 1;

// numeri interi (anche negativi)
// variabili Signed Integer: i8, i16, i32, i64, i128, isize
let range:i16 = 300;

// float numbers
// variabili con virgola: f32 , f64 (assigned as default → let z = 3.14)
let pi:f32 = 3.14;

// booleans
let is_valid:bool = true;
let _is_valid_format:bool = false;      // se non istanziata la booleana non da warning ( con _ all'inizio : "Rust, ignora il suo utilizzo" )

// char
let my_grade:char = 'A';        // per singolo carattere
```

il numero 8 indica 1 byte
16 → 2 byte
32 → 4 byte
64 → 8 byte
128 → 16 byte (raramente usato)
size → si adatta alla grandezza dell'architettura in uso 32 o 64 bit

i ed u indicano
i → signed : range di numeri negativi e positivi
u → unsigned : range di numeri SOLO positivo

gli indici usati per ciclare negli arrai DEVONO essere `unsigned`

## Operazioni

```rs
// parsing
let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number ");      // parsing & shadowing

// add
age = age + 1;          // aggiungo 1 alla mia età
year += 1;              // aggiungo 1 all'anno
year = year + 1;        // aggiungo ancora 1 all'anno

// ERRORE (add)
year ++;    // non valido in Rust

// altre operazioni
let num_1 = 6;
let num_2 = 3;
println!("6 + 3 = {}", num_1 + num_2);      // addizione             6 + 3 = 9
println!("6 - 3 = {}", num_1 - num_2);      // sottrazione           6 - 3 = 3
println!("6 * 3 = {}", num_1 * num_2);      // moltiplicazione       6 * 3 = 18
println!("6 / 3 = {}", num_1 / num_2);      // divisione             6 / 3 = 2
println!("6 % 3 = {}", num_1 % num_2);      // modulo                6 % 3 = 0

// random values , richiede → use rand::Rng;
let random_num = rand::thread_rng().gen_range(1..101);      // genera valore random : da 1 a 100 (101 non incluso)
println!("Random: {}", random_num);
```

## Shadowing

In Rust è possibile assegnare un nome già in uso ad una nuova variabile. Quando succede il sistema sposta il puntatore sull'ultima occorrenza dichiarata.

Se per dire scrivo

```rs
let my_books = 43
let my_books = 55       // shadowing implicito (non modifico la mutabilità)
// Il sistema capirà che i miei libri sono 55 e non c'è modo di recuperare il valore precedente se prima non lo salvo in uno stack separato.

let my_games = 15
let mut my_games = my_games + 10;   // shadowing esplicito (il nuovo my_games è adesso mutabile)
// Qua il sistema crea una variabile ex novo my_games che conterrà il valore precedente + 10

let x: i8 = 127;    // i8 → -128 ... 127

// ERRORE
let y = x + 1; // panic in debug, overflow
```

## Espressioni (IF)

Prosegui dal minuto 24.09 dek video YT
https://www.youtube.com/watch?v=ygL_xcavzQ4
