# Take user input in Rust

At **project/src/**  
  
``$ git clone https://github.com/evolade/take.git``  
  
This will create a folder named "take".
Open the Rust file that you want to use input.
## Add
```rust
#[path = "take/input.rs"]  
mod take;  

    let _name = take::input("name: ");

```
