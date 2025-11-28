# Accounting Spec Implementation

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.91.1%2B-blue.svg)](https://www.rust-lang.org)


This project is a Rust implementation of an accounting specification based on Domain-Driven Design (DDD) principles. It handles sales registration, transfers between sections, term management, and end-of-term corrections.

## Features

- **Section Management**: Create organizational units (Department, Division, Section).
- **Term Management**: Manage accounting periods (Open/Closed).
- **Sales Management**:
    - Register new sales.
    - Transfer sales between sections (with audit trail).
    - Adjust sales amounts.
- **Corrections**: Handle end-of-term discrepancies with balancing entries.

## Architecture

The project follows a Clean Architecture / DDD approach:

- **`src/domain`**: Contains the core business logic, entities (`Section`, `Term`, `Sales`), and value objects (`Money`). It defines repository traits but has no external dependencies on infrastructure.
- **`src/application`**: Contains the application services (`AccountingService`) that orchestrate the domain objects to fulfill use cases.
- **`src/infrastructure`**: Contains the concrete implementations of repositories (currently in-memory `HashMap` storage).
- **`src/main.rs`**: The entry point that demonstrates the application flow.

## Prerequisites

- Rust 1.91.1 or later (Note: Dependencies in `Cargo.toml` are pinned for compatibility with older Rust versions if needed).

## Usage

To run the demonstration scenario:

```bash
cargo run
```

This will execute a sequence of operations:
1. Create Sections and a Term.
2. Register a Sale.
3. Transfer the Sale to another Section.
4. Close the Term.
5. Perform a post-closing correction (rebalancing).

## Documentation

- [Domain Model](docs/domain_model.md) (Deleted in previous step, but conceptually relevant)
- [Action Model](docs/action_model.md) (Deleted in previous step, but conceptually relevant)
- [Accounting Spec](docs/accounting_spec.md) (Deleted in previous step, but conceptually relevant)

## License

[MIT](LICENSE)
