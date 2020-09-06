```
Downloads and uploads zookeeper znodes data

USAGE:
    zk-loader.exe [FLAGS] [OPTIONS]

FLAGS:
        --delete     Delete znodes recursively
    -d, --dump       Dump data from znode to file
    -h, --help       Prints help information
    -r, --restore    Restore data from file to znode
    -V, --version    Prints version information

OPTIONS:
    -e, --excluded-znodes <ZNODES>    Excluded znodes. '/zookeeper' will be excluded any way.
    -f, --file <FILE>                 Path to data dump file [default: zk-dump.tar.gz]
    -s, --servers <SERVERS>           Zookeeper hosts [default: 127.0.0.1:2181]
    -z, --znodes <ZNODES>             Znodes paths to dump, restore or delete [default: /]

```
Changes:

--- 0.1.2 ---
- Flag for deleting znoded added
- Message output muted

--- 0.1.1 ---
 - Ephemeral znodes are excluded from processing.