use std::env;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let n = env::args().nth(1).unwrap().parse::<u128>()?;

    (1..=n).for_each(|i| {
        println!("{}", fizzbuzz(i));
    });

    Ok(())
}

fn fizzbuzz(n: u128) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_owned(),
        (0, _) => "Fizz".to_owned(),
        (_, 0) => "Buzz".to_owned(),
        _ => n.to_string(),
    }
}
