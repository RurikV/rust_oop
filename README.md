# Design Patterns in Rust

This repository demonstrates the implementation of two design patterns in Rust: the Decorator pattern and the Visitor pattern.


# Decorator Pattern Implementations in Rust

This repository contains two different implementations of the Decorator pattern in Rust: one using static dispatch and another using dynamic dispatch.

## Implementation Details

- `decorator_static_impl.rs`: Static implementation using Rust's type system and generics.
- `decorator_dyn_impl.rs`: Dynamic implementation using trait objects for runtime flexibility.

## Static Implementation

The static version (`decorator_static_impl.rs`) provides a compile-time approach to the Decorator pattern. It's characterized by its performance advantages due to the lack of runtime overhead typically associated with dynamic dispatch.

### Advantages
- No heap allocation
- No dynamic dispatch
- Compile-time type checking

## Dynamic Implementation

The dynamic version (`decorator_dyn_impl.rs`) allows for decorators to be applied and modified at runtime, giving more flexibility but at the cost of performance.

### Advantages
- Runtime flexibility
- Ability to modify decoration chain during execution

## Getting Started

To run the examples provided within each file:

```bash
cargo run --bin decorator_static_impl
cargo run --bin decorator_dyn_impl

# Visitor Pattern in Rust

## Overview

The Visitor pattern is a way of separating an algorithm from an object structure on which it operates. This is particularly useful for performing operations across a set of objects with different types.

## Implementation

In this repository, I provide two implementations of the Visitor pattern: static and dynamic.

- `visitor_static_impl.rs`: This file contains the static implementation of the Visitor pattern. The static approach utilizes Rust's powerful type system and compile-time checks to ensure type safety and efficiency.
  
- `visitor_dyn_impl.rs`: The dynamic implementation allows for more flexibility at the cost of runtime type checks. This is achieved by leveraging trait objects and the `Any` trait for downcasting.

## Usage

The Visitor pattern is utilized by defining a `Visitor` trait with a `visit` method for each type in the object structure. Each type implements an `accept` method that takes a `Visitor` and calls the `visit` method corresponding to its own type.

In the static implementation, Rust's generics and trait bounds are used to achieve polymorphism at compile time. In the dynamic implementation, trait objects are used to allow for different object types to be visited at runtime.

## Example

An example scenario included in this repository is shape analysis, where different geometric shapes (`Circle`, `Square`) are visited by a `ConcreteVisitor` that performs a specific operation, such as calculating the area or drawing the shape.

## Conclusion

The Visitor pattern is a powerful tool in Rust for creating operations that need to be performed across a variety of types without coupling the operations to the object structure.

