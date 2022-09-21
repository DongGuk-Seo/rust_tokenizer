## Using tokenizer-lindera with rust

Available tokenizer with Korean

### Codes
```rust
use lindera::tokenizer::Tokenizer;
use lindera::LinderaResult;

fn main() -> LinderaResult<()> {
    // create tokenizer
    let tokenizer = Tokenizer::new()?;

    // tokenize the text
    let tokens = tokenizer.tokenize("식사는스테이크가어떠신가요?")?;

    // output the tokens
    for token in tokens {
        println!("{}", token.text);
    }

    Ok(())
}
```


### how to run
```shell script
% cargo run
```

### check your result

```text
식사
는
스테이크
가
어떠신가요
?
```