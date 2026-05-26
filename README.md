# BufferBox
## Simple utility to paste binaries (images) or text from clipboard to files

## Main development at [gitlab](https://www.gitlab.com/SileboxUnderfined/buffer_box)

## Usage
### Without argument:
```bash
buffer_box
```

### With argument:
```bash
buffer_box /path/to/dst/filename.[txt,png,jpg,etc]
```

## Building
1. install [rust](https://www.rust-lang.org)
2. clone repo
```bash
git clone https://gitlab.com/SileboxUnderfined/buffer_box.git
```
3. go to directory
```bash
cd buffer_box
```
4. build
```bash
RUSTFLAGS="-C target_cpu=native -Zlocation-detail=none -Zfmt-debug=none" cargo build --release
```