# minigrep

```bash
# Problem parsing arguments: not enough arguments
cargo run xxx

# Application error: No such file or directory (os error 2)
cargo run xxx poem.txtx

# ok
cargo run xxx poem.txt

# 0: I'm nobody! Who are you?
# 1: Are you nobody, too?
# 2: How dreary to be somebody!
cargo run body poem.txt

# 0: How dreary to be somebody!
# 1: How public, like a forg
CASE_INSENSITIVE=1 cargo run how poem.txt
```
