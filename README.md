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
