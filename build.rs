use cmake::Config;
fn build_agent_ui() {
    let ui = std::path::Path::new("libs/ztm/agent/gui");
    if cfg!(feature = "agent-ui") {
        if !ui.exists() {
            // run npm run build in the libs/ztm/gui
            let _ = std::process::Command::new("npm")
                .current_dir("libs/ztm/gui")
                .arg("run")
                .arg("build")
                .output()
                .expect("failed to run npm run build in ztm/gui");
        }
    } else {
        if ui.exists() {
            std::fs::remove_dir_all(ui).expect("failed to remove agent/gui");
        }
    }
}

fn main() {
    build_agent_ui();

    // run npm install in the libs/ztm
    // TODO expect didn't report error when npm install failed, try npm-rs in the future
    let _ = std::process::Command::new("npm")
        .current_dir("libs/ztm/pipy")
        .arg("install")
        .output()
        .expect("failed to run npm install in ztm/pipy");

    let mut config = Config::new("libs/ztm/pipy");
    // clean previous build content

    // set to use clang/clang++ to compile
    config.define("CMAKE_C_COMPILER", "clang");
    config.define("CMAKE_CXX_COMPILER", "clang++");

    // compile ztm in pipy
    config.define("PIPY_SHARED", "ON");
    config.define("PIPY_GUI", "OFF");
    config.define("PIPY_CODEBASES", "ON");
    config.define(
        "PIPY_CUSTOM_CODEBASES",
        "ztm/agent:../agent,ztm/hub:../hub,ztm/ca:../ca",
    );

    std::env::set_var("CMAKE_BUILD_PARALLEL_LEVEL", "4");

    // add target to build
    config.no_build_target(true);

    // build
    let dst = config.build();

    // ** `cargo:rustc-*` format is used to pass information to the cargo build system
    // parse to `rustc` to look for dynamic library, used in running
    println!(
        "cargo:rustc-link-arg=-Wl,-rpath,{}/build,-rpath,$ORIGIN",
        dst.display()
    );
    println!(
        "cargo:rustc-cdylib-link-arg=-Wl,-rpath,{}/build,-rpath,$ORIGIN",
        dst.display()
    );

    // add the path to the library to the linker search path, used in build
    println!("cargo:rustc-link-search={}/build", dst.display());

    println!("cargo:rustc-link-lib=pipy");
}
