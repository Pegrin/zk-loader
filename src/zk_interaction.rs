extern crate zookeeper;

use std::borrow::BorrowMut;
use std::fs::File;
use std::time::Duration;

use flate2::Compression;
use flate2::write::GzEncoder;
use tar::{Builder, Header};

use self::zookeeper::ZooKeeper;

pub fn dump(servers: &str, znode_paths: Vec<&str>, dump_file: &str, excluded_znodes: Vec<&str>) {
    let zk_client = ZooKeeper::connect(servers, Duration::from_secs(15), |_| {}).unwrap();
    zk_client.exists("/", false).expect("Connection failed");
    for znode_path in &znode_paths {
        zk_client.exists(*znode_path, false).expect(format!("Expected znode is absent: {}", *znode_path).as_str());
    }
    for tree_root_znode_path in znode_paths {
        dump_znode_tree(&zk_client, tree_root_znode_path, dump_file, &excluded_znodes);
    }
}

pub fn restore(servers: &str, dump_file: &str, znode_paths: Vec<&str>, excluded_znodes: Vec<&str>) {
    let zk_client = ZooKeeper::connect(servers, Duration::from_secs(15), |_| {}).unwrap();
    zk_client.exists("/", false).expect("Connection failed");
    //TODO Add restoring
}

fn dump_znode_tree(zk_client: &ZooKeeper, tree_root_znode_path: &str, dump_file: &str, excluded_znodes: &Vec<&str>) {
    let dump_file = File::create(dump_file).expect(format!("Can't create file '{}'", dump_file).as_str());
    let enc = GzEncoder::new(dump_file, Compression::fast());
    let mut tar_archive = tar::Builder::new(enc);
    dump_znodes_recursively(zk_client, tree_root_znode_path, excluded_znodes, tar_archive.borrow_mut());
    tar_archive.finish().unwrap();
}

fn dump_znodes_recursively(zk_client: &ZooKeeper, znode_path: &str, excluded_znodes: &Vec<&str>, tar_archive: &mut Builder<GzEncoder<File>>) {
    if excluded_znodes.contains(&znode_path) {
        return;
    }
    let (data, _) = zk_client.get_data(znode_path, false).unwrap();
    write_znode_data_to_tar(znode_path, data, tar_archive);
    let children = zk_client.get_children(znode_path, false).unwrap();
    let current_path = ensure_ends_with_slash(znode_path);
    children.iter()
        .map(|child| current_path.clone() + child)
        .for_each(|child| dump_znodes_recursively(zk_client, child.as_str(), excluded_znodes, tar_archive));
}

fn ensure_ends_with_slash(znode_path: &str) -> String {
    let mut current_path = String::from(znode_path);
    if !current_path.ends_with("/") {
        current_path.push('/');
    }
    current_path
}

fn write_znode_data_to_tar(znode_path: &str, data: Vec<u8>, tar_archive: &mut Builder<GzEncoder<File>>) {
    let mut header = Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_cksum();
    let tar_path = znode_path_to_tar_path(znode_path);
    println!("Writing: {}", tar_path);
    tar_archive.append_data(&mut header, tar_path, data.as_slice()).unwrap();
}

fn znode_path_to_tar_path(znode_path: &str) -> String {
    Option::from(String::from(znode_path))
        .map(|path| path + "/____data")
        .map(|path| path.chars()
            .skip_while(|char| char == &'/')
            .collect::<String>())
        .unwrap()
}

