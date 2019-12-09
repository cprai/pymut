# PyMut
### Author: Chuck Rai
### Built using: https://github.com/RustPython/RustPython

## Overview

Build executable to `./target/debug/pymut`:
```
cargo build
```

Usage:
```
pymut -m <mode> -d <database> -f <filename>
```

Mode is one of:
```
Explore
Execute
```

Database is a filename of an sqlite3 .db file

In explore mode, filename is the name of the file to apply mutations to

In execute mode, filename is the name of the file to execute

## Example Usage

`library.py`:
```
def add(a, b):
    return a + b
```

`test.py`:
```
from library import add
assert add(3, 4) == 7
```

Explore possible mutations in `library.py` and save results to `mutations.db`:
```
pymut -m Explore -d mutations.db -f library.py
```

Iterate over every mutation in `mutations.db` and execute `test.py` for each one:
```
pymut -m Execute -d mutations.db -f test.py
```

View results:
```
sqlite3 -column mutations.db "select * from results;"
```

## Results Format

Format of the results table:
```
Column 1: SHA-1 hash of the mutated file
Column 2: Location of mutation within file
Column 3: String representation of applied mutation
Column 4: SHA-1 hash of file used to execute test
Column 5: Execution result
```

Possible execution results:
```
Success: Mutated program executed and terminated successfully
RuntimeError: Mutated program ran into an error while executing
Timeout: Mutated program took too long and was terminated
```

## Run tests

Run all tests (from base directory):
```
./run_all_tests.sh
```

Run tests individually (from a directory inside of `./tests`):
```
export PYMUT_PATH=/path/to/built/pymut/executable
./run.sh
```