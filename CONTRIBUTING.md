# Contributing to `minmath`

Thank you for considering contributing to `minmath`! Your help is greatly appreciated, whether it's through fixing bugs, adding features, improving documentation, or providing feedback.

## 📚 Table of Contents

- [🎯 Project Goals](#project-goals)
- [🐞 Reporting Issues](#reporting-issues)
- [📄 Licensing](#licensing)
- [🤝 Need Help?](#need-help)
- [🛠️ How to Contribute](#how-to-contribute)
- [💬 Commit Message Style](#commit-message-style)

---

## Project Goals

`minmath` aims to provide minimal, efficient, and well-tested mathematical utilities for Rust. We're especially focused on:
- Simplicity and clarity over complexity
- Performance and correctness
- No unnecessary dependencies

If your contribution aligns with this philosophy, it's likely to be a good fit!

## Reporting Issues

If you find a bug or have a feature idea, please open an issue and include:
- A clear description
- Steps to reproduce (if it's a bug)
- Expected behavior
- Relevant code or stack trace, if any

## Licensing

By contributing to this repository, you agree that your contributions will be licensed under the same license as the project: [LICENSE.md](LICENSE.md).

## Need Help?

If you're unsure how to contribute or want to chat about ideas, feel free to open a discussion or reach out via GitHub Issues.

## How to Contribute

1. **Fork** this repository and **clone** it to your local machine:
    ```bash
    git clone https://github.com/jodus-melodus/minmath.git
    cd minmath
    ```

2. Create a new branch for your contribution:
    ```bash
    git checkout -b feature/my-new-feature
    ```

3. Make your contribution. Please follow Rust style guidelines and keep the code clean and idiomatic.  
   Run `cargo fmt` and `cargo clippy` to format and lint your code before committing.

4. Write tests to cover your changes:
    - Add unit tests.
    - Run all tests with:
        ```bash
        cargo test
        ```

5. Commit your changes with a clear message:
    ```bash
    git add .
    git commit -m "Describe your changes here"
    ```

6. Push to your **fork**:
    ```bash
    git push origin feature/my-new-feature
    ```

7. Create a pull request on GitHub and describe your changes.

_If your change is large or you want to discuss it beforehand, feel free to open an issue first!_

### Thank you!


## Commit Message Style

Try to keep commit messages clear and concise. For example:
- `Fix: incorrect matrix multiplication`
- `Add dot product for 3D vectors`
- `Refactor: simplify angle normalization`
