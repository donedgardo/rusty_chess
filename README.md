# Rusty Chess

Rusty Chess is an experimental project showcasing the application of Test-Driven Development (TDD) and Continuous Integration/Continuous Deployment (CI/CD) principles in the development of a chess game using Rust. The project explores how robust testing and automated workflows can streamline development and ensure high-quality software.

[Live Demo](https://donedgardo.itch.io/rusty-chess)
## Project Goals

The primary objectives of this project are:

1. **Practice TDD in Rust:** Applying TDD methodologies to build a chess game from the ground up.
2. **Establish CI/CD Pipelines:** Automating the development lifecycle using modern CI/CD practices.
3. **Promote Best Practices:** Demonstrating how TDD and CI/CD can be leveraged for efficient and reliable Rust development.

---

## Features

- **Chess Game Implementation:**
  - Core chess mechanics developed with a focus on correctness and maintainability.
  - Iterative development driven by comprehensive unit tests.

- **TDD Workflow:**
  - Tests written before functionality to ensure a fail-pass-refactor cycle.
  - Strict adherence to the TDD red-green-refactor process.

- **CI/CD Pipeline:**
  - Automated pipelines for testing, building, and deployment.
  - Integration with GitHub Actions for continuous feedback and quality assurance.
 
- **Polymorphic Design for Chess Pieces:**
  - Leveraged polymorphism to create a flexible and maintainable system for chess pieces.
  - Each piece inherits from a common base class, allowing consistent behavior while enabling unique functionality.
  - This approach simplifies the addition of new pieces or modifications to existing ones.

---

## Technologies Used

- **Rust:** Language used for game development.
- **Cargo:** Rust's package manager and build system.
- **GitHub Actions:** For CI/CD pipeline automation.


---

## Getting Started

### Prerequisites

- Rust (version X.X or later)
- Cargo installed

### Setup

1. Clone the repository:

    ```bash
    git clone https://github.com/donedgardo/rusty_chess.git
    cd rusty_chess
    ```

2. Build the project:

    ```bash
    cargo build
    ```

3. Run the tests:

    ```bash
    cargo test
    ```

---

## Running Tests

Unit tests are the backbone of the development process for Rusty Chess. To execute the tests:

```bash
cargo test
```

The tests include:

- **Game Mechanics:** Validations for chess rules and logic.
- **Edge Cases:** Coverage for scenarios such as stalemates and checkmates.

---

## CI/CD with GitHub Actions

### Pipeline Overview

The GitHub Actions workflow includes:

1. **Code Checkout:** Pulls the latest code from the repository.
2. **Build:** Ensures the project compiles successfully.
3. **Tests:** Runs unit tests to validate functionality.
4. **Deployment:** Deploys the application (future enhancement).

### Example GitHub Actions Workflow

```yaml
name: CI/CD Pipeline

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main

jobs:
  build-and-test:
    runs-on: ubuntu-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v3

    - name: Set up Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Build
      run: cargo build --verbose

    - name: Test
      run: cargo test --verbose
```

---

## Future Enhancements

- Expand game logic to include advanced chess features such as en passant and pawn promotion.
- Improve CI/CD pipeline with deployment and notification steps.
- Explore integration testing with a simple UI or command-line interface.

---

## Contributing

Contributions are welcome! If you have ideas or improvements, feel free to open an issue or submit a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- Rust community for excellent tooling and documentation.
- Inspiration from TDD and CI/CD best practices in modern software development.
