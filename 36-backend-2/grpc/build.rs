fn main() -> Result<(), Box<dyn std::error::Error>> {
    // если protoc установлен в системе, эти две строки можно убрать
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    unsafe { std::env::set_var("PROTOC", protoc) };

    tonic_prost_build::compile_protos("proto/tasks.proto")?;
    Ok(())
}
