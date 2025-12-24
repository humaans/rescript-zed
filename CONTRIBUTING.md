# Contributing to rescript-zed

## Developing

This guide covers local development for the rescript-zed extension. For more detailed information about Zed extension development, see the [official Zed documentation](https://zed.dev/docs/extensions/developing-extensions).

### Prerequisites

Before starting to develop this extension, make sure you have:

- [Rust installed via rustup](https://rustup.rs/) (required for Zed extensions)
- Zed editor installed

**Note:** Rust must be installed via rustup. If you have Rust installed via homebrew or otherwise, installing dev extensions will not work.

### Development Workflow

When developing the extension, you can use it in Zed without needing to publish it by installing it as a _dev extension_.

1. **Clone this repository**

2. **Install the dev extension in Zed:**
   - Open Zed
   - Open the Extensions page (Cmd + Shift + P, then type "zed: extensions")
   - Click the `Install Dev Extension` button (or use the `zed: install dev extension` action)
   - Select the directory containing this extension

3. **Making changes:**
   - Edit the extension code as needed
   - You do **not** need to build anything manually before installing/reinstalling
   - After making changes, use the "Reinstall" button from the Extensions menu to reload your changes

4. **Sanity check (optional):**
   - You can run `cargo build` in the extension directory to verify the Rust code compiles
   - This is not required for (re)installing the extension, but can help catch compilation errors early

### Debugging

If you need to troubleshoot, here are some useful debugging tools:

**View Zed logs:**

```sh
tail -f ~/Library/Logs/Zed/Zed.log
```

Or use the `zed: open log` command from Zed.

**For more verbose debug output (recommended):**

Close Zed completely and relaunch it from the command line with:

```sh
zed --foreground
```

This will show more verbose INFO level logging in your terminal, including `println!` and `dbg!` output from your extension code. This is the preferred way to debug extensions during development.

**View language server logs:**

Open Cmd + Shift + P and find:

```
dev: open language server logs
```

### Using a Local Language Server Build

If you're also developing the ReScript language server locally, you can configure Zed to use your local build instead of the published version.

Add the following to your Zed settings (`zed: open settings file`):

```json
{
  "lsp": {
    "rescript-language-server": {
      "binary": {
        "path": "/absolute-path/to/your/node-or-bun",
        "arguments": [
          "/path/to/your/rescript-vscode/server/out/cli.js",
          "--stdio"
        ]
      }
    }
  }
}
```

Replace the paths with your actual local paths:
- `path`: Path to your Node.js/Bun runtime (e.g., `/Users/username/.bun/bin/bun` or `/usr/local/bin/node`)
- First argument: Path to your local language server CLI (e.g., `/Users/username/Projects/rescript-vscode/server/out/cli.js`)

**Note:** Make sure your local language server is built before using it. For rescript-vscode, this typically means running the build command in that repository first.

### Publishing Changes

If you already have the published version of the extension installed, it will be uninstalled automatically when you install the dev extension. The Extensions page will indicate that the upstream extension is "Overridden by dev extension".

To publish updates to the extension, follow the [Zed extension publishing guidelines](https://zed.dev/docs/extensions/developing-extensions#publishing-your-extension).
