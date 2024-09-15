This example app converts a capn proto schema file to mermaid class diagram.

# How to installo
```
$ cargo deb
$ cd ./target/debian
$ sudo dpkg -i capnp-mmd_0.1.0-1_amd64.deb
$ npm install -g @mermaid-js/mermaid-cli 
```

# How to use

```
$ npm install -g puppeteer # DO NOT INSTALL GLOBALLY
$ capnp-mmd ${TARGET_SCHEMA_ABSLUTE_PATH} -i${IMPORT_PATH_A} -i${IMPORT_PATH_B}
```