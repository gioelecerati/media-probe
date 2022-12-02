# Media Probe

Basic media probe using ffprobe.
Scan a folder and generate a `JSON`, `xml` or `csv` with informations about the media files in it.

### Build

Install Rust
```
cargo build --release
```

### Usage

```
cp target/release/media-probe /usr/local/bin
```

```
media-probe --folder /home/gioele/Videos --simplified --format csv
```

### Options

```
Usage: media-probe [OPTIONS]

Options:
  -f, --format <FORMAT>  [default: json] [possible values: xml (not implemented yet), json, csv (only simplified)]
      --folder <FOLDER>  [default: .]
      --output <OUTPUT>  [default: output]
  -r, --recursive        
  -s, --simplified       Generate basic informations about the media files. [default: false] 
  -h, --help             Print help information
  -V, --version          Print version information
```



