# sleepdrifter
Lazy evaluation library for Rust


## Examples

Calculating the root-mean-square:

```rust
let a = lazy(34.2);
let b = lazy(25.6);

let rms = ((a + b) / lazy(2.0)).map(|mean| mean.sqrt());

println!("{}", rms.evaluate());
```

Parameter usage:

```rust
let (a, setter) = Parameter::empty();
let b = lazy(25.6);

let rms = ((a + b) / lazy(2.0)).map(|mean| mean.sqrt());

setter.set(34.2);

println!("{}", rms.evaluate());
```
