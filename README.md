Stripped Binaries
---

# Todo:
- Config the main.yml workflow to build the binary for linux;
- Compress the binary using UPX;
- Get the statistics of binary size and lines of code used;
- insert the statistics into the README.md;
- create a new release of the project with the zip source code and binaries;

use `run` to run the program

useful websites:
https://www.cs.montana.edu/courses/spring2005/518/Hypertextbook/vijay/syscalltable/37.htm
https://arm64.syscall.sh/

## Functions
|       | Linux aarch64 | Linux x86_64 | Windows | Windows on arm | MacOS |
|-------|---------------|--------------|---------|----------------|-------|
| exit  | yes           | yes          | yes     | no             | yes   |
| read  | yes           | yes          | no      | no             | no    |
| write | yes           | yes          | no      | no             | no    |
| exec  | yes           | not tested   | no      | no             | no    |
| kill  | not tested    | no           | no      | no             | no    |
