# minigrep

```bash
cargo build --release

cd target/release

# run
# CASE_INSENSITIVE: ignore case
CASE_INSENSITIVE=1 ./minigrep

./minigrep <query> <filename> [> <output_filename>]

# example
CASE_INSENSITIVE=1 cargo run Let poem.txt
```
