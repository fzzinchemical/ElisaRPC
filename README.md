# ElisaRPC

This Project is a fork of the original [ElisaRPC](github.com/renarin-kholin/ElisaRPC) project, which is no longer maintained.
Due to recode and some changes, the bugs and features of this branch may differ from the original one.

**If you encounter any issues, please do not report them to the original project. Instead, report them to this fork or contact me.**


# Known Issues
- Some Album covers are not fetched.
- RPC does not stop in discord even when no song is playing.

## Build Instructions

You need to have rust and cargo installed to build this project.
[playerctl](https://github.com/altdesktop/playerctl) is also required.

Clone the git repository
`git clone https://github.com/shishudesu/ElisaRPC && cd ElisaRPC`

Build using the release profile
`cargo build --release`

Run the binary
`./target/release/elisa_rpc`

## Binaries

You can also download the prebuilt binary directly from the Releases tab.

## Supported Platforms

Only Linux is currently supported.

