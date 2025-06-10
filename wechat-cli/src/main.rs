use clap::{Command, Arg};
use wechat_saver_lib::neo_backup;
use wechat_saver_lib::new_miui_backup;

// ./wechat-cli --workspace "/Volumes/hkdisk/neo-backup-dagongren/workspace3" --data "/Volumes/hkdisk/neo-backup-dagongren/data.tar.gz" --external "/Volumes/hkdisk/neo-backup-dagongren/external_files.tar.gz" --endpoint "http://192.168.6.115:9012"

#[tokio::main]
async fn main(){
    let matches = Command::new("Wechat Backup")
        .version("1.0")
        .author("Author Name <author@example.com>")
        .about("Performs a quick backup of Wechat data")
        .arg(Arg::new("workspace")
            .short('w')
            .long("workspace")
            .value_name("WORKSPACE")
            .help("Sets the workspace directory")
            .value_parser(clap::value_parser!(String))
            .required(true))
        .arg(Arg::new("data_tar_gz_file")
            .short('d')
            .long("data")
            .value_name("DATA_TAR_GZ_FILE")
            .help("Sets the data tar.gz file")
            .value_parser(clap::value_parser!(String))
            .required(true))
        .arg(Arg::new("external_tar_gz_file")
            .short('e')
            .long("external")
            .value_name("EXTERNAL_TAR_GZ_FILE")
            .help("Sets the external tar.gz file")
            .value_parser(clap::value_parser!(String))
            .required(true))
        .arg(Arg::new("endpoint")
            .short('p')
            .long("endpoint")
            .value_name("ENDPOINT")
            .help("Sets the endpoint URL")
            .value_parser(clap::value_parser!(String))
            .required(true))
        .arg(Arg::new("backup_type")
            .short('b')
            .long("backup-type")
            .value_name("BACKUP_TYPE")
            .help("Sets the type of backup (neo or miui)")
            .value_parser(["neo", "miui"])
            .required(true))
        .get_matches();

    let workspace = matches.get_one::<String>("workspace").unwrap();
    let data_tar_gz_file = matches.get_one::<String>("data_tar_gz_file").unwrap();
    let external_tar_gz_file = matches.get_one::<String>("external_tar_gz_file").unwrap();
    let endpoint = matches.get_one::<String>("endpoint").unwrap();
    let backup_type = matches.get_one::<String>("backup_type").unwrap();

    match backup_type.as_str() {
        "neo" => neo_backup::quick_backup(workspace, data_tar_gz_file, external_tar_gz_file, endpoint).await.unwrap(),
        "miui" => new_miui_backup::quick_backup(workspace, data_tar_gz_file, external_tar_gz_file, endpoint).await.unwrap(),
        _ => println!("Invalid backup type")
    }
}