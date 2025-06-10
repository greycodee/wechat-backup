use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::common::utils::change_file_extension;


/**
    ffmpeg 用于语音转码

    微信语音 -> [SILK_V3 Decoder] -> PCM -> [ffmpeg] -> MP3

*/
#[allow(dead_code)]
pub fn verify_ffmpeg_install() -> Result<(), Error> {
    Command::new("ffmpeg").arg("-version").output()?;
    Ok(())
}

fn transcode_media(input: &Path, output: &Path) -> std::io::Result<()> {
    let ffmpeg_command = Command::new("ffmpeg")
        .arg("-y") // 覆盖输出文件
        .arg("-f")
        .arg("s16le") // 输入文件格式
        .arg("-ar")
        .arg("24000") // 采样率
        .arg("-ac")
        .arg("1") // 声道数量
        .arg("-i")
        .arg(input) // 输入PCM文件
        .arg(output) // 输出MP3文件
        .output()?;
    // 检查命令的输出结果
    if !ffmpeg_command.status.success() {
        return Err(Error::new(std::io::ErrorKind::Other, "Transcoding failed!"));
    }
    Ok(())
}

pub fn transcode_pcm_to_mp3(input_pcm: &Path) -> std::io::Result<PathBuf> {
    let output_mp3 = change_file_extension(input_pcm, "mp3");
    match transcode_media(input_pcm,&output_mp3) {
        Ok(_) => {
            Ok(output_mp3)
        }
        Err(e) => {
            Err(e)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_verify_ffmpeg_install() {
        match verify_ffmpeg_install() {
            Ok(_) => {
                println!("ffmpeg found!");
            }
            Err(e) => {
                panic!("ERR: {}", e);
            }
        }
    }

    #[test]
    fn test_transcode_pcm_to_mp3() {
        let pcm_file_path = Path::new("/Volumes/hkdisk/wechat-backup/20241117/wxid_jafjkmbud9l912/voice2/e6/17/msg_3219401122221c8bfd467b8103.pcm");
        match transcode_pcm_to_mp3(pcm_file_path) {
            Ok(_mp3) => {
                println!("transcode success!");
            }
            Err(e) => {
                panic!("ERR: {}", e);
            }
        }
    }
}
