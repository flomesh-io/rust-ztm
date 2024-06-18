use cmake::Config;

fn main() {

    // run npm install in the libs/ztm
    let _ = std::process::Command::new("npm")
        .current_dir("libs/ztm/pipy")
        .arg("install")
        .output()
        .expect("failed to execute process");

    let mut config = Config::new("libs/ztm/pipy");
    // set to use clang/clang++ to compile
    config.define("CMAKE_C_COMPILER", "clang");
    config.define("CMAKE_CXX_COMPILER", "clang++");

    // compile ztm in pipy
    config.define("PIPY_SHARED", "ON");
    config.define("PIPY_GUI", "OFF");
    config.define("PIPY_CODEBASES", "ON");
    // let pwd = std::env::current_dir().unwrap().to_str().unwrap().to_string();
    // config.define(
    //     "PIPY_CUSTOM_CODEBASES",
    //     //"agent:libs/ztm/agent,hub:libs/ztm/hub,ca:libs/ztm/ca",
    //     &format!("ztm/agent:{}/libs/ztm/agent,ztm/hub:{}/libs/ztm/hub,ztm/ca:{}/libs/ztm/ca", pwd, pwd, pwd),
    // );
    config.define("PIPY_CUSTOM_CODEBASES", "ztm/agent:../agent,ztm/hub:../hub,ztm/ca:../ca");

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
    // add the path to the library to the linker search path, used in build
    println!("cargo:rustc-link-search={}/build", dst.display());

    println!("cargo:rustc-link-lib=pipy");
}
