use std::{
    fs::File,
    io::{Read, Write},
};
use zip::{self, write::SimpleFileOptions};

fn main() {
    let current_dir = std::env::current_dir().unwrap();

    let dir_name = "./upload-file-zip";
    let file_name = format!("{dir_name}/fuga.txt");
    let key_name = "existing.zip";
    std::fs::create_dir(dir_name).unwrap();
    let mut file = std::fs::File::create(file_name.to_owned()).unwrap();
    file.write_all(b"Hello, world!").unwrap();

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    let mut buffer: Vec<u8> = Vec::new();

    let dist_path = std::path::Path::new(&file_name);
    let zip_file = std::fs::File::create(key_name).unwrap();
    let mut zip = zip::ZipWriter::new(zip_file);

    zip.start_file("upload-file-zip/fuga.txt", options).unwrap();

    let mut f = std::fs::File::open(file_name).unwrap();
    f.read_to_end(&mut buffer).unwrap();
    zip.write_all(&buffer).unwrap();
    buffer.clear();

    zip.finish().unwrap();
}
