extern crate zookeeper;

use std::borrow::BorrowMut;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use tar::{Archive, Builder, Header};

use self::zookeeper::{Acl, CreateMode, ZooKeeper};

pub struct ZkLoader<'a> {
    zk_client: ZooKeeper,
    znode_paths: Vec<&'a str>,
    excluded_znodes: Vec<&'a str>,
    dump_file: &'a str,
}

impl <'a> ZkLoader<'a> {
    pub fn new(servers: &'a str, znode_paths: Vec<&'a str>, dump_file: &'a str, excluded_znodes: Vec<&'a str>) -> Result<Self, String> {
        let zk_client = ZooKeeper::connect(servers, Duration::from_secs(15), |_| {}).unwrap();
        if zk_client.exists("/", false).is_ok() {
            Ok(ZkLoader { zk_client, znode_paths, excluded_znodes, dump_file })
        } else {
            Err("Connection failed: ".to_string())
        }
    }

    pub fn all_znodes_exist(&self) -> bool{
        (&self.znode_paths)
            .into_iter()
            .all(|znode_path| self.zk_client.exists(*znode_path, false).unwrap().is_some())
    }
}

pub fn dump(servers: &str, znode_paths: Vec<&str>, dump_file: &str, excluded_znodes: Vec<&str>) {
    let loader = ZkLoader::new(servers, znode_paths, dump_file, excluded_znodes).unwrap();
    if !loader.all_znodes_exist() {
        panic!("Some of dumped znodes are absent");
    }
    for tree_root_znode_path in loader.znode_paths {
        dump_znode_tree(&loader.zk_client, tree_root_znode_path, &loader.dump_file, &loader.excluded_znodes);
    }
}

pub fn restore(servers: &str, dump_file: &str, znode_paths: Vec<&str>, excluded_znodes: Vec<&str>) {
    let zk_client = ZooKeeper::connect(servers, Duration::from_secs(15), |_| {}).unwrap();
    zk_client.exists("/", false).expect("Connection failed");
    let tar_gz = File::open(dump_file).expect("Can't read tar file");
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    let entries = archive.entries().expect("Can't unpack tar file");
    for file in entries {
        let mut file = file.unwrap();
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).unwrap();
        let path = file.path().unwrap();
        let znode_path = tar_path_to_znode_path(path.to_str().unwrap());
        let is_excluded = excluded_znodes.iter()
            .any(|excluded| znode_path.starts_with(excluded));
        let is_for_restoring = znode_paths.iter()
            .any(|for_restoring|znode_path.starts_with(for_restoring));
        if !is_excluded && is_for_restoring {
            create_znodes_for_path(&zk_client, znode_path.as_str(), data);
        }
    }
}

fn create_znodes_for_path(zk_client: &ZooKeeper, path: &str, data: Vec<u8>) {
    let split: Vec<&str> = path.split('/').collect();
    for i in 1..split.len() {
        let new_znode = path_from_n_first_znodes(&split, i);
        zk_client.create(new_znode.as_str(), vec![], Acl::open_unsafe().clone(), CreateMode::Persistent);
    }
    let new_znode = path_from_n_first_znodes(&split, split.len() - 1);
    zk_client.set_data(new_znode.as_str(), data, Option::None).unwrap();
}

fn path_from_n_first_znodes(split_path: &Vec<&str>, n: usize) -> String {
    split_path.iter()
        .skip(1)
        .take(n)
        .fold(String::new(), |acc, node| acc + "/" + node)
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
    let (data, stat) = zk_client.get_data(znode_path, false).unwrap();
    if stat.is_ephemeral() {
        return;
    }
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

fn tar_path_to_znode_path(tar_path: &str) -> String {
    Option::from(String::from(tar_path))
        .map(|path| path.replace("____data", ""))
        .map(|path| {
            if path.ends_with("/") {
                path.chars()
                    .take(path.len() - 1)
                    .collect()
            } else {
                path
            }
        })
        .map(|path| if !path.starts_with("/") { String::from("/") + path.as_str() } else { path })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use zk_interaction::{dump, restore, tar_path_to_znode_path, znode_path_to_tar_path};

    use super::zookeeper::{Acl, CreateMode, ZooKeeper};

    fn zk_client() -> ZooKeeper {
        ZooKeeper::connect("localhost:2181", Duration::from_secs(15), |_| {}).unwrap()
    }

    #[test]
    pub fn test_dump_restore() {
        let zk = zk_client();
        let dump_file = "test-dump-file.tar.gz";
        let root_znode = ("/test_ase2134234", b"123data!".to_vec());
        let excluded_znode = ("/test_ase2134234/2", b"123data!+2".to_vec());
        let child_znode = ("/test_ase2134234/1", b"123data!+1".to_vec());
        let ephemeral_znode = ("/test_ase2134234/ephemera-znode", b"data".to_vec());

        zk.create(root_znode.0, root_znode.1.clone(), Acl::open_unsafe().clone(), CreateMode::Persistent);
        zk.create(child_znode.0, child_znode.1.clone(), Acl::open_unsafe().clone(), CreateMode::Persistent);
        zk.create(excluded_znode.0, excluded_znode.1.clone(), Acl::open_unsafe().clone(), CreateMode::Persistent);
        zk.create(ephemeral_znode.0, ephemeral_znode.1.clone(), Acl::open_unsafe().clone(), CreateMode::Ephemeral);

        dump("localhost:2181", vec![root_znode.0], dump_file, vec![excluded_znode.0]);
        zk.delete(child_znode.0, None).unwrap();
        zk.delete(excluded_znode.0, None).unwrap();
        zk.delete(root_znode.0, None).unwrap();
        restore("localhost:2181", dump_file, vec![root_znode.0], vec![excluded_znode.0]);

        assert_eq!(zk.get_data(child_znode.0, false).unwrap().0, child_znode.1);
        assert_eq!(zk.get_data(root_znode.0, false).unwrap().0, root_znode.1);
        assert!(zk.exists(excluded_znode.0, false).unwrap().is_none())
    }

    #[test]
    pub fn tar_path_to_znode_path_test() {
        let zk_path = tar_path_to_znode_path("____data");
        assert_eq!(zk_path, "/");

        let zk_path = tar_path_to_znode_path("banana/____data");
        assert_eq!(zk_path, "/banana");
    }

    #[test]
    pub fn znode_path_to_tar_path_test() {
        let tar_path = znode_path_to_tar_path("/");
        assert_eq!(tar_path, "____data");

        let tar_path = znode_path_to_tar_path("/banana");
        assert_eq!(tar_path, "banana/____data");
    }
}

