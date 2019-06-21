use cc;
use glob::glob; // TODO(mr): Don't need glob if we just need core?

// TODO(mr): Could also provide option to link to system version but I don't think that's
// particularly useful in Rust.
// TODO(mr): Consider separating compilation of the library from the C API
fn main() {
    let mut soloud = cc::Build::new();

    soloud.cpp(true);
    soloud.include("../../include");

    soloud.file("../../src/c_api/soloud_c.cpp");
    for entry in glob("../../src/core/*.cpp").unwrap() {
        soloud.file(entry.unwrap());
    }

    // TODO(mr): Make feature flags for which backendsto enable. We might also need to specify which
    // backend code to compile here.
    soloud.define("WITH_COREAUDIO", Some("true"));
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    for entry in glob("../../src/backend/coreaudio/*.cpp").unwrap() {
        soloud.file(entry.unwrap());
    }

    // TODO(mr): Alternatively, fix/disable these warnings in the source, or disable all warnings
    // here--this currently may be ignored by some compilers. Same deal when compiling the C API.
    soloud.flag_if_supported("-Wno-unused-parameter");
    soloud.flag_if_supported("-Wno-delete-non-virtual-dtor");
    soloud.flag_if_supported("-Wno-missing-field-initializers");
    soloud.flag_if_supported("-Wno-unused-function");

    soloud.compile("soloud");
}
