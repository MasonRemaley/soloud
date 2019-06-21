use cc;

fn main() {
    // TODO(mr): Gives off warning about non-virtual destructor, silence or fix this
    cc::Build::new()
        .file("../../src/c_api/soloud_c.cpp")
        .include("../../include")
        .compile("soloud");
}
