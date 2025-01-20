### Rust App with Yew and WASM

```shell

trunk build --release
trunk serve


--
trunk config show
trunk clean

Images and other resource types can be copied into the dist dir by adding 
a link like this to your source HTML: <link data-trunk rel="copy-file" href="path/to/image"/>

To copy an entire directory of assets/images, you can use 
the following HTML: <link data-trunk rel="copy-dir" href="path/to/images-dir"/>.

```rust

//     <link data-trunk rel="css" href="main.css" /> 

let url = "https://github.com/yewstack/yew/blob/master/examples/todomvc/src/main.rs";

```
