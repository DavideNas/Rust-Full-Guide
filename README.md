# Rust-Full-Guide

Guida basata sul corso YT : https://www.youtube.com/watch?v=ygL_xcavzQ4

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

## Strings

Le stringhe in Rust sono istanziate come oggetti. Hanno caratteristiche simili agli array, ma come array di caratteri.

Prendiamo come esempio il codice sotto:

```rs
fn main() {
  let mut st1 = String::new();            // creo una nuova stringa
  st1.push('A');                          // aggiungo un carattere 'A' con .push
  st1.push_str(" word");                  // aggiungo alla fine una parola ' word' con .push_str

  // start loop
  for word in st1.split_whitespace() {    // splitto la stringa in base ai whitespace " "
    println!("{}", word);                 // stampo ciascun elemento splittato
  }
  let st2 = st1.replace("A", "Another");  // creo una nuova stringa sostituendo alla prima "A" con "Another"
  println!("{}", st2);                    // stampo la nuova stringa "Another word"
}
```

Nell'esempio si può notare come l'oggetto string

- si comporti come un array (.push)
- con proprietà e metodi ah hoc per la manipolazione del contenuto (.replace, .split_whitespace)

La variabile 'word' è una variabile di tipo **&str** che consente di puntare alla stringa e visualizzarne il contenuto.

Quindi :

- **String** è il tipo "stringa"
- **&str** è il tipo "pointer to string"

Altro esempio con le stringhe:

```rs
fn main() {
  let st3 = String::from("x r t b h k k a m c");    // stringa di caratteri separati da whitespace " "
  let mut v2 Vec<char> = st3.chars().collect();     // vettore di 'char' che estrae i caratteri automaticamente col metodo .char().collect()

  // riordinamento
  v1.sort();                // sistema i carateri interni a v1 in ordine alfabetico

  // rimuove duplicati
  v1.dedup();               // elimina i duplicati (in questo caso un carattere 'k')

  // start loop
  for char in v1 {          // cicla ogni carattere presente in 'v1'
    println!("{}", char);   // stampa ogni elemento preso da 'v1'
  }

  let st4: &str = "A welcome message";          // creo una variabile di tipo "pointer to string"
  let mut st5: String = st4.to_string();        // creo una mutabile come stringa che prende il valore "A welcome message" (ovvero la stringa puntata da st4)
  println!("{}", st5);                          // stampa la stringa convertita da &str "A welcome message"

  // bytes
  let byte_arr1 = st5.as_bytes();               // 'byte_arr1' è di tipo '&[u8]' ovvero uno slice di bytes

  // slice
  let st6 = &st5[0..6];                         // una slice 'st6' dei primi 6 elementi caratteri in 'st5'

  println!("String length : {}", st6.len());    // stampo la lunghezza della slice "A welc" → "String length : 6"

  st5.clear();                                  // cancello st5
  let st6 = String::from("How are");            // creo st6 → "How are"
  let st7 = String::from(" you?");              // creo st7 → " you?"
  let st8 = st6 + &st7;                         // concat di 2 stringhe 'st6' e 'st7'

  // start loop
  for char in st8.bytes() {                     // prende ciascun carattere nella stringa st8 leggendolo come byte
    println!("{}", char);                       // stampa il carattere in codice decimale (da 0 a 255) → convertito come binario puro
    // output completo [72, 111, 119, 32, 97, 114, 101, 32, 121, 111, 117, 63]
  }
}
```

Si può notare che:

- in **st5** il valore aggiunto deriva da una variabile **&str** deve essere quindi convertito in **String** col metodo **.to_string()**
- **byte_arr1** prende il contenuto di st5 e lo converte in bytes _[65, 32, 119, 101, 108, 99, 111, 109, 101, 32, 109, 101, 115, 115, 97, 103, 101]_
  - questo avviene grazie al metodo **.as_bytes()**
  - il tipo **&[u8]** è implicito (corrispondente al tipo byte), immutabile (non modificabile tramite st6), condiviso
- anche in **st6** viene creato un riferimento "pointer to string" implicito, immutabile, condiviso
- **st8** è il risultato del concat tra le stringhe **st6** ed **st7**, in Rust questa operazione:
  - fa il move della prima stringa ovvero trasferisce il contenuto di **st6** in **st8**.
  - prende SOLO il riferimento della seconda stringa **&st7** dove legge il contenuto per agganciarlo al primo.
- nel loop chiama il metodo**.bytes()** che converte ciascun carattere come byte
  - se sono caratteri complessi come µ, ♂, «, ♦, 😄, il valore potrebbe richiedere 2 o anche 4 bytes rappresentati come array di numeri
    - µ → [194, 181] = 2 bytes
    - ♂ → [226, 178, 178] = 3 bytes
    - « → [194, 171] = 2 bytes
    - ♦ → [226, 153, 166] = 3 bytes
    - 😄 → [240, 159, 152, 132] = 4 bytes
  - tutti i caratteri (moji comprese) non superano mai i 4 bytes

In generale Rust interpreta i caratteri leggendo e raggruppando i byte successivi in base al valore del primo.

Qua una tabella di riferimento per i caratteri UTF-8 multibyte:

| Tipo byte         | Decimal range |
| ----------------- | ------------- |
| ASCII (1 byte)    | 0..127        |
| Continuation byte | 128..191      |
| Primo byte 2-byte | 192..223      |
| Primo byte 3-byte | 224..239      |
| Primo byte 4-byte | 240..247      |

---

## Casting

Il casting è quella cosa che permette una conversione di tipo tra diverse tipologie di variabili.

Prendiamo come esempio questo codice:

```rs
let int_u8: u8 = 5;           // u8 (byte) = 5
let int2_u8: u8 = 4;          // u8 (byte) = 4
let int3_u32: u32 = (int_8 as u32) + (int2_8 as u32);       // u32 con casting a u32 di 'int_u8' ed 'int2_u8'
```

Qua il nuovo numero necessita di una conversione degli operatori per validarli

---

## Enum

Gli enum sono utili per definire gruppi di variabili con valori specifici

Esempio di enum coi giorni della settimana

