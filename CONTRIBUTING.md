# Contributing

## Hall of Contributors

- [Muhtasim Rasheed](https://github.com/Muhtasim-Rasheed) - Creator and maintainer of Crust
- [P4ncake!](https://github.com/P4ncake4451) - Co-creator of Crust; implemented effects and designed the logo
- *Possibly you?*

## Dear potential contributor,

Thank you for your interest in contributing to Crust! We appreciate your time and effort. To ensure a smooth contribution process, please follow these guidelines:

1. **Fork the Repository**: Start by forking the repository to your own GitHub account. This allows you to make changes without affecting the original project.
2. **Clone Your Fork**: Clone your forked repository to your local machine using the command:
   ```bash
   git clone https://github.com/your-username/crust-engine.git
   ```
3. **Build the project**: Before making any changes, ensure that you can build the project successfully and also install all dependencies. Navigate to the project directory and run:
   ```bash
   cargo build
   ```
4. **You're ready to hack!**

## Things you can do

- Improve features
- Fix bugs
- Tweak/Improve the documentation
- Help split the big monolithic file `crust-engine/src/utils/sprite.rs` into smaller files
- Pick a [good first issue](https://github.com/Muhtasim-Rasheed/crust-engine/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22good%20first%20issue%22) or any other issue that interests you

Even tiny changes are welcome (Seriously)!

## Code style

Before commiting, run:

```sh
cargo fmt
```

This will format the codebase with `rustfmt`. Keeping things neat helps everyone.

## Commit messages

When committing changes, please use clear and descriptive commit messages. But silly commit messages are also welcome, as long as they are not too silly. For example:

- "Fix typo in README"
- "Add new feature to improve user experience"
- "Refactor code for better readability"
- "horror", "horror 2", "less horror", "no more horror"
- "sorry i added my test directory"
