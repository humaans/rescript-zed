# rescript-zed

ReScript support for [Zed](zed.dev) editor.

This extension plugs in the following projects:

- [tree-sitter-rescript](https://github.com/rescript-lang/tree-sitter-rescript) parser
- [@rescript/language-server](https://github.com/rescript-lang/rescript-vscode) LSP

## Associate res files with ReScript

Open Settings with Cmd+, and add:

```json
"file_types": {
  "ReScript": ["res", "resi"]
}
```

## Developing

Zed and it's support for extensions is being actively developed. The current workflow that can be used to build this extension locally and install it into Zed is:

Clone zed and build the `zed-extension` cli:

    git clone git@github.com:zed-industries/zed.git
    cd zed
    cargo build --release --package extension_cli
    ln -sf "$(pwd -P)/target/release/zed-extension" /usr/local/bin/zed-extension

Build and install the extension locally

    make build

Tail zed logs

    tail -f ~/Library/Logs/Zed/Zed.log

After opening a ReScript file, open Cmd + Shift + P nav and find

    language selector: toggle

And to see the language server logs, open Cmd + Shift + P nav and find

    debug: open language server logs

## Known issues

This is the a very early cut of this extension and I have noticed:

- it's very slow to bring up autocomplete
- the syntax partially loses highlighting as you're typing
- the language server crashes quite easily and doesn't restart

We'll look to work through all these issues in the future releases!
