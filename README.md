# ALX Rust Macros

A comprehensive Rust metaprogramming and macro engineering repository focused on mastering Rust compile-time systems through practical ALX-style projects.

This repository explores Rust macros from beginner fundamentals to advanced framework-level metaprogramming concepts used in real-world systems engineering, backend infrastructure, and compiler tooling.

---

# Overview

Rust macros are one of the most powerful features in the language. They allow developers to generate code at compile time, reduce boilerplate, enforce correctness, build custom DSLs (Domain Specific Languages), and create framework-level abstractions.

This repository is designed as a structured learning and engineering lab for:

* mastering `macro_rules!`
* understanding token tree parsing
* building recursive macros
* learning macro hygiene
* designing compile-time DSLs
* writing procedural macros
* generating APIs and framework systems

The projects follow an ALX-inspired progression model, starting with macro basics and gradually advancing into compiler-style engineering.

---

# Learning Objectives

By completing the projects in this repository, you will learn how to:

* Define and invoke Rust macros
* Understand macro expansion
* Work with fragment specifiers
* Parse token trees (`tt`)
* Use repetition patterns effectively
* Build recursive macros
* Design custom DSLs
* Debug macro expansion
* Understand macro hygiene and scoping
* Build procedural macros
* Parse Rust syntax trees using `syn`
* Generate code using `quote`
* Design framework-level compile-time systems

---

# Repository Structure

```text
alx-rust-macros/
│
├── README.md
├── LICENSE
├── .gitignore
├── Cargo.toml
│
├── 0x00-macro-basics/
├── 0x01-repetition-patterns/
├── 0x02-token-tree-parsing/
├── 0x03-recursive-macros/
├── 0x04-macro-hygiene/
├── 0x05-procedural-macros/
├── 0x06-framework-projects/
│
└── docs/
```

---

# Module Breakdown

## 0x00 - Macro Basics

Introduction to Rust macros and compile-time expansion.

### Topics

* `macro_rules!`
* pattern matching
* macro templates
* fragment specifiers
* simple expansion

### Projects

* hello macro
* calculator macro
* sum macro
* debug macro
* logging macro

---

## 0x01 - Repetition Patterns

Understanding repetition and variadic macro systems.

### Topics

* `$(...)*`
* `$(...)+`
* optional trailing commas
* repeated expansion
* list parsing

### Projects

* custom `vec!`
* variadic print macro
* array builder
* comma parser

---

## 0x02 - Token Tree Parsing

Advanced parsing using token trees and DSL design.

### Topics

* `tt` fragments
* nested parsing
* syntax design
* compile-time builders
* DSL engineering

### Projects

* JSON macro
* HTML DSL
* CSS DSL
* query builder macro

---

## 0x03 - Recursive Macros

Recursive compile-time systems and token processing.

### Topics

* recursive expansion
* recursive parsing
* token counting
* nested evaluation

### Projects

* recursive sum macro
* recursive parser
* token counter
* nested expression evaluator

---

## 0x04 - Macro Hygiene

Understanding variable scoping and hygienic expansion.

### Topics

* hygienic macros
* scope isolation
* identifier collisions
* safe variable generation
* expansion safety

### Projects

* hygiene lab
* scoped bindings
* safe temporary variables
* collision prevention systems

---

## 0x05 - Procedural Macros

Advanced AST-level metaprogramming.

### Topics

* `proc_macro`
* derive macros
* attribute macros
* syntax parsing
* token stream manipulation
* AST transformations

### Libraries

* `syn`
* `quote`
* `proc_macro2`

### Projects

* `derive(Display)`
* `derive(Builder)`
* custom attribute macros
* compile-time validators

---

## 0x06 - Framework Projects

Framework-inspired compile-time systems.

### Topics

* API generation
* routing systems
* compile-time validation
* framework abstractions
* backend DSLs

### Projects

* mini router
* mini serde
* mini actix
* compile-time API generator

---

# Technologies Used

## Language

* Rust

## Tooling

* Cargo
* Cargo Workspaces

## Macro Systems

* `macro_rules!`
* procedural macros

## Libraries

* `syn`
* `quote`
* `proc_macro2`

---

# Recommended Prerequisites

Before starting this repository, you should understand:

* Rust ownership and borrowing
* functions and modules
* enums and structs
* traits and generics
* pattern matching
* iterators and collections

---

# Running Projects

## Clone Repository

```bash
git clone <repository-url>
cd alx-rust-macros
```

---

## Build Entire Workspace

```bash
cargo build
```

---

## Run Specific Project

Example:

```bash
cargo run -p hello_macro
```

---

## Run Tests

```bash
cargo test
```

---

# Debugging Macros

This repository heavily encourages learning macro debugging techniques.

## Useful Tools

### cargo-expand

Install:

```bash
cargo install cargo-expand
```

Use:

```bash
cargo expand
```

---

## Trace Macro Expansion

```rust
trace_macros!(true);
```

---

## Print Generated Syntax

```rust
log_syntax!();
```

---

# Engineering Philosophy

This repository is designed around the idea that:

> Macro engineering is compiler engineering.

The goal is not only to write macros, but to:

* understand how Rust parses syntax
* think in tokens and expansions
* design safe compile-time systems
* create maintainable abstractions
* build framework-level tooling

---

# Real-World Inspiration

The concepts explored here are inspired by systems used in:

* Meta Platforms infrastructure tooling
* Google compiler systems
* Amazon backend infrastructure
* Microsoft systems engineering

as well as Rust ecosystem projects like:

* Serde
* Tokio
* Axum
* Actix Web
* SQLx

---

# Documentation

Additional notes and explanations can be found inside the `docs/` directory.

Topics include:

* macro expansion
* token trees
* recursion
* hygiene
* procedural macro architecture
* DSL design patterns

---

# Contribution

Contributions are welcome.

To contribute:

1. Fork the repository
2. Create a feature branch
3. Implement improvements
4. Add documentation and tests
5. Submit a pull request

---

# License

This project is open-source and available under the MIT License.

---

# Author

ALX Rust Macro Engineering Lab

Focused on systems programming, metaprogramming, compiler-style engineering, and Rust infrastructure development.
