fn main() {
    prost_build::compile_protos(&["src/bin/prost/person.proto"], &["src/"]).unwrap();
}
