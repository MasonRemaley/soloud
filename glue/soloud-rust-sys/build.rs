use cc;
use glob::glob;

fn main() {
    let mut build = cc::Build::new();

    build.cpp(true);
    build.include("../../include");

    build.file("../../src/c_api/soloud_c.cpp");
    for entry in glob("../../src/core/*.cpp").unwrap() {
        build.file(entry.unwrap());
    }
    for entry in glob("../../src/audiosource/**/*.cpp").unwrap() {
        build.file(entry.unwrap());
    }

    compile_backend(&mut build);

    // NOTE: I'm disabling warnings here that I get from the soloud code, but the same syntax might
    // not be used for disabling warnings on all compilers.
    build.flag_if_supported("-Wno-unused-parameter");
    build.flag_if_supported("-Wno-delete-non-virtual-dtor");
    build.flag_if_supported("-Wno-missing-field-initializers");
    build.flag_if_supported("-Wno-unused-function");
    build.flag_if_supported("-Wno-missing-braces");
    build.flag_if_supported("-Wno-unused-value");
    build.flag_if_supported("-Wno-char-subscripts");

    build.compile("soloud");
}

#[cfg(feature = "coreaudio")]
fn compile_backend(build: &mut cc::Build) {
    build.define("WITH_COREAUDIO", Some(""));

    for entry in glob("../../src/backend/coreaudio/*.cpp").unwrap() {
        build.file(entry.unwrap());
    }

    println!("cargo:rustc-link-lib=framework=AudioToolbox");
}

#[cfg(feature = "null")]
fn compile_backend(build: &mut cc::Build) {
    build.define("WITH_NULL", Some(""));

    for entry in glob("../../src/backend/null/*.cpp").unwrap() {
        build.file(entry.unwrap());
    }
}

#[cfg(feature = "sdl1_dynamic")]
fn compile_backend(build: &mut cc::Build) {
    build.define("WITH_SDL1", Some(""));

    build.file("../../src/backend/sdl/soloud_sdl1.cpp");

    cc::Build::new()
        .cpp(false)
        .define("WITH_SDL", "") // NOTE: vs sdl1..?
        .file("../../src/backend/sdl/soloud_sdl1_dll.c")
        .compile("soloud_sdl1_dll");


    println!("cargo:rustc-link-lib=dylib=sdl");
}

#[cfg(feature = "sdl2_dynamic")]
fn compile_backend(build: &mut cc::Build) {
    build.define("WITH_SDL2", Some(""));

    build.file("../../src/backend/sdl/soloud_sdl2.cpp");

    cc::Build::new()
        .cpp(false)
        .define("WITH_SDL2", "")
        .file("../../src/backend/sdl/soloud_sdl2_dll.c")
        .compile("soloud_sdl1_dll");


    println!("cargo:rustc-link-lib=dylib=sdl2");
}

#[cfg(not(any(feature = "sdl2_dynamic", feature = "sdl1_dynamic", feature = "null", feature = "coreaudio")))]
fn compile_backend(_: &mut cc::Build) {
    compile_error!("no backend set, use feature flags (e.g. `--features coreaudio`, or `features soloud-rust-sys/coreaudio` if using the wrapper crate)");
}
