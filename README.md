IP Tool Core
===============
Rust based core of the IP Tool.

Include this lib crate  as a dependency for any program to make use of it.<br />
Simply set your top level crate as a workspace and add iptool-core as a dependency.<br />
Note that you can remove this README and LICENSE when using this lib if you so desire.
You only need the "src" and the "Cargo.toml".

You can use this repo as a submodule in any git project as well.

Example
--------
Add these to the Cargo.toml of the top level crate for the project using iptool-core:

```
[dependencies]
iptool-core = { path = "iptool-core" }
```

and

```
[workspace]
members = ["iptool-core"]
```

Now a `cargo build --release` or `cargo build` will compile the lib and the main program.<br />
See (insert link) for examples of how to wrap interfaces around this lib.
