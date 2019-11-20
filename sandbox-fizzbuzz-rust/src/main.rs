type UnitResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> UnitResult {
    let n = std::env::args().nth(1).ok_or("None")?.parse::<u128>()?;

    let _ = (1..n + 1).for_each(|i| {
        println!("{}", fizzbuzz(i));
    });

    Ok(())
}

fn fizzbuzz(number: u128) -> String {
    match number {
        n if n % 3 == 0 && n % 5 != 0 => "Fizz".to_string(),
        n if n % 5 == 0 && n % 3 != 0 => "Buzz".to_string(),
        n if n % 3 == 0 && n % 5 == 0 => "FizzBuzz".to_string(),
        n => n.to_string(),
    }
}
