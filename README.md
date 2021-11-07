A small unix and windows lib to search for executables in path folders.


Example:
```rs
use searchpath::search_path;
use std::ffi::OsString;

fn main() {
    let path = std::env::var_os("path");
    let path_ext = std::env::var_os("pathext");
    let files = search_path("explo", path.as_ref().map(OsString::as_os_str), path_ext.as_ref().map(OsString::as_os_str));
    for file in files {
        println!("{}", file);
    }
}
```
Will print something like
```
bat
bashbug
bash
base32
basenc
basename
base64
```