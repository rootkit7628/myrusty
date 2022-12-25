use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Atsika tsika hihisa !");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("Firy me gne sifranao.");

    let mut guess = String::new();

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Eee! nagnadary koa i endrinira tiky");

        let guess: u32 = guess.trim().parse().expect("Tsy henonao me nge raha zakaiko ake, ino hoa gne sifranao !");

        println!("nge nomenao {guess} anah ake.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Pitiky lotry!"),
            Ordering::Greater => println!("Zakabe lotry!"),
            Ordering::Equal => {
                println!("Marigny !");
                break;
            }
        }
    }
}
