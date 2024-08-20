# Denim Language Tokenizer

Interprets Denim source files as a sequence of "tokens".

## Inspiration

This tokenizer implementation is close relative of Rust's lexer,
[rustc-lexer](https://github.com/rust-lang/rust/tree/master/compiler/rustc_lexer).

## Testing

### Updating goldens

To update the goldens, simply ensure that the `UPDATE_EXPECT` environment
variable is set when running the golden tests.

```sh
$ UPDATE_EXPECT=true cargo test
```

For more on this, see the [`expect_test` docs](https://docs.rs/expect-test/latest/expect_test/#introduction).