Run the examples using `cargo run --example <example>`

Run the tests using `cargo test -- --test-threads=1`

Gmsh is a shared resource, and Rust tests run in parallel by default, so `cargo test` 
alone will crash.  

## Linking to the Gmsh library 
Gmsh ships prebuilt binary SDK libraries for Linux, MacOS, and Windows.

The download server is: [http://gmsh.info/bin/](http://gmsh.info/bin/)

Download a recent version and make sure the filename ends with `sdk`.

### Linux 
Place all dynamic library `libgmsh.so*` files where the linker can find them. 

### Windows 
Gmsh is built by MinGW on Windows, not the Visual Studio toolchain.

Because of ABI differences, the easiest way to link your Rust programs to Gmsh is to use the `x86_64-pc-windows-gnu` target and `stable-gnu` toolchain. 
```shell
rustup default stable-gnu
```

Then, download the Windows SDK zip file and ensure the linker can find the library. 

If it can't, you'll get large scary errors when you run `cargo test -- --test-threads=1`

One way to make sure the linker can find Gmsh is copying the `.lib` and `.dll` files to your `rustup` directory, e.g.: 

`.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\lib`
