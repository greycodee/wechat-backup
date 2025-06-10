
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use tar::Archive;
use tokio::join;
use zip::ZipArchive;

const START_HEADER: &str = "apps/";

pub async fn unpack_backup_file(
    work_space: &Path,
    android_backup_file: &Path,
    android_sdcard_file: &Path,
) -> std::io::Result<PathBuf> {
    // 判断 android_backup_file 和 android_sdcard 是否存在
    if !android_backup_file.exists() {
        panic!("android_backup_file not exists");
    }
    if !android_sdcard_file.exists() {
        panic!("android_sdcard not exists");
    }
    // 判断work_space是否存在，不存在则创建
    if !work_space.exists() {
        std::fs::create_dir_all(work_space)?;
    }

    // 判断work_space 是否存在lock文件，存在则退出
    let lock_file = work_space.join("lock");
    if lock_file.exists() {
        let temp_dir_name = std::fs::read_to_string(&lock_file)?;
        let temp_dir = work_space.join(&temp_dir_name);
        return Ok(temp_dir);
    }

    let temp_dir_name = uuid::Uuid::new_v4().to_string();
    let temp_dir = work_space.join(&temp_dir_name);
    if !temp_dir.exists() {
        std::fs::create_dir_all(&temp_dir)?;
    }

    // 创建lock文件
    std::fs::File::create(&lock_file)?;
    std::fs::write(&lock_file, &temp_dir_name)?;

    let temp_dir_1 = temp_dir.clone();
    let temp_dir_2 = temp_dir.clone();

    let android_backup_file = android_backup_file.to_path_buf();
    let android_sdcard_file = android_sdcard_file.to_path_buf();

    let unpack_android_backup_task = tokio::spawn(async move {
        let res = unpack_android_backup(&android_backup_file, &temp_dir_1);
        match res {
            Ok(_) => {
                println!("unpack_android_backup success");
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    });

    let unpack_zip_file_task = tokio::spawn(async move {
        let res = unpack_zip_file(&android_sdcard_file, &temp_dir_2);
        match res {
            Ok(_) => {
                println!("unpack_zip_file success");
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    });

    match join!(unpack_android_backup_task, unpack_zip_file_task){
        (Ok(_), Ok(_)) => {
            println!("unpack_android_backup and unpack_zip_file success");
            Ok(temp_dir)
        }
        (Err(e1), Err(e2)) => {
            Err(Error::new(std::io::ErrorKind::Other, format!("unpack_android_backup error: {}, unpack_zip_file error: {}", e1, e2)))
        }
        (Err(e), _) => {
            Err(Error::new(std::io::ErrorKind::Other, format!("unpack_android_backup error: {}", e)))
        }
        (_, Err(e)) => {
            Err(Error::new(std::io::ErrorKind::Other, format!("unpack_android_backup error: {}", e)))
        }
    }

}

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

fn unpack_android_backup(android_backup_file: &Path, output_dir: &Path) -> io::Result<()> {
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

fn unpack_zip_file(zip_file_path: &Path, output_dir: &Path) -> std::io::Result<()> {
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
