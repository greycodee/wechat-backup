pub mod file {
    use std::fs::File;
    use std::io;
    use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
    use std::path::Path;
    use tar::Archive;
    use zip::ZipArchive;

    const START_HEADER: &str = "apps/";

    fn find_capture_position(file: &mut File) -> io::Result<u64> {
        let mut window = vec![0; START_HEADER.len()];
        let mut position: u64 = 0;
        loop {
            if position > 200 {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "wechat backup flag not found!",
                ));
            }
            file.seek(SeekFrom::Start(position))?;
            file.read_exact(&mut window)?;
            if window == START_HEADER.as_bytes() {
                file.seek(SeekFrom::Start(position))?;
                return Ok(position);
            }
            position += 1;
        }
    }

    fn file_extract(file: File, out_dir: &Path) -> io::Result<()> {
        // let total_size = file.metadata()?.len();
        let mut archive = Archive::new(file);

        // let mut unpacked_size = 0;
        for entry in archive.entries()? {
            let mut entry = entry?;
            // let entry_size = entry.header().size()?;
            entry.unpack_in(out_dir)?;

            // unpacked_size += entry_size;
            // let progress = (unpacked_size as f64 / total_size as f64) * 100.0;
            // println!("Progress: {:.2}%", progress);
        }
        Ok(())
    }

    pub fn unpack_android_backup(android_backup_file: &Path, output_dir: &Path) -> io::Result<()> {
        if !android_backup_file.exists() {
            return Err(Error::new(ErrorKind::NotFound, "file not found!"));
        }
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)?;
        }
        let mut file = File::open(android_backup_file)?;
        let _ = find_capture_position(&mut file)?;
        file_extract(file, output_dir)?;
        Ok(())
    }

    pub fn unpack_zip_file(zip_file_path: &Path, output_dir: &Path) -> std::io::Result<()> {
        let file = File::open(zip_file_path)?;
        let mut archive = ZipArchive::new(file)?;
        let archive_len = archive.len();
        for i in 0..archive_len {
            let mut file = archive.by_index(i)?;
            let file_name = file.name();
            let mut out_path = output_dir.to_path_buf();
            if let Some(stripped_file_name) = file_name.strip_prefix('/') {
                out_path.push(stripped_file_name);
            } else {
                out_path.push(file_name);
            }

            if file_name.ends_with('/') {
                std::fs::create_dir_all(out_path)?;
            } else {
                if let Some(p) = out_path.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = File::create(&out_path)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
            // print progress
            // let progress = (i as f64 / archive_len as f64) * 100.0;
            // println!("Progress: {:.2}%", progress);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    #[test]
    fn test_unzip_file() {
        let zip_file_path = Path::new("/Users/zheng/Downloads/20241024_091952/backup_wechat.zip");
        let out_put_dir = Path::new("/Users/zheng/Downloads/20241024_091952/android_test");

        match crate::miui_backup::android_backup::file::unpack_zip_file(zip_file_path, out_put_dir) {
            Ok(_) => {
                println!("unzip success");
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_unpack_android_backup() {
        let android_backup_file = Path::new("/Users/zheng/Downloads/20241024_091952/wechat.bak");
        let output_dir = Path::new("/Users/zheng/Downloads/20241024_091952/android_test");
        match crate::miui_backup::android_backup::file::unpack_android_backup(
            android_backup_file,
            output_dir,
        ) {
            Ok(_) => {
                println!("unpack success");
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
