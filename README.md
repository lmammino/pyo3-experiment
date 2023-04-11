# pyo3-experiment

## Advent of Code 2022 Day 19 Solution in Rust and Python

This repository contains the solution for Advent of Code 2022 Day 19 in Rust and Python. The code is organized in the `src` directory, which contains the rust source code, along with the `Cargo.toml` and `pyproject.toml` files that define the dependencies for the Rust and Python code respectively.

### Requirements

* Rust
* Python 3
* Maturin (can be installed via pip)

## init

```
python -m venv .env   
source .env/bin/activate
pip install maturin
```

## build the rust library

```
maturin develop 
```

This will install the module in the current environment. Use the -r option for a release build.

## run the python code

```
python 2022ex19.py
```

## compile and run

```
maturin develop -r && python 2022ex19.py
```

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue. If you would like to contribute code, please fork the repository and submit a pull request.