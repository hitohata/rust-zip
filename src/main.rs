use zip::{self, write::SimpleFileOptions};
use std::io::Write;

fn main() {
    let dir_name = "./upload-file-zip";
    let file_name = format!("{dir_name}/fuga.txt");
    let key_name = "existing.zip";
    std::fs::create_dir(dir_name).unwrap();
    let mut file = std::fs::File::create(file_name.to_owned()).unwrap();
    file.write_all(b"Hello, world!").unwrap();


    let options = SimpleFileOptions::default()
        .compression_method()
        .unix_permissions(0o755);

    let dist_path = std::path::Path::new(&file_name);
    let zip_file = std::fs::File::create(key_name).unwrap();
    let mut zip = zip::ZipWriter::new(zip_file);

}
