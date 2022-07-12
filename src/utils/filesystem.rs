use std::{
    fs::{self, File},
    io,
};

use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use zip::result::ZipError;
use zip::write::FileOptions;

use std::path::Path;
use walkdir::{DirEntry, WalkDir};

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

pub fn tar_dir(
    tar_path: impl AsRef<std::path::Path>,
    dir_path: impl AsRef<std::path::Path>,
) -> io::Result<()> {
    let file = File::create(tar_path)?;
    let mut archive_builder = tar::Builder::new(file);
    archive_builder.append_dir_all(".", dir_path)
}
pub fn do_zip_dir(
    ar_path: &str,
    dir_path: &str,
) -> zip::result::ZipResult<()> {

    doit(dir_path, ar_path, zip::CompressionMethod::Zstd)
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

fn doit(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}
