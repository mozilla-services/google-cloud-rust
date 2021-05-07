# Simple mod.rs updater

This is a very simple, very hacky `mod.rs` updater.

After a grpcio submodule update, it's possible that the third party APIs may have changed radically. This can lead to headaches around trying to update the various `mod.rs`. This script is a very simple tool that attempts to rectify the local generated files with what the `mod.rs` knows.

This file is in python3 because it's hacky. I look forward to your PRs and rust versions.

It presumes that you're running it from within this directory, but you can pass

`MOD_UP_PATH=path/to/googleapis-raw python3 mod_updater.py`

and it will update the files in `path/to/googleapis-raw`
