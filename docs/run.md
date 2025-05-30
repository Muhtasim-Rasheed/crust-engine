If you want to run Crust projects, you need to have the Crust engine installed on your system. You can only build it from source at the moment.

## Building Crust

Firstly, clone the Crust repository from GitHub:

```bash
git clone https://github.com/Muhtasim-Rasheed/crust.git
cd crust
```

To build Crust, you need to have the Rust toolchain installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can build and install Crust by running the following command in the terminal:

```bash
cargo install --path .
```

This will compile Crust and install it on your system. Make cargo can be found in your PATH, so you can run the `crust` command from anywhere in your terminal.

## Running your Crust Project

Once you have Crust installed, you can run your project using the following command in your terminal:

=== "Linux / macOS / Windows (WSL)"
    ```bash
    crust --project path/to/your_project.toml
    ```
=== "Windows"
    ```batch
    crust --project path\to\your_project.toml
    ```

Replace the path with the path to your project's `project.toml` file. This will start the Crust engine and run your project.
If you don't mention the path to the `project.toml` file, Crust will open a file picker dialog for you.

For other commands, you can run `crust --help` or `crust -h` to see the available options and commands.
