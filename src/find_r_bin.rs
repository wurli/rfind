use glob::glob;
use pathsearch::find_executable_in_path;
use std::env;
use std::path::{Path, PathBuf};

pub fn find_r_bin() -> Option<PathBuf> {
    // If R is on the path, and the file exists, return the path
    if let Some(path) = dir_from_path_envvar() {
        return Some(path);
    }

    // If the R_HOME env var is set, use that, if the file exists
    if let Some(path) = dir_from_r_home_envvar() {
        return Some(path);
    }

    if let Some(path) = dir_from_common_loc() {
        return Some(path)
    }

    return None;
}

pub fn dir_from_path_envvar() -> Option<PathBuf> {
    if let Some(path) = find_executable_in_path("R") {
        if Path::new(&path).exists() {
            return Some(path);
        }
    }
    return None;
}

pub fn dir_from_r_home_envvar() -> Option<PathBuf> {
    if let Ok(path) = env::var("R_HOME") {
        let mut buf = PathBuf::new();
        buf.push(&path);
        if Path::new(&buf).exists() {
            return Some(buf);
        }
    }
    None
}

pub fn dir_from_common_loc() -> Option<PathBuf> {
    let locations = vec![
        "/opt/local/bin/R",                                                 // macOS standard location
        "/Library/Frameworks/R.framework/Versions/current/Resources/bin/R", // Common alternative on macOS
        "/usr/bin/R",                                                       // Linux standard location
        "/usr/local/bin/R",                                                 // Common alternative on Linux
        "opt/bin/R",                                                        // Common alternative on Linux
        "C:/Program Files/R/R-*/bin/R",                                     // Windows
    ];

    for loc in locations {
        let mut files = glob(&loc).unwrap().filter_map(Result::ok);
        let path = files.next();
        if let Some(file) = path {
            return Some(file)
        }
    }
    None
}


#[cfg(test)]
mod tests {

    use std::{env, fs, path::PathBuf};

    use crate::find_r_bin;

    #[test]
    fn dir_from_r_home_envvar_produces_none_correctly() {
        env::set_var("R_HOME", "");
        assert_eq!(find_r_bin::dir_from_r_home_envvar(), None);
    }

    #[test]
    fn dir_from_r_home_envvar_produces_path_correctly() {
        // Create a dummy file in temp location since the dir is only returned
        // if the R executable actually exits 
        let mut tempfile = env::temp_dir();
        tempfile.push("fake-file.txt");
        let _ = fs::write(&tempfile, b"Some text");

        env::set_var("R_HOME", &tempfile);
        assert_eq!(find_r_bin::dir_from_r_home_envvar(), Some(tempfile));
    }

    #[test]
    fn dir_from_path_envvar_produces_none_correctly() {
        // Haven't yet figured out how to effectively mock the PATH variable
        assert_eq!(find_r_bin::dir_from_path_envvar(), Some(PathBuf::from("/usr/local/bin/R")));

        env::set_var("PATH", "");
        assert_eq!(find_r_bin::dir_from_path_envvar(), None)
    }

}
