use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[cfg(unix)]
fn is_executable(path: &Path, _path_ext: &OsStr) -> std::io::Result<bool> {
    use libc::{access, X_OK};
    let cstr = std::ffi::CString::new(path.as_os_str().as_bytes().to_vec())?;
    let res = unsafe { access(cstr.as_ptr(), X_OK) };
    Ok(res == 0)
}

#[cfg(windows)]
fn is_executable(path: &Path, path_ext: &OsStr) -> std::io::Result<bool> {
    let path_string = path.display().to_string();
    let path_ext = std::env::split_paths(path_ext);
    let mut has_extension = false;
    for extension in path_ext {
        if path_string.ends_with(&extension.display().to_string()) || path_string.ends_with(&extension.display().to_string().to_lowercase()) {
            has_extension = true;
            break;
        }
    }
    Ok(has_extension)
}

/**
 * @param path The path variable. On linux PATH, on windows path
 * @param path_ext For windows, executables extensions. pathext enviroment variable
 */
pub fn search_path(command: &str, path: Option<&OsStr>, path_ext: Option<&OsStr>) -> Vec<String> {
    let path = path.unwrap_or_else(|| OsStr::new(""));
    let path_ext = path_ext.unwrap_or_else(|| OsStr::new(""));
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
                            if let Ok(true) = is_executable(&entry.path(), path_ext) {
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
