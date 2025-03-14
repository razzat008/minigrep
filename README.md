    small implmentation of the grep utility in rust

# Setup
```bash
git clone https://github.com/razzat008/minigrep
cd minigrep
```

## Usage
- normal search 
```bash
cargo run -- <your_string> <your_file>
```
something like
```bash
cargo run -- they poem.txt
```

- override default behaviour
```bash
IGNORE_CASE=1 cargo run they poem.txt
```
