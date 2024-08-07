use super::*;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_copy_file() {
    let dir = tempdir().unwrap();
    let src_path = dir.path().join("source.txt");
    let dst_path = dir.path().join("destination.txt");

    // Create a source file with some content
    let mut src_file = File::create(&src_path).unwrap();
    writeln!(src_file, "Hello, world!").unwrap();

    // Copy the file
    copy_file(src_path.to_str().unwrap(), dst_path.to_str().unwrap()).unwrap();

    // Read the destination file and verify its content
    let mut dst_file = File::open(&dst_path).unwrap();
    let mut content = String::new();
    dst_file.read_to_string(&mut content).unwrap();

    assert_eq!(content, "Hello, world!\n");
}

#[test]
fn test_copy_empty_file() {
    let dir = tempdir().unwrap();
    let src_path = dir.path().join("empty_source.txt");
    let dst_path = dir.path().join("empty_destination.txt");

    // Create an empty source file
    File::create(&src_path).unwrap();

    // Copy the file
    copy_file(src_path.to_str().unwrap(), dst_path.to_str().unwrap()).unwrap();

    // Read the destination file and verify its content
    let mut dst_file = File::open(&dst_path).unwrap();
    let mut content = String::new();
    dst_file.read_to_string(&mut content).unwrap();

    assert_eq!(content, "");
}

#[test]
fn test_copy_non_existent_file() {
    let dir = tempdir().unwrap();
    let src_path = dir.path().join("non_existent.txt");
    let dst_path = dir.path().join("destination.txt");

    // Attempt to copy a non-existent file
    let result = copy_file(src_path.to_str().unwrap(), dst_path.to_str().unwrap());

    assert!(result.is_err());
}
