use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        // This works around the missing 'vmeta.pb-c.h' included in vmeta_frame_proto.h
        .clang_arg("-D_VMETA_FRAME_PROTO_H_")
        // Set the possible folders where the header files should be searched in
        .clang_arg("-Iinclude/")
        .clang_arg("-Iinclude/libfutils")
        .clang_arg("-Iinclude/libh264")
        .clang_arg("-Iinclude/libh265")
        .clang_arg("-Iinclude/libmedia-buffers")
        .clang_arg("-Iinclude/libmp4")
        .clang_arg("-Iinclude/libpomp")
        .clang_arg("-Iinclude/librtp")
        .clang_arg("-Iinclude/librtsp")
        .clang_arg("-Iinclude/libsdp")
        .clang_arg("-Iinclude/libtransport-packet")
        .clang_arg("-Iinclude/libtransport-socket")
        .clang_arg("-Iinclude/libvideo-decode")
        .clang_arg("-Iinclude/libvideo-defs")
        .clang_arg("-Iinclude/libvideo-encode")
        .clang_arg("-Iinclude/libvideo-metadata")
        .clang_arg("-Iinclude/libvideo-scale")
        .clang_arg("-Iinclude/libvideo-streaming")
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
