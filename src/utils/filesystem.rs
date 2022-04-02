use std::{
    fs::{self, File},
    io,
};

pub fn copy_dirs(
    src_path: impl AsRef<std::path::Path>,
    dst_path: impl AsRef<std::path::Path>,
) -> io::Result<()> {
    fs::create_dir_all(&dst_path)?;
    for entry in fs::read_dir(src_path)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        #[allow(unused_must_use)]
        {
            if ty.is_dir() {
                copy_dirs(entry.path(), dst_path.as_ref().join(entry.file_name()));
            } else {
                fs::copy(entry.path(), dst_path.as_ref().join(entry.file_name()));
            }
        }
    }
    Ok(())
}

pub fn zip_dir(
    tar_path: impl AsRef<std::path::Path>,
    dir_path: impl AsRef<std::path::Path>,
) -> io::Result<()> {
    let file = File::create(tar_path)?;
    let mut archive_builder = tar::Builder::new(file);
    archive_builder.append_dir_all(".", dir_path)
}
