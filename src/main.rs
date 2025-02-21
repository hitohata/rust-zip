use std::{
    fs::File,
    io::{Read, Write},
};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use zip::{self, unstable::write, write::SimpleFileOptions};

fn main() {
    let dir_name = "./upload-file-zip";
    let key_name = "existing.zip";
    std::fs::create_dir_all(dir_name).unwrap();

    let zip_file = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(true).open(key_name).unwrap();
    let zip_writer = zip::ZipWriter::new(zip_file);

    let zip = Arc::new(Mutex::new(zip_writer));

    for i in ["a", "b", "c", "d"] {
        write_file(i, zip.clone());
    }

    if let Ok(mutex) = Arc::try_unwrap(zip) {
        let value = mutex.into_inner().unwrap();
        value.finish().unwrap();
    } else {
        panic!("error")
    };

}

fn write_file(file_name: &str, zip: Arc<Mutex<zip::ZipWriter<File>>>) {
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    let contents = format!("Contents, {}", file_name);

    let buffer = contents.as_bytes();

    // let mut zip = zip::ZipWriter::new_append(zip_file).unwrap();

    let mut zip = zip.lock().unwrap();

    zip.start_file(file_name, options)
        .unwrap();

    println!("{:?}: {:?}", file_name, &buffer);

    zip.write_all(buffer).unwrap();
}
