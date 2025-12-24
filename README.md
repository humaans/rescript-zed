# rescript-zed

ReScript support for [Zed](https://zed.dev) editor.

This extension plugs in the following projects:

- [tree-sitter-rescript](https://github.com/rescript-lang/tree-sitter-rescript) parser
- [@rescript/language-server](https://github.com/rescript-lang/rescript-vscode) LSP

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

## Developing

See [CONTRIBUTING.md](CONTRIBUTING.md) for instructions on how to develop this extension locally.
