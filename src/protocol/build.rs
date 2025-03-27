fn main() {
  tonic_build::configure()
    .build_server(true)
    .out_dir("src/server")
    .compile_protos(
      &[
        "src/pb/common.proto",
        "src/pb/kv.proto",
        "src/pb/placement.proto",
        "src/pb/openraft.proto",
      ],
      &["src/pb"],
    )
    .unwrap();
}
