# Expand your possibilities with the Try `?` Operator

[![crate](https://img.shields.io/crates/v/try_more.svg)](https://crates.io/crates/try_more)
[![documentation](https://docs.rs/try_more/badge.svg)](https://docs.rs/try_more)

 Have you ever found yourself writing a function which may return early based on some condition?

 ```rust
 fn my_function() {
     // ...

     if condition_a {
         return;
     }
     
     // ...
 
     if condition_b {
         return;        
     }
     
     // ...
 }
 ```

 It doesn't look Rusty, right? This crate offers an extension trait to be able to convert from
 a `bool` to a `ControlFlow` and leverage the mighty power of `?` to get rid of those checks:

 ```rust
 use core::ops::ControlFlow;
 use try_more::BoolFlow;

 fn my_function() -> ControlFlow<()> {
     // ...

     BoolFlow::r#break(condition_a)?;

     // ...

     condition_b.r#break()?;
     
     // ...
   
 }
 ```

 There's also other methods besides `continue` and `break` which allows to control the value which is passed to the `Break` variant.

