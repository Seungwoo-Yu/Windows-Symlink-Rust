use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;
use windows_symlink::{symlink, SymlinkFlag};
use crate::tests::TestEnv;

pub fn test1() {
    let mut test_env = TestEnv {
        used_paths: vec![]
    };

    (&mut test_env).used_paths.push(PathBuf::from("windows-symlink-tests"));
    let root_path = (&test_env).used_paths.get(0).unwrap().clone();
    fs::create_dir_all(&root_path)
        .expect("Couldn't create test folder, windows-symlink-tests.");

    (&mut test_env).used_paths.push(root_path.join("copy_me.txt"));
    let target_path = (&test_env).used_paths.get(1).unwrap().clone();
    fs::write(&target_path, "Hehe yes! This is it!")
        .expect("Couldn't create target file.");

    (&mut test_env).used_paths.push(root_path.join("paste_here.txt"));
    let destination_path = (&test_env).used_paths.get(2).unwrap().clone();
    symlink(&target_path, &destination_path, SymlinkFlag::File)
        .expect("Couldn't create symlink.");

    read_test(&mut test_env);
    write_test(&mut test_env);
}

fn read_test(env: &mut TestEnv) {
    let symlink_path = (&env).used_paths.get(2).unwrap().clone();

    let text = fs::read_to_string(symlink_path)
        .expect("Couldn't read texts from symlink.");

    dbg!(&text);
    assert_eq!(text, "Hehe yes! This is it!");
}

fn write_test(env: &mut TestEnv) {
    let original_path = (&env).used_paths.get(1).unwrap().clone();
    let symlink_path = (&env).used_paths.get(2).unwrap().clone();

    let mut symlink_file = OpenOptions::new().read(true).append(true).open(symlink_path)
        .expect("Couldn't read texts from symlink.");

    (&mut symlink_file).write(b" Yay!")
        .expect("Couldn't write texts to symlink.");

    (&mut symlink_file).flush().expect("Couldn't flush file.");
    (&mut symlink_file).rewind().expect("Couldn't rewind file.");

    let mut symlink_text = String::new();
    (&symlink_file).read_to_string(&mut symlink_text)
        .expect("Couldn't read texts from symlink.");

    dbg!(&symlink_text);
    assert_eq!(&symlink_text, "Hehe yes! This is it! Yay!");

    let original_text = fs::read_to_string(original_path)
        .expect("Couldn't read texts from original file.");

    dbg!(&original_text);
    assert_eq!(&original_text, "Hehe yes! This is it! Yay!");
}
