type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

use std::env;

fn main() -> Result<()> {
    let n = env::args().nth(1).unwrap().parse::<u128>()?;

    (1..=n).for_each(|i| {
        println!("{}", fizzbuzz(i));
    });

    Ok(())
}

fn fizzbuzz(number: u128) -> String {
    match number {
        n if n % 3 == 0 && n % 5 != 0 => "Fizz".to_owned(),
        n if n % 5 == 0 && n % 3 != 0 => "Buzz".to_owned(),
        n if n % 3 == 0 && n % 5 == 0 => "FizzBuzz".to_owned(),
        n => n.to_string(),
    }
}
