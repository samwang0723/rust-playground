use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["src/pb/abi.proto"], &["."])?;

    Ok(())
}
