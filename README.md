# Stripped Binaries

---

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