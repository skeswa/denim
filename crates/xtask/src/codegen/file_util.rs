use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::CodegenCommands;

/// Checks that the `file` has the specified `contents`. If that is not the
/// case, updates the file and then fails the test.
#[allow(clippy::print_stderr)]
pub fn ensure_file_contents(
    command: &CodegenCommands,
    project_root: &PathBuf,
    file: &Path,
    contents: &str,
    check: bool,
) -> bool {
    let contents = normalize_newlines(contents);
    if let Ok(old_contents) = fs::read_to_string(file) {
        if normalize_newlines(&old_contents) == contents {
            // File is already up to date.
            return false;
        }
    }

    let display_path = file.strip_prefix(project_root).unwrap_or(file);
    if check {
        panic!(
            "{} was not up-to-date{}",
            file.display(),
            if std::env::var("CI").is_ok() {
                format!(
                    "\n    NOTE: run `cargo xtask {command}` locally and commit the updated files\n"
                )
            } else {
                "".to_owned()
            }
        );
    } else {
        eprintln!(
            "\n\x1b[31;1merror\x1b[0m: {} was not up-to-date, updating\n",
            display_path.display()
        );

        if let Some(parent) = file.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(file, contents).unwrap();
        true
    }
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}
