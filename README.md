# Rust Template
Template for a rust CLI application with generated GUI
## About the Template

Replace this section with "About `project name`"

This template sets up a rust project using rstest for unit testing, and `cargo-wix` for building an installer for windows

## Getting Started

### Setup

### Windows

- [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
	- See rust install [instructions](https://www.rust-lang.org/tools/install)
- Rustup
	- `winget install Rustlang.Rustup`
- pre-commit
	- `winget install Python.Python.3.11` (or latest python available)
	- `pip install pre-commit`
	- `pre-commit install --install-hooks`

### Testing

```bash
cargo test
```

### Building Executable

```bash
cargo build --release
```

### Building the Windows Installer

Cargo wix does a good job of generating a good installer out of the box, so this template will not provide a `.wxs` file.  Feel free to look at the [python template](https://github.com/drbartling/python_template) for an example of another wxs file.

- Install build dependencies
	- Windows: `choco install wixtoolset`
- Define the installer
	- `cargo wix init`
- Build the installer
	- `cargo wix`
- Commit the new installer definition
	- `git commit -a`

## Using the Template

Modify the getting started section for your specific application.  Update the rest of the template mostly by searching for "hello" and replacing it with what's appropriate for your application.

## Template Goal

Most people and organisations have figured out hot to implement the features customers are requesting.  But often "doing things the right way" is deferred until it becomes nearly impossible to ever do things the "right way".

I want to make it easy to start a new project with tests, documentation, installers, etc.  Everything that makes it easier to get those features out the door and delivered relatively stress free.

I don't think this template does that for a oython project, yet, but it puts a few things in place and will make it easier for people I know to start a new python project and deliver it to co-workers, customers, freinds, and family.

Hope it helps.

## Feedback
Create issues or pull requests if you think there's a way to improve it.
