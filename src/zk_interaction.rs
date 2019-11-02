extern crate zookeeper;

use std::borrow::BorrowMut;
use std::fs::File;
use std::time::Duration;

use flate2::Compression;
use flate2::write::GzEncoder;
use tar::{Builder, Header};

use self::zookeeper::ZooKeeper;

pub fn dump(servers: &str, znode_path: &str, dump_file: &str) {
    let zk_client = ZooKeeper::connect(servers, Duration::from_secs(15), |_| {}).unwrap();
    zk_client.exists(znode_path, false).expect("Expected znode absent");
    dump_to_targz(&zk_client, znode_path, dump_file);
}

fn dump_to_targz(zk_client: &ZooKeeper, znode_path: &str, dump_file: &str) {
    let dump_file = File::create(dump_file).unwrap();
    let enc = GzEncoder::new(dump_file, Compression::fast());
    let mut tar = tar::Builder::new(enc);
    dump_znode_recursively(zk_client, znode_path, tar.borrow_mut());
}

fn dump_znode_recursively(zk_client: &ZooKeeper, znode_path: &str, tar: &mut Builder<GzEncoder<File>>) {
    println!("Writing: {}", znode_path);
    let (data, _) = zk_client.get_data(znode_path, false).unwrap();
    let mut header = Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_cksum();
    tar.append_data(&mut header, String::from("zk-data") + znode_path, data.as_slice()).unwrap();
    let children = zk_client.get_children(znode_path, false).unwrap();
    let mut current_path = String::from(znode_path);
    if !current_path.ends_with("/") {
        current_path.push('/');
    }
    children.iter()
        .map(|child| current_path.clone() + child)
        .for_each(|child| dump_znode_recursively(zk_client, child.as_str(), tar));
}