```rs
fn main() {
  enum Day {      // creo l'enum
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
  }


  impl Day {
    fn is_weekend(&self) -> bool{         // '&self' è l'argomento implicito 'today'
      // switch/case su 'self'
      match self {
        Day::Saturday | Day::Sunday => true,    // se 'self' è 'Saturday' o 'Sunday'
        _ => false                              // gli altri giorni
      }
    }
  }

  let today:Day = Day::Monday;

  // switch/case su 'today'
  match today {
    Day::Monday => println!("Everyone hates monday"),
    Day::Tuesday => println!("Donut day"),
    Day::Wednesday => println!("Hump day"),
    Day::Thursday => println!("Pay day"),
    Day::Friday => println!("Almost Weekend"),
    Day::Saturday => println!("Weekend"),
    Day::Sunday => println!("Weekend"),
  }

  println!("Is today the weekend {}", today.is_weekend());    // chiamo il metodo .is_weekend
}
```

Dall'esempio si posono notare alcune cose come:

- il metodo **.is_weekend** è legato all'oggetto 'today' di tipo **Day**
- la funzione `fn is_weekend(&self) -> bool` ottiene come parametro l'oggetto associato al metodo (today = self)
- **impl Day** serve ad ottenere le proprietà di metodo sulle istanze di **Day**
  - da qui posso fare **today.is_weekend()**
  - Quindi Rust capisce che **is_weekend** è un comportamento legato al tipo **Day**
  - il nome **_'self'_** è una variabile Rust che ha validità SOLO all'interno dei metodi **impl**

In alternativa a **impl Day** avrei potuto scrivere così:

```rs
// today è un valore passato ed associato ad una variabile interna 'day'
fn is_weekend(day: &Day) -> bool {
  // switch/case su 'day'
  match day {
        Day::Saturday | Day::Sunday => true,
        _ => false,
    }
}

// passo today come valore alla funzione is_weekend
println!("Is today the weekend {}", is_weekend(&today));
```

Mettendo a confronto le 2 chiamate si può dedurre che :

- la prima è definita come _dot syntax_
  - il parametro letto **self** viene capito come valore di **impl** (ovvero **Day**)
- la seconda chiamata è quella normale
  - qua **self** non può essere usato deve essere esplicitamente includo il tipo Day
  - come mostrato `is_weekend(day: &Day)`

**N.B.**  
'self' pò essere usato SOLO dentro un blocco **impl**

---

## Vectors

I vettori in Rust sono come array che possono crescere SOLO se mutabili e memorizzano dati di un solo tipo.

```rs
fn main() {
  let vec1: Vec<i32> = Vec::new();  // creo un vettore vuoto
  let mut vec2 = vec![1, 2, 3, 4];  // creo un vettore mutabile di numeri (i32 è impostaato come tipo default)
  vec2.push(5);                     // aggiungo in coda a vec2 un numero "5"
  println!("1st : {}", vec2[0]);    // stampa → 1st : 1
  let second: &i32 = &vec2[1];      // non copio vec2[1] ma restituisco un riferimento (salvato in 'second')

  // switch/case sul 2° elemento di 'vec2'
  match vec2.get(1) {
    Some(second) => println!("2nd : {}", second),   //  come Option<T> o un valore generico non definito
    None => println!("No 2nd value"),               //  indica assenza di valore
  }

  // start loop in vec2
  for i in &mut vec2 {
    *i *= 2;                  // moltiplico ciascun elemento *2
  }

  // start loop in vec2
  for i in &vec2 {
    println!("{}", i);        // stampo ciascun valore dell'array (mutato per il loop precedente)
  }

  println!("Vec Length {}", vec2.len());      // stampa → Vec Length 5
  println!("Pop : {:?}", vec2.pop())           // stampa → Pop : Some(10)
}
```

**N.B.**

- in `let second: &i32 = &vec2[1];` Rust può dedurre il tipo perché lo capisce dal valore restituito
- Il parametro `{:?}` stampa il tipo (ovvero Option<T>) che rust traduce come **Some**
- `vec2.pop()` legge l'ultimo elemento del vettore rimuovendolo da esso

---

## Le funzioni

### Semplice funzione senza parametri

```rs
fn say_hello() {        // creo una semplice funzione 'say_hello'
  println!("Hello");
}

fn main() {
  say_hello();          // chiamo la funzione sopra
}
```

### Funzione con passaggio di parametri

```rs
fn get_sum(x: i32, y: i32) {              // prende x e y come numeri (i32) passati alla chiamata
  println!("{} + {} = {}", x, y, x+y);    // stampa → 5 + 4 = 9
}

fn main() {
  get_sum(5, 4);        // chiama la funzione e gli passo 5 e 4 che sono poi importati come x e y
}
```

### Funzione con valore di ritorno

```rs
fn get_sum_2(x: i32, y: i32) -> i32 {   // qua definisco il tipo di ritorno ( -> i32) dopo la definizione dei paramemtri
  x + y     // ritorna x + y
}

fn main() {
  println!("Sum of 5 + 4 {}", get_sum_2(5, 4));      // chiama la funzione e gli passo 5 e 4 che sono poi importati come x e y
}
```

### Funzione con operazioni aggiuntive

```rs
fn get_double_sum(mut x: i32, mut y: i32) -> i32 {    // importo i numeri in variabili x e y mutabili
  x *= 2;       // raddoppio x
  y *= 2;       // raddoppio y
  x + y         // ritorna il doppio della somma
}

fn main() {
  println!("Double sum of 5 + 4 = {}", get_double_sum(5, 4));   // Stampa → Double sum of 5 + 4 = 18
}
```

**N.B.**

- le variabili x e y sono moltiplicati uno ad uno prima della somma
- i valori di return sono distinti dalla mancanza del ; alla fine
  - se mettessi solo `x + y;` la funzione non tornerebbe nulla
  - potrei però scrivere un classico `return x + y;` che risolverebbe il problema

### Funzione con ritorno miltiplo

```rs
// definisco il ritorno multiplo coi valori tramite parentesi
fn get_2(x: i32) -> (i32, i32) {      // ritorna 2 numeri : -> (i32, i32)
  return (x+1, x+2);                  // il primo numero è x+1 ; il secondo è x+2
}

fn main() {
  let (val_1, val_2) = get_2(3);      // il valore (multiplo) di ritorno è distribuito sulla struttura composta da 'val_1' e 'val_2'
  println!("Nums : {} {}", val_1, val_2);   // stampa → Nums : 4 5
}
```

