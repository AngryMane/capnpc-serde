This example app converts a capn proto schema file to mermaid class diagram.

# How to use

```
$ sudo npm install -g puppeteer
$ npm install -g @mermaid-js/mermaid-cli # DO NOT INSTALL GLOBALLY
$ capnp-mmd ${TARGET_SCHEMA_ABSLUTE_PATH} -i${IMPORT_PATH_A} -i${IMPORT_PATH_B}
```