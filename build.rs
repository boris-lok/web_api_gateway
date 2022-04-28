fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("gen")
        .compile(&["proto/customer.proto"], &["proto"])
}
