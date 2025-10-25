#![allow(unused)]

use rand::Rng;
use rand::prelude::*;

fn main() {
    // Todo
    // - [x] build a logic to generate a password
    // - [x] testing that everything works

    let length: usize = 2000;
    let chars_pool:Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+[]{}|;:',.<>?/`~".chars().collect();

    // eseguo l'operazione di random e salvo il risultato in 'rng'
    let mut rng = rand::rng();

    // avvio loop che va da 0 a lenght
    let password: String = (0..length)
        .map(
            // - ignoro la clojure (uso _ quella anonima)
            // - 'choose' restituisce un riferimento a un elemento del vettore (ovvero Option<&char>)
            |_| *chars_pool.choose(&mut rng).unwrap(),
        )
        .collect();

    println!("Password is: {}", password);
}

/*  SPIEGAZIONE

1.  chars_pool.choose(&mut rng) → ritorna Some(&'E') (riferimento al char 'E')
2.  .unwrap() → estrae il valore, ora hai &'E'
3.  * → dereferenzia, ottieni il vero char 'E'
4.  map() raccoglie tutti questi char e .collect() li mette in una String

*/
