## Quickscopejs

A simple cli utility that takes a script directory as an argument and outputs a compiled 'scoped.js' file containing all your scripts scoped according to the file hierarchy for you to source in your html.

## Example

$ quickscopejs scripts_dir
>> ./scoped.js created

## Why

I don't like how you're forced to learn a huge framework to make js scripting practical.  I'd rather just have simple scoping and be able to organize my files however I want.

## Installation

For now you need to install Rust, then clone the repo and run `cargo compile`.  Pre-compiled downloads will probably happen at some point.
