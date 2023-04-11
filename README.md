# py03-experiment

Solving advent of code 2022 day 19 in rust+python

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

use `-r` for a release build

this will isntall the module in the current environment

## run the python code

```
python 2022ex19.py
```

## compile and run

```
maturin develop -r && python 2022ex19.py
```