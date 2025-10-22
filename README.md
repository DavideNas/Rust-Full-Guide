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

**Errori da evitare**
Se assegno ad una variabile un tipo byte col valore massimo supportato, devo essere sicuro di non superarlo.
Qua un esempio pratico (da NON fare):

```rs
let x: i8 = 127;    // i8 → -128 ... 127

// ERRORE
let y = x + 1; // panic in debug, overflow
```

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

---

# Lifetime delle variabili

In Rust ogni variabile è bindata alle regole del linguaggio.  
Queste sono specifiche direttive di allocazione della memoria in fase di assegnazione e valgono per ciascuna variabile.
In sostanza ci sono varie restrizioni/convenzioni che definiscono il ciclo/vita e la gestione delle allocazioni:

- Dichiarazione e mutabilità
- Shadowing
- Ownership
- Borrowing
- Heap vs Stack
- Pattern di gestione comuni per heap e DI
- convenzioni di nome

## Dichiarazione e mutabilità

Queste regole offrono una miglior sicurezza in fase di gestione delle variabili. Nello specifico esistono 2 tipologie di variabili.

- variabili immutabili
- variabili mutabili

```rs
let x = 5;      // immutabile (definita una sola volta)
let mut y = 10; // mutabile (il valore può essere modificato)
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
```

- posso definire variabili con lo stesso nome nello stesso scope
- utile per trasformazioni senza dichiarare nuove variabili
- non collegato al borrow checker

## Ownership

La ownership (proprietà) è uno dei punti cardine di rust.  
Questa permette di allocare il valore della variabile una volta e di spostarne il proprietario su richiesta.  
Di conseguenza:

- ogni valore ha un solo owner
- la memoria effettua un drop automatico quando l'owner precedente esce dallo scope
- l'ownership può essere
  - spostata (move)
  - prestata (borrow)

I tipi di variabili che sono affetti da Ownership sono :

- **String**,
- **Vec<T>**,
- **HashMap<K, V>** (o **HashSet<T>**),
- **Box<T>**,
- **File** (o **Mutex<T>**),
- tuple con almeno un tipo "NON COPY" come **(i32, String)**, **(Vec<i32>, bool)**

## Move Ownership

Lo spostamento di un contenuto elimina il contenuto precedente, questo perché in fase si assegnazione la variabile non viene copiata ma riassegnata
E' un po' come se invece che una variabile definissi il puntatore diretto alla memoria del contenuto, come mostrato nell'esempio:

```rs
let s1 = String::form("hello");
let s2 = s1;          // ownership spostata a s2
println!("{}", s1);   // ERRORE
```

> In questo caso la stringa "hello" che apparteneva ad 's1' non può avere validità in print perché 's2' ha preso il suo posto

## Borrowing Ownership

Il Borrowing non è altro che un "puntatore temporaneo" alla variabile usato per eventuali modifiche o utilizzi dislocati.
Il prestito come nel caso delle variabili può essere:

- mutabile (un solo prestito)
- immutabile

Esempio di **Borrowing mutabile**:

```rs
fn add_exclamation(s: &mut String) {
    s.push('!');      // modifica del contenuto
}

main() {
    let mut s1 = String::from("hello");
    add_exclamation(&mut s1); // borrow mutabile (concesso 1 per variabile)

    println!("{}", s1); // ok, s1 è stato modificato
}
```

Esempio di **Borrowing immutabile**:

```rs
fn print_string(s: &String) {
    println!("{}", s);
}

fn main() {
    let s1 = String::from("hello");
    print_string(&s1); // borrow immutabile (sfruttato all'interno della funzione)

    println!("{}", s1); // ok, s1 mantiene ownership
}

// oppure (esempio + semplice)
let s1 = String::from("hello");
let r = &s1;      // borrow immutabile
```

## Heap vs Stack

Qua le cose si fanno più complicate poiché non si tratta di tipologie di dichiarazione ma di definizione di aree di competenza.  
Quindi la distinzione tra Heap e Stack sta proprio nel capire DOVE deve essere usata una variabile e che visibilità deve avere.  
Questo è uno dei punti cardine per capire Rust poiché definisce con precisione la variabile e le sue proprietà di manipolazione.

Qua quindi si possono distinguere 2 tipologie di variabili:

- variabili "stack" : quelle con tipi di dimensione nota al compilatore come **i32**, **bool**, **char**

```rs
let x = 5;   // i32, piccolo e copiabile
let y = x;   // copy, x resta valido
```

- variabili "heap" : quelle con tipi dinamici come **String**, **Vec**

```rs
let s1 = String::from("hello"); // String allocata sul heap
let s2 = s1;                     // move, s1 perde ownership
```

## Pattern per la gestione heap e DI

Alcuni pattern come il DI in Rust sono possibili grazie all'integrazione di tipi specifici come i **Box**

```rs
fn create_logger() -> Box<dyn Logger> { Box::new(ConsoleLogger) }
```

Un esempio di tipi di dati usati per Heap e DI

- **Box<T>** → ownership unica di un dato heap.
- **Rc<T>** / **Arc<T>** → riferimento condiviso (single-thread o multithread).
- **Provider functions** → creare e restituire dati in modo sicuro e deterministico.

## Convenzioni di nome

Variabili normali → snake*case: `let my_var = 5;`
Costanti → UPPER_SNAKE_CASE: `const MAX_SIZE: usize = 100;`
Variabili con * iniziale → “ignora il warning se non usata”: `let _unused = 10;`

