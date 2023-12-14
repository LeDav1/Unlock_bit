# Unlock_bit

## Choice of Programming Language
- Rust
  - Faster, realist settings, opportunity to learn Rust
  - More complicated syntax than python
  - Must know victim's architecture

## Ransomware developer team


### Plan
- Learn how Rust works
- Use a tutorial to do the ransomware
  - https://www.youtube.com/watch?v=1u1njcN_6O8&ab_channel=MichaelMullin
  - https://www.youtube.com/watch?v=l_St5T1dOdo&ab_channel=FredrikChristenson
  - https://www.youtube.com/watch?v=mIpePYFBKtY&ab_channel=Heapzip
- Test the ransomware on a dedicated directory
- Test the ransomware on a virtual machine

## Forensics Team
- Mateo
- Corentin

## Documentation

I/O calls and best practices are taken from [Rust's documentation on I/O](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

### Usage:

```bash
cargo run -- command path
```

Example

```rust
cargo run -- encrypt path 
```

## Coding practices

**Separation of Concerns for Binary Projects**

The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. As a result, the Rust community has developed guidelines for splitting the separate concerns of a binary program when main starts getting large. This process has the following steps:

- Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
- As long as your command line parsing logic is small, it can remain in main.rs.
- When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

The responsibilities that remain in the main function after this process should be limited to the following:

- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a run function in lib.rs
- Handling the error if run returns an error

This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the task at hand. Because you can’t test the main function directly, this structure lets you test all of your program’s logic by moving it into functions in lib.rs. The code that remains in main.rs will be small enough to verify its correctness by reading it. Let’s rework our program by following this process.