### Funzione di ritorno con passaggio di lista

```rs
// importo un riferimento a '&num_list' tramite '&[i32]' salvandolo in 'list'
fn sum_list(list: &[i32]) -> i32 {
  let mut sum = 0;  // creo una mutabile 'sum'

  // itera nella lista facendo riferimento ad ogni elemento con '&val'
  for &val in list.iter() {
    sum += &val;
  }
  sum
}

fn main() {
  // vero un vettore di numeri → [i32]
  let num_list = vec![1, 2, 3, 4, 5];
  // passo a sum_list un riferimeto alla lista '&num_list'
  println!("Sum of list = {}", sum_list(&num_list));
}
```

---

## Generics

Capita che bisogna lavorare con diversi tipi di dati, questo però a livello di parametri non sempre è prevedibile.  
In Rust esiste il tipo generico **T**

```rs
use std::ops::Add;

// per il valore di ritorno devo prevedere un operazione Add
fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
  // sommo i due valori importati nella funzione
  return x + y;
}

fn main() {
  println!("5 + 4 = {}", get_sum_gen(5, 4));            // stampa → 5 + 4 = 9
  println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));    // stampa → 5.2 + 4.6 = 9.8
}
```

Come si può notare il generico T è più versatile del tipo definito i32, u16, etc...  
Ma per far capire alla funzione che si tratta di un valore valido devo definire un Bound tramite l'espressione  
`<T:Add<Output = T>>`, in questo modo comunico alla funzione (che gestisce i generics) che:

- I valori T devono supportare l'operando Add (ovvero devono "capire" il +)
- L'output è dello stesso tipo dei valori importati (se passo un double, ritorna un double)
- Add comunque necessita dell'import `use std::ops::Add;` perché è un check extra

Il bound di una funzione che gestisce i generics può essere di 2 tipi:

```rs
// I° modalità
fn f<T: Bound>(...)   // come in questo caso
// II° modalità
fn f<T>(...) where T: Bound // separando le definizioni rendendo il codice più pulito
```

Seguendo la seconda opzione l'esempio precedente sarebbe stato quindi

```rs
fn get_sum_gen<T>(x: T, y: T) -> T
  where T:Add<Output = T> {
  return x + y;
}
```

Tabella di riferimento per le lettere usate nelle Generics

| Lettera       | Uso tipico                                      | Significato convenzionale |
| ------------- | ----------------------------------------------- | ------------------------- |
| `T`           | **Type**                                        | Tipo generico "di base"   |
| `U`, `V`, `W` | Tipi aggiuntivi (secondo, terzo, quarto, ecc.)  |                           |
| `E`           | **Error** type (es. in `Result<T, E>`)          |                           |
| `K`, `V`      | **Key / Value** (es. in mappe, `HashMap<K, V>`) |                           |
| `'a`, `'b`    | **Lifetime** (non tipi, ma parametri di durata) |                           |

---

## Ownership

Per comprendere bene cosa sia la Ownership bisogna fare un focus sui tipi di memoria usata da un programma.  
Questi tipi di memoria sono:

- Stack: la memoria (detta memoria a pila) che gestisce i valori in un formato LIFO
  - Questa memoria deve avere una dimensione fissa definita
- Heap: quando si mette un dato nella memoria heap (o memoria cumulativa) serve richiedere una certa quantità di spazio.
  - L'OS trova spazio disponibile e ritorna un indirizzo diretto a quella specifica porzione di memoria.
  - Questa indirizzo viede definito come puntatore.

Ci sono poi alcune regole fondamentali da prendere in considerazione per capire la ownership (o proprietà) di un valore:

1. Ogni valore ha una variabile che è chiamata owner (proprietario)
2. Esiste un solo proprietario (un solo owner) per volta
3. Quando il proprietario "esce" dallo scope del programma il valore sparisce (Rust gestisce la deallocazione in automatico)

Per comprendere correttamente questi passaggi considera il codice seguente:

```rs
fn main() {
  let str1 = String::from("World");   // creo un oggetto stringa passando il parametro "World" al costruttore
  let str2 = str1;                    // fa il move del valore → da str1 a str2
  println!("Hello {}", str1);         // ERRORE : prestito del valore spostato str1
}
```

Questo mostra che dopo lo spostamento della stringa "World" da str1 a str2, il parametro str1 viene deallocato da Rust.  
Non è più possibile ottenere un valore direttamente da str1 perché semplicemente non esiste più (dopo il move).

Se invece facessi un clone tramite metodo `.clone` avverrebbe questo

```rs
fn main() {
  let str1 = String::from("World");
  let str2 = str1.clone();            // fa una copia esatta del valore str1 in str2 (adessso ho 2 copie)
  println!("Hello {}", str1);         // VALIDO: str1 non è stato deallocato adesso stampa "Hello World"
}
```

Per affrontare le varie manipolazioni di stringa considera questa serie di funzioni come esempio:

```rs
// I° funzione : stampa una stringa come composizione di un valore passato al suo interno
fn print_str(x: String) {
  println!("A string {}", x);     // A string 'x' → dove x è il valore passato
}

// II° funzione : stampa una stringa (come prima) e ritora il valore della stringa passata
// -> String : definisce il valore di ritorno
fn print_return_str(x: String) -> String {
  println!("A string {}", x);     // A string 'x' → dove x è il valore passato
  x                               // ritorna sempre 'x'
}

// III° funzione : importa un riferimento alla stringa che viene modificata al suo interno
// name è di tipo → "&mut String" ovvero un riferimento alla variabile passata (che può averne al massimo uno)
fn change_str(name: &mut String) {
  name.push_str(" is happy");     // modifica 'name' aggiungendo " is happy", dove 'name' è la stringa passata
  println!("Message : {}", name); // stampa "Message : Dave is happy" (Dave è il parametro importato in 'name')
}

fn main() {
  let str1 = String::from("World");

  // stampe semplici
  print_str(str1);                    // chiama la funzione
  let str3 = print_return_str(str1);  // str3 ottiene il valore di ritorno di 'print_return_str'
  println!("str3 = {}", str3);        // stampa il valore salvato in str3 → str3 = World

  // manipolazione di stringa
  // str2 è una mutabile (ovvero è permessa la manipolazione della stringa)
  let mut str2 = String::from("Dave");  // creo un oggetto stringa (mutabile) che contiene il nome "Dave"
  change_string(&mut str2);             // chiamo la funzione 'change_string' e gli passo il riferimento alla variabile 'str2'
}
```

