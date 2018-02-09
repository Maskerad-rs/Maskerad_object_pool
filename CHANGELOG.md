# Maskerad Object Pool

#### Version 0.2.0
Refactored entirely the crate and the documentation, following the 
[Rust API guidelines](https://rust-lang-nursery.github.io/api-guidelines/about.html).

#### version 0.3.0
Added the log dependency.

Placed debug!, trace! and error! logs trough all the codebase. Debug! logs are placed at the beginning
of all public functions. Trace! logs are placed at the beginning of all private functions and
at various places of all functions.
