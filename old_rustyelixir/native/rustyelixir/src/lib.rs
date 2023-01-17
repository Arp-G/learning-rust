use rustler::{Encoder, Env, Error, Term};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler::rustler_export_nifs! {
    "Elixir.Rustyelixir",
    [
        ("add", 2, add),
        ("is_prime", 1, is_prime),
    ],
    None
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let num1: i64 = args[0].decode()?;
    let num2: i64 = args[1].decode()?;

    Ok((atoms::ok(), num1 + num2).encode(env))
}

/*
  RESULT

    Result is a richer version of the Option type that describes possible error instead of possible absence.
    That is, Result<T, E> could have one of two outcomes:
    Ok(T): An element T was found
    Err(E): An error was found with element E

    Eg: When parsing strings we can match on result
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };

    With Rustler the result of a NIF is a Result<T, E> where
    T = rustler::Term
    E = rustler::error::Error
  ----------------------------------------------------------------
  ENV

  On each NIF call, a Env is passed in.
  The Env is used for most operations that involve communicating with the BEAM, like decoding and encoding terms.

  Ref: https://docs.rs/rustler/0.18.0/rustler/env/struct.Env.html

  ----------------------------------------------------------------
  TERM

  Term here is used to represent all erlang terms. Terms are always lifetime limited by a Env.
  Terms cannot exist outside of the lifetime of the Env that owns it.
  Ref: https://docs.rs/rustler/0.18.0/rustler/struct.Term.html

  Here...
  `env: Env<'a>, args: &[Term<'a>]` means the Env has a lifetime `'a` and the term has the same lifetime.

  The term has a decode() method used as the primary method of extracting the value from a Term.
  Before returning from the NIF we must encode the result in a term to represent it in a format recognizable to beam
  that is we must encode it to a term tied to the Env.
  Therefore we see the NIF results a Result of Term or rustler::error::Error

  Here when we do `Ok((atoms::ok(), false).encode(env))`
  It returns {:ok, boolean} value to elixir represented encoded as a Term
*/

fn is_prime<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let number: i64 = args[0].decode()?;

    for i in 2..number {
        if number % i == 0 {
            return Ok((atoms::ok(), false).encode(env));
        }
    }

    Ok((atoms::ok(), number > 1).encode(env))
}