**N.B.**

- nella terza funzione devo passare un riferimento alla variabile mutabile tramite **&mut** che ne permette la modifica
- questa variabile viene importata come **name: &mut String** che da la possibilità di prendere questo riferimento e modificarlo tramite 'name'

**IMPORTANTE** : in Rust una variabile può avere SOLO una versione mutabile alla volta (all'interno della funzione)

---

## Gli Hashmaps

Gli hashmaps in Rust sono usati per memorizzare le coppie **key:value**.  
Per usarli è necessario l'import della libreria come descritto nel codice seguente:

```rs
// faccio l'import della classe HashMap
use std::collections::HashMap;

fn main() {
  // creo l'oggetto 'heroes'
  let mut heroes = HashMap::new();
  heroes.insert("Superman", "Clark Kent");    // aggiungo la I° coppia Key-Value
  heroes.insert("Batman", "Bruce Wayne");     // aggiungo la II°
  heroes.insert("The Flash", "Barry Allen");  // aggiungo la III°

  // inizio loop
  for(k, v) in heroes.iter() {    // itera prendendo la coppia 'k' (key), v (value)
    println!("{} = {}", k, v);    // e la stampa come ad esempio "Superman = Clark Kent"
  }

  // stampa la lunghezza dell'oggetto hashmap (che ha 3 coppie valore)
  println!("Length : {}", heroes.len());      // stampa : "Length : 3"

  // check sulla key che contiene "Batman" → prende &"Batman" perché viene fatta la get sul riferimento
  if heroes.contains_key(&"Batman") {
    let the_batman = heroes.get(&"Batman"); // salvo il valore della chiave in 'the_batman'

    // switch/case sulla variabile 'the_batman'
    match the_batman {
      // se presente la chiave : &"Batman"
      Some(x) => println!("Batman is on the hero list"),
      // se non presente la chiave : &"Batman"
      None => println!("Batman is not in the hero list"),
    }
  }
}
```

---

## Le strutture

In Rust le strutture sono tipi di dati personalizzati progettati per raggruppare/includere diversi campi al loro interno.

Prendi come esempio il codice seguente:

```rs
fn main() {
  // creo una struttura Customer (contiene i valori: name, address e balance)
  struct Customer {
    name: String,
    address: String,
    balance: f32,
  }

  // popolo una variabile con la struttura 'Customer'
  let mut bob = Customer{
    name: String::from("Bob Smith"),      // assegno il nome
    address: String::from("55 Main St"),  // l'indirizzo
    balance: 234.50                       // ed il saldo
  };

  // posso poi modificare un valore del customer semplicemente accedendo al suo attributo
  // modifico quindi l'indirizzo facendo 'bob.address'
  bob.address = String::from("505 Main St");
}
```

Posso poi creare una struttra coi Generics, prendiamo come esempio il codice seguente:

```rs
fn main() {
  // creo una struttura con 2 generici (T ed U)
  struct Rectangle<T, U> {
    length: T,      // assegno il primo generico T a length
    height: U,      // assegno il secondo generico U a height
  }
  // creo una variabile 'rec' di tipo Rectangle
  let rec = Rectangle {
    length: 4,      // il primo valore lenght = 4
    height: 10.5    // il secondo valore height = 10.5
  }
}
```

---

## I Trait

I Trait in Rust sono una sorta di interfacce che definiscono le proprietà che devono avere gli oggetti che ne prendono il loro 'tratto'.

Considera il codice seguente:

```rs
fn main() {
  // costante del PI greco
  const PI: f32 = 3.141592;

  // definisco il tratto Shape (un contratto / interface)
  trait Shape {
    // ciascun elemento deve avere 2 funzioni 'new' e 'area'
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32;
  }

  // creo 2 strutture coi valori 'length' e 'width'
  struct Rectangle {length: f32, width: f32};
  struct Circle {length: f32, width: f32};

  // implemento il tratto Shape nella struttura Rectangle
  impl Shape for Rectangle {
    // definisco la funzione new definendo il valore '&self' con 'Rectangle'
    fn new(length: f32, width: f32) -> Rectangle {
      return Rectangle{length, width};
    }
    // definisco la funzione area definendo il valore di ritorno 'f32' con la moltiplicazione tra i 2 parametri della struttura 'lenght' e 'width'
    fn area(&self) -> f32 {
      return self.length * self.width;
    }
  }

  // implemento il tratto Shape nella struttura Circle
  impl Shape for Circle {
    // definisco la funzione new definendo il valore '&self' con 'Circle'
    fn new(length: f32, width: f32) -> Circle {
      return Circle{length, width};
    }
    // definisco la funzione area definendo il valore di ritorno 'f32' con il calcolo matematico relativo all'area del cercio
    fn area(&self) -> f32 {
      return (self.length / 2.0).powf(2.0) * PI;
    }
  }

  let rec: Rectangle = Shape::new(10.0, 10.0);
  let circ: Circle = Shape::new(10.0, 10.0);
  println!("Rec Area : {}", rec.area());
  println!("Circ Area : {}", circ.area());
}
```

Guardando con attenzione posso notare come :

- con l'implementation di Shape creo un tipo di dato più ricco per la Struttra Shape
  - aggiunge metodi quindi si "comporta" come una classe
- in Rust il tipo Rectangle e Circle implementano l'interfaccia Shape ma rimangono sempre dei tipi di variabile
  - nessuna ereditarietà, non c'è polimorfismo ma solo composizione/implementazione

---

## Packages Crates and Modules

E' giusto definire alcune caratteristiche che riguardano i moduli:

- **Crates** sono moduli creati per produrre una libreria o un eseguibile
- **Modules** servono ad organizzare e gestire la privacy (ovvero ciò che deve essere **public** o **private**)
- **Packages** fanno la build, il rest e lo sharing del **crates**
- **Paths** un modo per recperare un componente come una struttura, una funzione

Prendiamo un caso esempio, voglio creare un modulo per un ristorante:

- creo una cartella **/restaurant/** con all'interno un file **mod.rs**
- scrivo quindi questo codice nel file **/restaurant/mod.rs**

```rs
// dichiaro un nuovo modulo pizza_order
mod pizza_order {
  // creo una struttura public per Pizza, coi parametri: 'dough', 'cheese' e 'topping'
  pub struct Pizza {
    pub dough: String,
    pub cheese: String,
    pub topping: String,
  }

  // creo una funzione lunch che necessita dell'implementazione della struct Pizza
  impl Pizza {
    // passo l'attributo 'toppin' come parametro e definisco il ritorno come 'Pizza' (la struttura definita sopra)
    pub fn lunch(topping: &str) -> Pizza {
      // definisco i 3 parametri interni
      Pizza {
        dough: String::from("regular dough"),   // assegno un valore statico a douch → "regular dough"
        cheese: String::from("mozzarella"),     // assegno un valore statico a cheese → "mozzarella"
        topping: String::from(topping),         // assegno un valore dinamico a topping → ovvero il 'topping: &str' importato nella funzione
      }
    }
  }
  // creo un secondo modulo interno 'help_customer'
  pub mod help_customer{
    // definisco una funzione semplice che stampa un messaggio
    fn seat_at_table() {
      println!("Customer seated at table");
    }
    // funzione che stampa il messaggio relativo alla pizza ordinata (con il topping)
    fn serve_customer(cust_pizza: super::Pizza) {
      println!("The customer is served a regualr pizza with {}", cust_pizza.topping);
    }
    // definisco la funzione principale per la presa dell'ordine
    pub fn take_order() {
      seat_at_table();
      // creo un a variabile Pizza che inizializza il topping come "veggies
      let cust_pzza: super::Pizza = super::Pizza::lunch("veggies");
      // chiamo la funzione per "servire la pizza orfinata"
      serve_customer(cust_pizza);
    }
  }
}

// funzione principale 'order_food' chiamata dal main
pub fn order_food() {
  // richiamo la funzione interna passando all'interno dei namespaces
  crate::restaurant::pizza_order::help_customer::take_order();
}
```

Quindi implemento il modulo nel file **main.rs**

```rs
mod restaurant;   // dichiaro il modulo che voglio usare → 'restaurant'
use crate::restaurant::order_food;  // includo il nome della funzione 'restaurant::order_food' usata per accedere al modulo

fn main() {
  // chiamo la funzione 'order_food()'
  order_food();
}
```

---

## Files and Errors

Rust non ha una gestione delle eccezioni come nei linguaggi più comuni.  
Gli errori sono gestiti tramite il richiamoo di una macro 'panic' che esegue una stampa prima di uscire dal runtime come :

```rs
panic!("My custom fatal error... finish him !!")
```

Un esempio indiretto della gestione potrebbe essere:

```rs
fn main() {
  let lil_arr = [1, 2];
  println!("{}", lil_arr[10]);
}
```

Qua non viene menzionata la macro 'panic!' ma sotto il cofano viene gestito l'errore che comunica l'errore "out of bounds"

Se invece prendiamo come esempio una creazione di un file, come nel codice seguente:

```rs
use std::fs::File;
// considerando che il Result prevede 2 varianti : Ok e Err
  // enum Result<T, E> {
  // Ok(T),
  // Err(E), }
  // Dove 'T' rappresenta la generics del tipo di dato di ritorno
  // 'E' rappresenta la generics del tipo di errore

fn main() {
  // creo una variabile per il path del file da creare
  let path = "lines.txt";
  // chiamo il metodo di creazione file che salva il Result in 'output'
  let output = File::create(path);

  // switch/case su output (vedi referenza ad inizio snippet)
  let mut output = match output {
    // se tutto a posto ritorna il file stesso
    Ok(file) => file,
    // se da errore chiama la macro panic! e ritorna il messaggio
    Err(error) => {
      panic!("Problem creating file ; {:?}", error);
    }
  };

  // chiamo la macro write! che si occupa di scrivere nel file
  write!(output, "Just some\nRandom words")
    // se ci sono errori lancia un messaggio di errore
    .expect("Failed to write to file");

  // apro il file in input
  let input = File::open(path).unwrap();
  // creo un buffer dati che legge il contenuto del file
  let buffered = BufReader::new(input);
  // loop del buffer: linea per linea
  for line in buffered.lines() {
    // stampa la linea corrente, il metodo .unwrap() elimina la &, da il contenuto e non il risultato
    println!("{}", line.unwrap());
  }

  // creo un secondo file
  let output2 = File::create("rand.txt");
  // switch/case su output2
  let output2 = match output2 {
    // come prima se ok ritorna il file creato
    Ok(file) => file,
    // se da errore apre un secondo switch/case sul tipo di errore
    Err(error) => match error.kind() {
      // se non trova alcun tipo di errore definito
      ErrorKind::NotFound => match File::create("rand.txt") {
        // ...prova a ricreare il file
        Ok(fc) => fc,
        // se da ancora errore esce comunicado l'errore "Can't create file" con le specifiche
        Err(e) => panic!("Can't create file: {:?}", error),
      },
      // per altre casistiche non previste, esce dal runtime chiamando la macro panic! e stampando l'errore
      _other_error => panic!("Problem opening file : {:?}", error),
    },
  };
}
```

---

## Gli iterator

Come già visto gli iterator servono a ciclare all'interno di un loop per scansionare ciascun elemento dell'array.

```rs
fn main() {
  // creo il mio solito array di numeri
  let mut arr_it = [1, 2, 3, 4];
  // scansiono ogni elemento tamite metodo .iter
  for val in arr_it.iter() {
    println!("{}", val);    // stampa ciascun elemento (uno ad uno)
  }
  // creo una variabile per l'iter sull'array
  let mut iter1 = arr_it.iter();
  // Questo mi permette di scegliere l'elemento manualmente
  // Il metodo .next() prende l'elemento iniziale dell'iterator
  println!("1st : {:?}", iter1.next());   // stampa il primo della lista → 1st : Some(1)
}
```

---

## Le Closures

Le Closure sono funzioni anonime, e sono come salvate in una variabile. Sono utili a passare una funzione in un'altra funzione.  
Hanno una struttura simile a : `let var_name = |parameters| -> return_type {BODY}`

```rs
fn main() {
  // variabile che prende in carico una funzione closure
  let can_vote = |age: i32| {   // passo il parametro 'age'
    age >= 18           // ritorna true SOLO se maggiorenne
  };
  // stampa la stringa con il risultato diretto della chiamata alla closure 'can_vote'
  printl!("Can vote : {}", can_vote(8));
}
```

Un'altra delle caratteristiche fondamentali delle closure sta nella possibilità di stampare le variabili usate al suo interno.  
Vedi l'esempio sotto per capire meglio il loro utilizzo:

```rs
fn main() {
  // creo una semplice variabile mutabile
  let mut samp1 = 5;
  // definisco una closure per stamparla
  let print_var = || println!("samp1 = {}", samp1);   // stampa → samp1 = 5
  // chiamo la closure
  print_var();
  // modifico ancora la variabile = 10
  samp1 = 10;
  // creo una funzione closure che aggiunge 1
  let mut change_var = || samp1 += 1;
  change_var();         // chiama la closure per aggiungere 1 a 'samp1'
  println!("samp1 = {}", samp1);      // stampa → samp1 = 11
  // reimposto la variabile a 10
  samp1 = 10;
  println!("samp1 = {}", samp1);      // stampa → samp1 = 10
}
```

> Come si può notare 'samp1' è usata sia dentro che fuori la funzione closure.

Un'altra possibilità di implementazione è quella di passare una funzione nella closure.  
Prendiamo come esempio il codice seguente:

```rs
fn main() {
  // definisco una funzione generica che accetta 2 interi (a e b)
  // ed una funzione 'func' che prende due i32 e restituisce un i32.
  fn use_func<T>(a: i32, b: i32, func: T) -> i32
  where T: Fn(i32, i32) -> i32 {    // vincolo: T deve essere una closure o funzione compatibile
    func(a, b)    // ritorna il risultato dell'operazione
  }

  // definisco quindi 2 closure (funzioni anonime)
  // Una per la somma:
  let sum = |a, b| a+b;
  // Una per il prodotto:
  let prod = |a, b| a*b;

  // Chiamo la funzione use_func passando a=5, b=4 e la closure 'sum'
  println!("5 + 4 = {}", use_func(5, 4, sum));  // stampa → 5 + 4 = 9
  // Chiamo la funzione use_func passando a=5, b=4 e la closure 'prod'
  println!("5 * 4 = {}", use_func(5, 4, prod)); // stampa → 5 * 4 = 20
}
```

Il Bound come già visto serve a definire un vincolo/clausula che viene separata in questo caso dalla definizione della funzione tramite 'where'.

In alternativa avrei potuto usare l'altra espressione `fn f<T: Bound>(...)`

```rs
fn use_func<T: Fn(i32, i32) -> i32>(a: i32, b: i32, func: T) -> i32 {
  // [...]
}
```

> Questa versione è più compatta ma meno leggibile

--

## Lo smart pointer Box

Come abbiamo già visto lo stack è una porzione di memoria che definisce una porzione di memoria in formato LIFO.  
Il Box è uno smart pointer simile allo stack.

Per capire meglio creo uno snippet che lo implementa in maniera molto basilare:

```rs
fn main(){
  // Creo una variabile Box inizializzata col numero 10
  let b_int1 = Box::new(10);
  println!("b_int1 = {}", b_int1);    // stampa → b_int1 = 10
}
```

Prendiamo adesso un esempio diverso integrando una struttura come segue:

```rs
fn main(){
  struct Point<T> {
      x: T,
      y: T,
  }

  impl<T> Point<T> {
      fn new(x: T, y: T) -> Self {
          Point { x, y }
      }
  }

  let point1 = Point::new(5, 6);
  println!("{:?}", point1);  // stampa → Point { x: 5, y: 6 }
}
```

> Qua la struttura non necessita del Box poiché è limitata all'utilizzo di un solo nodo ma serve per capire il prossimo esempio...

Nell'esempio che segue si crea una struttura più complessa ad albero :

- ogni chiave ha 2 sottonodi, left e right
- a loro volta ciascuno di questi (se inizializzati) possono avere dei nodi figli (left e right)
- e così via...

Potenzialmente la struttura ha un heap infinito che se non limitato potrebbe occupare tutta la memoria (con calma ma potrebbe arrivarci in linea teorica.....).  
Qui lo "smart pointer" **Box** agisce proprio come una scatola con uno spazio limitato che definisce un puntatore a un valore T allocato nell' **Heap**,  
permettendo di definire tipi ricorsivi con dimensione finita.

```rs
fn main(){
  // creo una struttura ad albero
  struct TreeNode<T> {
    // ciascun nodo può avere :
    // un nodo sinistro
    pub left: Option<Box<TreeNode<T>>>,
    // un nodo destro
    pub right: Option<Box<TreeNode<T>>>,
    // una chiave
    pub key: T,
  }
  // qua implemento il tipo T che è il generico compreso nella struttura 'TreeNode'
  impl<T> TreeNode<T> {
    // creo una funzione new che gestisce la chiave come parametro primario
    pub fn new(key: T) -> Self {
      // inizializza i 2 nodi left e right a None mantenendo valida la chiave passata
      TreeNode { left: None, right: None, key, }
    }
    // imposta il nodo sinistro dell'albero e ritorna il nodo corrente
    pub fn left(mut self, node: TreeNode<T>) -> Self {
      // assegna al campo 'left' il nuovo nodo, incapsulato in Box e Option
      self.left = Some(Box::new(node));
      self
    }
    // imposta il nodo destro dell'albero e ritorna il nodo corrente
    pub fn right(mut self, node: TreeNode<T>) -> Self {
      // assegna al campo 'right' il nuovo nodo, incapsulato in Box e Option
      self.right = Some(Box::new(node));
      self
    }
  }

  // inizializzo una variabile 'node1' di tipo TreeNode
  let node1 = TreeNode::new(1)    // il primo è la chiave → 1
    .left(TreeNode::new(2))       // il nodo left → 2 , che è a sua volta la chiave di un nuovo TreeNode
    .right(TreeNode::new(3));     // il nodo right → 3 , che è a sua volta la chiave di un nuovo TreeNode
}
```

Qua l'utilizzo di Box alloca una sorta di Stack sulla memoria **Heap**.
Serve a limitare la ricorsione ad albero di TreeNode (Rust non permette le ricorsioni infinite).

Quindi quando definisco il nodo left o right all'interno della struttura con `Option<Box<TreeNode<T>>>`, sto dicendo che:

- **Option** è il tipo che permette di rendere una variabile nulla
- **Box** funge sa "smart pointer" per allocare una dimensione finita sulla memoria heap
- senza **Box** ovvero con `Option<TreeNode<T>>` otterrei un errore come **_recursive type `TreeNode` has infinite size_**

nelle definizioni interne dei metodi

- in `pub fn new(key: T) -> Self` il comando **-> Self** ritorna proprio il tipo della struct
- in `pub fn right(mut self, node: TreeNode<T>) -> Self` la variabile **mut self** è un riferimento alla classe (manipolata all'interno della funzione)
- `self` alla fine → ritorno l’oggetto concreto
- `self.left` → sto accedendo e modificando il campo **_left_** della struct

---

## Concurrency

La cocorrenza in Rust permette l'esecuzione di più blocchi di codice nello stesso momento. Questo stile di esecuzione è definito come programmazione parallela.  
Ci sono una serie di problemi comuni che riguardano al programmazione parallela:

1. I Thread solitamente cercano di accedere ai dati in un ordine sbagliato
2. Più thread sono bloccati in fase di esecuzione per via di una confusione sull'ordine di priorità ed i requisiti da rispettare per procedere.

Se per dire un thread si avvia SOLO se ha accesso ad una risorsa che però è presa da un secondo thread che la occupa:

- Il primo thread ha l'ok per avviarsi
- Il secondo thread occupa la risorsa impedendo l'avvio del primo
- Il sistema va in confusione facendo scelte casuali e creando anche problemi di prestazione

In Rust questa gestione avviene in automatico perché le impostazioni di priorità sono gestite internamente risolvendo gran parte di questi problemi.

In questo esempio si spiega come fare ad implementare la programmazione parallela in Rust:

```rs
use std::thread;
use std::time::Duration;

fn main() {
  // faccio partire un thread secondario
  thread::spawn(|| {
    // start spawned loop
    for i in 1..25 {
      println!("Spawned thread : {}", i);
      // ferma il loop per 1 millisecondo
      thread::sleep(Duration::from_millis(1));
    }
  });

  // start main loop
  for i in 1..20 {
    println!("Main thread : {}", i);
    // ferma il loop per 1 millisecondo
    thread::sleep(Duration::from_millis(1));
  }
}
```

Nel codice si può notare come il loop secondario (quello di "Spawned thread") abbia una durata maggiore → fino a 25.  
Il problema è che il "Main thread" cicla fino al 20. Così facendo quando conclude esce dal runtime lasciando lo "Spawned thread" incompleto.

C'è una soluzione che Rust offre per far fronte a questi problemi di interruzione improvvisa, mostrata nella revisione del codice precedente come segue:

```rs
use std::thread;
use std::time::Duration;

fn main() {
  // creo una variabile di riferimento al tread istanziato
  let thread1 = thread::spawn(|| {
    for i in 1..25 {
      println!("Spawned thread : {}", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..20 {
    println!("Main thread : {}", i);
    thread::sleep(Duration::from_millis(1));
  }

  // faccio una .join() dello "Spawned thread" sul main
  // mette in attesa la chiusura del runtime dando la possibilità a thread1 di finire
  thread1.join().unwrap();
}
```

Per fare un esempio concreto (real world) vediamo come si può integrare questa programmazione parallela con un esempio di gestione in un sistema bancario.

```rs
use std::thread;
use std::time::Duration;

fn main() {
  // creo una struttura 'Bank'
  pub struct Bank {
    // ... con un solo dato: il bilancio del conto
    balance: f32
  }
  // metodo 'withdraw' per il prelievo del denaro
  fn withdraw(the_bank: &mut Bank, amt: f32) {
    // sottraggo 'amt' (importo) dal bilancio del conto passato come riferimento mutabile
    the_bank.balance -= amt;
    // stampo il nuovo saldo
    println!("Balance : {}", the_bank.balance);
  }
  // creo un'stanza mutabile della banca con saldo iniziale a 100.0
  let mut bank = Bank{ balance: 100.0 };

  // effettuo un primo prelievo di 5.00 dal main thread
  withdraw(&mut bank, 5.00);

  // stampo il saldo dopo il primo prelievo
  println!("Balance : {}", bank.balance);

  // metodo 'customer' per la simulazione di un prelievo da parte di un utente
  fn customer(the_bank: &mut Bank) {
    // esegue un prelievo per l'utente passando : riferimento mutabile del saldo bancario e l'importo da prelevare 5.00
    withdraw(the_bank, 5.00);
  }

  // creo un nuovo thread di esecuzione
  thread::spawn(|| {
    // all'interno del thread creo una nuova istanza di 'Bank' (separata da quella del main)
    // per praticità non intervengo sullo stesso conto (richiederebbe un implementazione Mutex che evita una Race Condtion)
    let mut bank = Bank { balance: 100.0 };

    // ... ma richiamo in parallelo la stessa funzione
    customer(&mut bank)
  })
  // con .join aspetto che il thread finisca prima di terminare il main thread
  .join()
  // .unwrap gestisce il risultato del .join (panicherà se il thread ha generato un errore)
  .unwrap();
}
```

---

## Arc & Mutex

Come abbiamo visto prima il riferimento della banca è stato creato prima nel main thread e poi all'interno del thread istanziato alla fine.  
Questo perché in Rust la programmazione parallela non permette una modifica/scrittura contemporanea della stessa variabile senza compromettere l'accesso ai dati.  
Questa concomitanza conflittuale è chiamata 'Race Condition'.

Esiste una soluzione che richiede l'implementazione di alcune "funzioni di blocco" che impongono ad una determinata operazione
il controllo di blocco della variabile o del dato al momento della richiesta.

Questo tipo di implementazione in Rust avviene grazie ad alcuni componenti:

- Arc (Atomic Reference Counter): **Arc<T>** è uno smart pointer thread-safe che consente la condivisione su + threads della **ownership** di un dato.
- Mutex (Mutual Exclusion): **Mutex<T>** protegge l'accesso mutabile a un dato condiviso. Un solo thread per volta può bloccarlo per scrivere/leggere il valore.

L'implementazione avviene tramite una struttura come `Arc<Mutex<Bank>>` dove Bank è **T**, che permette l'accesso sicuro e condiviso tra threads.

Si può quindi rivedere il codice precedente con l'implementazione di Arc e Mutex come segue:

```rs
use std::sync::{Arc, Mutex};  // importo Arc (Atomic Reference Counter) e Mutex (mutua esclusione)
use std::thread;              // importo le funzionalità per creare e gestire thread

fn main() {
    // definisco una struttura 'Bank'
    pub struct Bank {
        // contiene un solo campo: il saldo del conto
        balance: f32,
    }

    // funzione che effettua un prelievo
    fn withdraw(the_bank: &mut Bank, amt: f32) {

        // check che controlla il saldo attuale per verificare che l'importo richiesto sia disponibile sul conto
        if the_bank.balance < amt {
          println!("Insufficient funds: balance = {}, attempted withdrawal = {}", the_bank.balance, amt);
        } else {
          // riduce il saldo dell'importo specificato
          the_bank.balance -= amt;
          // stampa il saldo aggiornato
          println!("Balance : {}", the_bank.balance);
        }
    }

    // funzione che rappresenta un cliente che interagisce con la banca
    fn customer(the_bank: Arc<Mutex<Bank>>) {
        // blocca il Mutex per ottenere accesso esclusivo e mutabile alla struct Bank
        let mut bank_ref = the_bank.lock().unwrap();
        // effettua un prelievo di 5.00
        withdraw(&mut bank_ref, 5.00);
        // il lock viene automaticamente rilasciato alla fine del blocco (quando bank_ref va fuori scope)
    }

    // creo l'istanza iniziale della banca, con saldo 100.0,
    // e la incapsulo in un Arc<Mutex<T>> per permettere accesso condiviso e sicuro tra thread
    let bank = Arc::new(Mutex::new(Bank { balance: 100.0 }));

    // crea 10 thread "clienti"
    let mut handles: Vec<_> = (0..10).map(|| {
      let bank_ref = bank.clone();
      thread::spawn(|| {
        customer(bank_ref)
      })
    }).collect();

    // aggiungo un nuovo thread in coda ai precedenti
    handles.push({
      // clono il riferimento a 'bank'
      let bank_ref = Arc::clone(&bank);
      // faccio lo spawn del nuovo thread chiamando 'customer' e passando il riferimento bank_ref
      thread::spawn(|| customer(bank_ref))
    });

    // prelievo nel main
    {
      // blocco il Mutex per ottenere l'accesso esclusivo al saldo
        let mut bank_ref = bank.lock().unwrap();
        // effettuo un prelievo
        withdraw(&mut bank_ref, 10.00);
    } // qui il Mutex viene sbloccato automaticamente

    //
    for handle_index in handles {
      handle_index.join().unwrap();
    }

    // alla fine stampo il saldo finale
    println!("Final Balance : {}", bank.lock().unwrap().balance);
}
```

Qua avviene l'implementazione delle 2 componenti :

```rs
let bank = Arc::new(Mutex::new(Bank { balance: 100.0 }));
```

Dove:

- **Arc** : crea una prima istanza condivisibile concorrente per la lettura o l'accesso concorrente
- **Mutex** : si implementa per permettere la modifica del dato (un solo thread per volta)

  - Solo un thread alla volta può leggere
  - Solo un thread alla volta può scrivere

Per accedere in maniera "esclusiva" alla banca utilizzo il metodo **.lock** :

```rs
let mut bank_ref = the_bank.lock().unwrap();  // Nella funzione 'customer', poi passato a 'withdraw'

let mut bank_ref = bank.lock().unwrap();    // nel main thread, tra le parentesi {}, per un prelievo aggiuntivo sempre tramite 'withdraw'

bank.lock().unwrap().balance    // nella macro `println!` per la verifica del conto
```

Altra cosa da notare è che quando creo le referenze per la banca (per il passaggio della referenza a 'withdraw') la imposto come segue:

```rs
let mut bank_ref = the_bank.lock().unwrap();    // Nella funzione 'customer'

let bank_ref = bank.clone();  // nel loop .map assegnato al vettore 'handles'

let bank_ref = Arc::clone(&bank); // nel .push sul vettore 'handles'

let mut bank_ref = bank.lock().unwrap();   // nel main
```

E' importante capire perché uso sempre **_bank_ref_** senza rischiare alcun tipo di _"shadowing"_, questo succede perché ogni riferimento su bank_ref appartiene ad uno scope ben distinto e quinid non viene sovrascritto da Rust.

---

## RwLock o Mutex

Per applicare un vero parallelismo esiste un terzo componente : **RwLock**

- **RwLock** : è un alternativa a _Mutex_ fa la stessa cosa (condividere la manipolazione tra i thread) ma:
  - permette più accessi contemporanei in lettura
  - ed uno solo in scrittura

La scelta tra Mutex e RwLock dipende dal tipo di accessi previsti ad un determinato dato:

- se ad esempio si tratta di un conto bancario è sufficente un Mutex
  - Il Mutex non distingue tra lettori e scrittori, considera tutti come "utilizzatori esclusivi"
- se ho bisogno di condividere il dato in lettura tra più parti (come ad esempio un app di criptovalute) è consigliabile RwLock
  - ci sono quindi tante chiavi per leggere ed 1 per scrivere

```rs
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

let data_mutex = Arc::new(Mutex::new(0));
let data_rwlock = Arc::new(RwLock::new(0));

// Mutex: anche solo leggere blocca gli altri
{
    let _lock1 = data_mutex.lock().unwrap();
    let _lock2 = data_mutex.lock().unwrap(); // ❌ blocca: lock esclusivo
}

// RwLock: più lettori insieme
{
    let _r1 = data_rwlock.read().unwrap();
    let _r2 = data_rwlock.read().unwrap(); // ✅ funziona: letture parallele
}
```

---
