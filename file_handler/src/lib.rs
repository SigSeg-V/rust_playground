// This library is to demonstrate copying, moving, and zipping files
use std::path::Path;
use std::fs;
use std::io;
// copying multiple files to another location, if it fails we can handle it
pub fn copy_recursively(from: impl AsRef<Path>, to: impl AsRef<Path>, recursive_copy: Option<bool>) -> io::Result<()> {
    fs::create_dir_all(&to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() && recursive_copy.unwrap_or(false) {
            copy_recursively(entry.path(), to.as_ref().join(entry.file_name()), recursive_copy)?;
        } else {
            fs::copy(entry.path(), to.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}