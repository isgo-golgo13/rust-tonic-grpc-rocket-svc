use std::path::PathBuf;

fn main()->Result<(),Box<dyn std::error::Error>> {
    let out_pb_dir = PathBuf::from("./protogen");
    tonic_build::configure().out_dir(out_pb_dir)
        .compile(&["proto/rocket-launch.proto"], &["proto"])
        .unwrap();
    Ok(())
 }
 