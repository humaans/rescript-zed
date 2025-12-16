# rescript-zed

ReScript support for [Zed](zed.dev) editor.

This extension plugs in the following projects:

- [tree-sitter-rescript](https://github.com/rescript-lang/tree-sitter-rescript) parser
- [@rescript/language-server](https://github.com/rescript-lang/rescript-vscode) LSP

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

## Settings

```json
  "lsp": {
    "rescript-language-server": {
      "initialization_options": {
        "extensionConfiguration": {
          "askToStartBuild": false
        }
      },
      "settings": {
        "version": "1.71.0-next-441959d.0"
      }
    }
  },
```

`initialization_options` are passed to the language server when it is started. They can be used to configure the language server. See [extensionConfiguration](https://github.com/rescript-lang/rescript-vscode/blob/441959d1feeaaffc1a589687758b1fbe1f649e72/server/src/config.ts#L5-L29)

`settings` are specific to the Zed extension.
With `version` you can point to a specific npm version of the [@rescript/language-server](https://www.npmjs.com/package/@rescript/language-server?activeTab=versions).
