
// standard rust error handing
// Result<T, E>
// match
// ? try opt

fn a_fn_that_errs() -> Result<(), String> {
    Err("to err is human".to_string())
}

// panic! unwrap expect. Simple and effective! and crashes your program
fn using_expect_and_unwrap() {
    let something = a_fn_that_errs().expect("this will crash the program");
    // same thing without message
    // a_fn_that_errs().unwrap();
}

// match to gracefully handle the error, or carry on.
fn basic_handling_with_match() {
    match a_fn_that_errs() {
        Ok(something) => {
            println!("all is good");
        }
        Err(e) => {
            println!("handling the error: {e}");
        }
    }
}

// if you don't want to handle the error yet use the `?` to bubble up.
fn try_operator() -> Result<(), String> {
    let something = a_fn_that_errs()?;

    // do other things

    Ok(())
}

fn main()
