# grep-off

A simple rip off of grep written in Rust. This project is mainly for education purposes and to learn Rust. It is not meant to be a replacement for grep.

The project is based on the instruction from chapter 12 of the book [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).

## Prerequisites

Install Rust (of course!).

## Quick start

```bash
  cargo run -- [PATTERN] [FILE] [OPTIONS]

  -i: case insensitive
  FILE: currently test.txt
```

## Examples

```bash
  cargo run -- rust test.txt
  cargo run -- rust test.txt -i
```
