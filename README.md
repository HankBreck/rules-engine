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

To build the project you must have `cargo-run-scripts` installed. You can do this by running the command below.

```shell
cargo install cargo-run-script
```

You can then build the Python module by running:

```shell
cargo run-script build-python
```