use cc;
use glob::glob; // TODO(mr): Don't need glob if we just need core?

// TODO(mr): Could also provide option to link to system version but I don't think that's
// particularly useful in Rust.
// TODO(mr): Consider separating compilation of the library from the C API
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

    // TODO(mr): Alternatively, fix/disable these warnings in the source, or disable all warnings
    // here--this currently may be ignored by some compilers. Same deal when compiling the C API.
    build.flag_if_supported("-Wno-unused-parameter");
    build.flag_if_supported("-Wno-delete-non-virtual-dtor");
    build.flag_if_supported("-Wno-missing-field-initializers");
    build.flag_if_supported("-Wno-unused-function");
    build.flag_if_supported("-Wno-missing-braces");
    build.flag_if_supported("-Wno-unused-value");
    build.flag_if_supported("-Wno-char-subscripts");

    build.compile("soloud");
}

// TODO(mr): Add readme explaining how to choose a backend. Also provide a default impl of compile
// backend that explains that you need to set one if you don't.
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

// TODO(mr): Init fails
#[cfg(feature = "sdl1_dynamic")]
fn compile_backend(build: &mut cc::Build) {
    build.define("WITH_SDL1", Some(""));

    build.file("../../src/backend/sdl/soloud_sdl1.cpp");

    cc::Build::new()
        .cpp(false)
        .define("WITH_SDL", "") // TODO(mr): vs sdl1..?
        .file("../../src/backend/sdl/soloud_sdl1_dll.c")
        .compile("soloud_sdl1_dll");


    println!("cargo:rustc-link-lib=dylib=sdl");
}

// TODO(mr): Init fails
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
