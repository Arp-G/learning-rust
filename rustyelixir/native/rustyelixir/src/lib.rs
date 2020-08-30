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

fn is_prime<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {

    let number: i64 = args[0].decode()?;
    
    for i in 2..number {
        if number % i == 0 {
            return Ok((atoms::ok(), false).encode(env));
        }
    }
    
    Ok((atoms::ok(), number > 1).encode(env))
}
