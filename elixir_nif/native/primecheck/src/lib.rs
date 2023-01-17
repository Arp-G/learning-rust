#[rustler::nif]

fn is_prime(number: i64) -> bool {
  for i in 2..number {
      if number % i == 0 {
          return false;
      }
  }
  true
}

rustler::init!("Elixir.RustPrimeCheck", [is_prime]);
