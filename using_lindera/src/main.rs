use std::io::{stdin};
use lindera::tokenizer::Tokenizer;
use lindera::LinderaResult;


fn main() -> LinderaResult<()> {
    let mut word = String::new();

    stdin().read_line(&mut word).unwrap();

    // create tokenizer
    let tokenizer = Tokenizer::new()?;

    // tokenize the text
    let tokens = tokenizer.tokenize(&word)?;

    // output the tokens
    for token in tokens {
        println!("{}", token.text);
    }

    Ok(())
}