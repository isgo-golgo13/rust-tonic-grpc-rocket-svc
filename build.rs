//extern crate dotenv;

//use dotenv;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //dotenv::dotenv().expect("Failed to read .env file");

    //let out_pb_dir = PathBuf::from(env::var("PROTOGEN").unwrap());
    let out_pb_dir = PathBuf::from("./protogen");
    tonic_build::configure()
        .out_dir(out_pb_dir)
        .compile(&["proto/rocket-launch.proto"], &["proto"])
        .unwrap();
    Ok(())
}
