# Rules Engine

## Getting started

### Python

You must activate the venv to start.

```shell
source .env/bin/activate
```

You may then install the dependencies by running:

```shell
pip install -r requirements.txt
```

### Rust

If you do not have it installed already you must install Cargo. Check the [Rust Documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html) for instructions on how to do this

To build the project you must have `cargo-run-scripts` installed. You can do this by running the command below.

```shell
cargo install cargo-run-script
```

You can then build the Python module by running:

```shell
cargo run-script build-python
```
You can execute the tests from zeroSteiner's [Python rule engine](https://github.com/zeroSteiner/rule-engine/) by running the following command
```shell
cargo run-script test-python
```
