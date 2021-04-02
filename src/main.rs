// 0. Correr "$ rustc main.rs" crea un binario ejecutable.
// 0.1 Cargo es el sistema de buildeo de Rust y package manager.
// 0.2 "$ cargo build" buildea el código y "$ cargo run" ejecuta el binario
//     generado en target/debug.
// 0.3 "$ cargo check" chequea que el código compile sin errores sin producir
//     un ejecutable.
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    // 1. El "!" indica que se invoca una macro en lugar de una función.
    println!("Hello World!");
    guess_game();
}

fn guess_game() {
    println!("Guess the number");
    // 4. "rand::thread_rng()" retorna un generador de números, que es local al hilo de ejecución actual.
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess:");
        // 2. Por defecto las variables son inmutables.
        // 2.1 El modificador "mut" convierte a la variable en mutable.
        // 2.2 El operador "::" indica que el método que le sigue es una "función asociada"
        //     la cual es uan función implementada en un tipo y no en una instancia específica.
        //     Algo así como un método estático
        let mut guess = String::new();
    
        // 3. "io::stdin" es equivalente a "std::io::stdin".
        // 3.1 El operador "&" indica que el argumento es una referencia. La ventaja de usar referencias
        //     es que permiten acceder datos multiples veces sin necesidad de copiar más datos en memoria.
        //     Las referencias son inmutables por default también.
        // 3.2 "read_line()" retorna un valor del tipo "Result", en este caso "io::Result". Los "Result" son enumerativos,
        //     por lo cual pueden tener un número fijo de variantes, en este caso "Ok" o "Err", que contiene información
        //     adicional. El método "expect()" muestra el mensaje indicado si el retorno es "Err".
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // 5. Rust permite hacer shadowing de variables, lo cual es útil para convertir variables de un tipo a otro
        // 5.1 El método "parse()" en un string convierte al mismo en otro tipo, y ese tipo, en este caso es inferido
        //     en el tipo de la variable ": u32".
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // 7. El parámetro "_" simboliza un comodín, es decir, un valor de cualquier tipo que, en este caso, arroje
            //    "Err".
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
    
        // 6. La estructura "match" funciona como un switch.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal =>  {
                println!("You win!");
                break;
            }
        }
    }
}


