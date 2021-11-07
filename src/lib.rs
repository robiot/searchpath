use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[cfg(unix)]
fn is_executable(path: &Path) -> std::io::Result<bool> {
    use libc::{access, X_OK};
    let cstr = std::ffi::CString::new(path.as_os_str().as_bytes().to_vec())?;
    let res = unsafe { access(cstr.as_ptr(), X_OK) };
    Ok(res == 0)
}

fn is_executable_platform(path: &Path) -> std::io::Result<bool> {
    #[cfg(unix)]
    return is_executable(path);
    #[cfg(windows)]
    return Ok(true);
}

pub fn search_path(command: &str, path: Option<&OsStr>) -> Vec<String> {
    let path = path.unwrap_or_else(|| OsStr::new(""));
    let mut path_iter = std::env::split_paths(path);

    let mut files = vec![];
    loop {
        if let Some(dir) = path_iter.next() {
            if !dir.exists() {
                break;
            }
            if let Ok(read_dir) = dir.read_dir() {
                for entry in read_dir.flatten() {
                    if let Some(s) = entry.file_name().to_str() {
                        if s.starts_with(command) {
                            if let Ok(true) = is_executable_platform(&entry.path()) {
                                files.push(s.to_string());
                            }
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
    files
}
