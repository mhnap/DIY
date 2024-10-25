fn main() {
    prost_build::compile_protos(&["src/bin/prost/person.proto"], &["src/"]).unwrap();
    tonic_build::compile_protos("src/bin/tonic/helloworld.proto").unwrap();
}
