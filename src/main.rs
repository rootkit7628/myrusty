use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn annouce_result(res: &str, guess: u8) {
    println!("{res}: {guess}");
}

fn main() {
    println!("Atsika tsika hihisa !");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Firy me gne sifranao.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Eee! nagnadary koa i endrinira tiky");

        let guess: u8 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
        };

        println!("nge nomenao {guess} anah ake.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Pitiky lotry!"),
            Ordering::Greater => println!("Zakabe lotry!"),
            Ordering::Equal => {
                annouce_result("Hitanao nge valiny !", guess);
                break;
            }
        }
    }
}
