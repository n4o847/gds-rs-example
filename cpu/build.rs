use cuda_builder::CudaBuilder;
use std::process::Command;

fn main() {
    CudaBuilder::new("../gpu")
        .copy_to("../resources/gpu.ptx")
        .build()
        .unwrap();

    Command::new("sed")
        .arg("-i")
        .args(&["-e", r#"s/^\.version .*/.version 7.6/"#])
        .args(&["-e", r#"s/^\.target .*/.target sm_86/"#])
        .arg("../resources/gpu.ptx")
        .output()
        .unwrap();
}