---

## Espressioni (IF)

Prendiamo in considerazione questo snippet di codice per individuare i compleanni importanti:

```rs
fn main() {
  let age = 8;
  if (age >= 1) && (age <= 18) {
    println("Important Birthday");
  } else if (age == 21) || (age == 50) {
    println("Important Birthday");
  } else if age >= 65 {
    println("Important Birthday");
  } else {
    println("Not Important Birthday");
  }
}
```

**N.B.** rcordati che i numeri non specificati vengono automaticamente istanziati come **i32**

## Espressioni (Ternaty Op)

Prendiamo adesso un caso tipo, per il check dell'tente votante (età >= 18)

```rs
fn main() {
  let mut my_age = 47;
  let can_vote = if my_age >= 18 {
    true
  } else {
    false
  }
  println!("Can Vote : {}", can_vote);
}
```

**N.B.** qua si tratta di programmazione funzionale (simile a _Scala_) poiché il risultato dell'espressione IF viene salvata direttamente in una variabile.

## Espressioni (Match)

Se prendiamo sempre l'esempio dei compleanni posso definire una serie di condizioni in maniera più elegante

```rs
fn main () {
  let age2 = 8;
  match age2 {
    1..=18 => println!("Important Birthday"),         // da 1 a 18 (compreso)
    21 | 50 => println!("Important Birthday"),        // il 21° o 50°
    65..=i32::MAX => println!("Important Birthday"),  // dal 65° ad il massimo supportato dal vartype → i32
    _ => println!("Not Important Birthday"),          // tutte le altre casistiche (come il default nello switch/case)
  };
}
```

Questa non è altro che una trasposizione dello switch/case in linguaggi più comuni con qualche caratteristica condizionale in più.  
Infatti è possibile integrare delle funzioni di confronto più evolute.

Nel caso seguente viene fatto un controllo sull'età minima per votare.  
Quì a differenza si usa un match più deduttivo dato dalle funzioni in **Ordering** (**_Less_** | **_Greater_** | **_Equal_**)

```rs
use std::cmp::Ordering;       // import necessario

fn main () {
  let my_age = 18;
  let voting_age = 18;
  match my_age.cmp(&voting_age) {
    Ordering::Less => println!("Can't Vote"),                     // se l'età è inferiore ai 18
    Ordering::Greater => println!("Can Vote"),                    // se l'età è superiore ai 18
    Ordering::Equal => println!("You gained the right to vote"),  // se hai esattamente 18 anni
  };
}
```

---

## Array e Loops

Prendiamo un esempio come un array di numeri consecutivi, voglio quindi effettuare delle operazioni sugli elementi contenuti come nell'esempio:

```rs
fn main() {
  let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];

  // stampe di valori
  println!("1st : {}", arr_1[0]);       // 1st : 1 (legge il primo elemento)
  println!("Length : {}", arr_1.len()); // Lenght : 9 (legge la lunghezza dell'array)
```

### Loop

Nell'esempio seguente il loop effettua un 'salto' dei valori che corrispondono ai numeri pari (se diviso 2 = 0 allora è pari)
di conseguenza stampa solo i numeri dispari.

```rs
  let mut loop_idx = 0;
  loop {
    if arr_1[loop_idx] % 2 == 0 {   // considero tutti gli indici divisibili per 2 (quelli pari: 0, 2, 4, 6, 8...)
      loop_idx +=1;                 // aggiunge 1 all'indirizzo e ricicla il loop
      continue;
    }
    if arr_1[loop_idx] == 9 {     // condizione che si verifica se l'indice è 9
      break;                      // esce dal loop
    }
    println!("Val : {}", arr_1[loop_idx]);   // stampa il contenuto dell'array solo quando non fa il 'continue' nella condizione precedente
    loop_idx +=1;                 // aggiunge 1 all'indice prima di proseguire
  }
}
```

### While

Altro ciclo molto importante è il **while** (molto usato su linguaggi più comuni)

```rs
fn main() {
  let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  let mut loop_idx = 0;
  while loop_idx < arr_2.len() {            // cicla fintanto che l'indice non arriva alla lunghezza dell'array
    println!("Arr : {}", arr_2[loop_idx]);    //  stampa il valore indicato nell'array
    loop_idx +=1;                 // incrementa l'indice
  }
}
```

### For

Il for ha più le sembianze di un classico foreach dove :

- il valore **val** prende in carico il contenuto di ciascuna cella in arr_2
- che cicla ciascun elemento grazie al metodo **.iter()**

```rs
fn main() {
  let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  let mut loop_idx = 0;
  for val in arr_2.iter() {
    println!("Val : {}", val);
  }
}
```

---

## Tuple

Le tuple in Rust possono gestire dati di tipo diverso, sono quindi molto versatili e manipolabili.
In particolare la forza di Rust sta nel gestire tuple con:

- strutture di dati complesse
- garanzie di velocità e sicurezza

```rs
// tupla con 3 tipi diversi
let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);

// stampo il contenuto alla posizione 1 (→ Derek)
println!("Name : {}", my_tuple.1);

// spread della tupla su 3 valori
let(v1, v2, v3) = my_tuple;

// stampa del valore v1 (→ 47)
println!("Age : {}", v1);
```

---

Prosegui dal minuto 41.03 dek video YT
https://www.youtube.com/watch?v=ygL_xcavzQ4
