fn has_protoc() -> bool {
    std::process::Command::new("protoc").arg("--version").output().is_ok()
}

fn main() {
    if has_protoc() {
        prost_build::compile_protos(&["src/bin/prost/person.proto"], &["src/"]).unwrap();

        // Files with gRPC services should not be compiled with `prost` but with `tonic`,
        // as `prost` will not generate code for RPC services, only for proto messages.
        // prost_build::compile_protos(&["src/bin/tonic/helloworld.proto"], &["src/"]).unwrap();

        tonic_build::compile_protos("src/bin/tonic/helloworld.proto").unwrap();

        // NOTE: Above usage of `tonic_build::compile_protos` will emit:
        // cargo:rerun-if-changed=src/bin/tonic/helloworld.proto
        // cargo:rerun-if-changed=src/bin/tonic
        // That will discard default change detection of re-running the build script
        // if any file within the package is changed.
        // And `prost_build::compile_protos` will not emit any `rerun-if-changed`.
        // So, change detection will not track `person.proto` file.
        // To fix this it's possible to add this directive manually or use `tonic_build::compile_protos`.
        println!("cargo::rerun-if-changed=src/bin/prost/person.proto");

        // Also just in case be aware of footgun <https://github.com/tokio-rs/prost/issues/139>
        // in combination with <https://doc.rust-lang.org/cargo/reference/build-scripts.html#change-detection>.
    } else {
        // Optional: Emit a warning to the console during build
        println!("cargo::warning=protoc not found. Skipping proto generation.");
    }
}
