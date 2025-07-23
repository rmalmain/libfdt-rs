# libfdt-rs

`libfdt-rs` is a library for handling FDT binaries.
It uses [libfdt](https://github.com/dgibson/dtc) under the hood.

## Zero-copy

As much as possible, the library avoids copying data.
Nodes and properties are cheap references into the FDT binary in memory.
Lifetime is property handled, avoiding some pitfalls met while manipulating FDT binaries.

## `Devicetree` compliant

This crates aims at being compliant with [the devicetree specification](https://www.devicetree.org/specifications/)
as much as possible.

This crate officially supports the [devicetree specification v0.4](https://github.com/devicetree-org/devicetree-specification/releases/tag/v0.4).

## Linux special properties

The crate handles special properties used by the Linux kernel.
It makes it easy to retrieve phandle links between subnodes, as detected by the Linux kernel.

## `no_std` compatible

The crate is fully compatible with no_std.

# Example code

Here is a short example, printing all the subnodes and the properties of the root node.

```rust
use std::fs;
use libfdt_rs::Fdt;

fn main() {
    let fdt_bin = fs::read("dtb/zuma-a0-foplp.dtb").unwrap();
    let fdt = Fdt::new(fdt_bin.into_boxed_slice()).unwrap();
    let root_node = fdt.get_node("/").unwrap();

    for subnode in root_node.subnodes_iter() {
        println!("subnode:?");
    }

    for property in root_node.properties_iter() {
        println!("subnode:?");
    }
}
```

# DTB samples

- [zuma-a0-foplp.dtb](dtb/zuma-a0-foplp.dtb): taken from [the GrapheneOS project](https://github.com/GrapheneOS/device_google_caimito-kernels_6.1/blob/ec749173e7e757fc60aeddc3fe3fd5780c622077/grapheneos/zuma-a0-foplp.dtb).
