IP Tool Core
===============
Rust based core of the IP Tool.

Include this lib crate  as a dependency for any program to make use of it.<br />
Simply set your top level crate as a workspace and add iptool-core as a dependency.<br />
Note that you can remove this README and LICENSE when using this lib if you so desire.
You only need the "src" and the "Cargo.toml".

You can use this repo as a submodule in any git project as well.

Supported Functions
------------
The tool supports the following IP functions:

  - Validates an IPv4 address.
  - Converts an IPv4 address to IPv6.
  - Converts an IPv4 address to it's binary representation.
  - Validates an IPv6 address.
  - Converts an IPv6 address to IPv4;
  - Converts an IPv6 address to it's binary representation.

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
