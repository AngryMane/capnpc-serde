This example app converts a capn proto schema file to mermaid class diagram.

# How to installo
```
$ cargo build # DO NOT SKIP becuase this deb will install debug binary. 
$ cargo deb
$ cd ./target/debian
$ sudo dpkg -i capnp-mmd_0.1.0-1_amd64.deb
```

# How to use

```
# mermaid
$ capnp-mmd ${TARGET_SCHEMA_ABSLUTE_PATH} -i${IMPORT_PATH_A} -i${IMPORT_PATH_B}

# plantuml
$ capnp-mmd ${TARGET_SCHEMA_ABSLUTE_PATH} -i${IMPORT_PATH_A} -i${IMPORT_PATH_B} -p
```

# Render Mermaid 

See https://github.com/mermaid-js/mermaid-cli

# Render Plantuml 

See https://plantuml.com/en-dark/command-line
