## Simple Calculator
Intended as a potential coding interview question, I implemented a simple calculator in Rust.

The input is a string such as "3 + 5 / 2", and, following typical order of operations, evaluates the expression to be 5.5. It's very simple, and only implements division, multiplication, subtraction, and addition, along with integer input. I would need a much more robust tokenizer to support anything more, but again, this was done to practice coding interview answer, where simple implementation is key.

All of the tests pass, thankfully.

I also did this as a way to explore some idiomatic rust implementation, such as using the powerful enums and iterators over loops. The best part of it all was, despite this being a systems language, once cargo check and clippy were happy, the code was 99% functional. These days it's rare to have so much information readily available as you're writing the code.