use libc::rand;
use soloud_rust_sys::*; // TODO(mr): reexport any needed constants
use std::f32::consts::PI;
use std::ffi::{CStr, CString};
use libc::c_int;
use soloud_rust::{SoLoudBuilder, SoLoud, Speech, Queue, Wav};

// TODO: capitalization/spacing on soloud? in crate/api?
pub fn main() {
    // Init the backend, if necessary
    #[cfg(any(feature = "sdl1_dynamic", feature = "sdl2_dynamic"))]
    unsafe {
        use libc::{c_int, uint32_t};
        extern "C" {
            fn SDL_Init(flags: uint32_t) -> c_int;
        }
        assert_eq!(SDL_Init(0x00000010), 0); // TODO(mr): Correct flags?
    }

    let mut soloud = SoLoudBuilder::new()
        .flags(SOLOUD_CLIP_ROUNDOFF | SOLOUD_ENABLE_VISUALIZATION,)
        .init();

    speech_test(&mut soloud);
    // queue_test(soloud);
}

fn speech_test(soloud: &mut SoLoud) {
    let mut speech = Speech::new();
    speech.set_text(CStr::from_bytes_with_nul(b"1 2 3       A B C        Doooooo    Reeeeee    Miiiiii    Faaaaaa    Soooooo    Laaaaaa    Tiiiiii    Doooooo!\0").unwrap());
    soloud.play(&speech);

    println!("Playing speech test..");
    let mut spin = 0;
    while soloud.get_voice_count() > 0 {
        visualize_volume(soloud, &mut spin);
    }
    println!("\nFinished.");
}

fn queue_test(soloud: &mut SoLoud) {
    unsafe {
        let mut queue = Queue::new();
        let mut wav = Vec::with_capacity(4);
        for _ in 0..wav.capacity() {
            wav.push(Wav::new());
        }
        let mut buf = vec![0.0; 2048];

        soloud.play(&queue);

        // TODO(mr): Okay creating and initializing together? Should we use lifetimes to prevent
        // dropping while playing somehow/can we? Or is that safe?
        let mut count = 0;
        for i in 0..4 {
            generate_sample(&mut buf, &mut count);
            wav[i].load_raw_wave_ex(&buf, 44100.0, 1);
            assert_eq!(queue.play(&wav[i]), 0);
        }

        println!("Playing queue / wav generation test..");

        let mut spin = 0;
        let mut cycle = 0;
        while count < 44100 * 10 && soloud.get_voice_count() > 0 {
            if queue.get_queue_count() < 3 {
                generate_sample(&mut buf, &mut count);
                wav[cycle].load_raw_wave_ex(&buf, 44100.0, 1);
                assert_eq!(queue.play(&wav[cycle]), 0);
                cycle = (cycle + 1) % 4;
            }
            visualize_volume(soloud, &mut spin);
        }

        while soloud.get_voice_count() > 0 {
            visualize_volume(soloud, &mut spin);
        }

        println!("\nFinished.");
    }
}

// TODO(mr): Clean up the spin thing
fn visualize_volume(soloud: &mut SoLoud, spin: &mut i32) {
    unsafe {
        let v = soloud.get_approximate_volume(0);
        print!("\r{} ", ['|', '\\', '-', '/'][(*spin & 3) as usize]);
        *spin += 1;
        let mut p = (v * 60.0) as i32;
        if p > 59 {
            p = 59;
        }
        // TODO(mr): Isn't there a format specifier for this?
        for _ in 0..p {
            print!("=");
        }
        for _ in p..60 {
            print!(" ");
        }
    }
}

// TODO(mr): Build the c example, make sure it sounds the same
// TODO(mr): Clean up loop
// TODO(mr): Don't use libc for rand
fn generate_sample(buf: &mut [f32], count: &mut i32) {
    let mut i = 0;
    let mut base = *count;
    while i < 2048 {
        buf[i] = (220.0 * PI * 2.0 * base as f32 * (1.0 / 44100.0)).sin()
            - (230.0 * PI * 2.0 * (base as f32) * (1.0 / 44100.0)).sin();
        buf[i] += (((unsafe { rand() } % 1024) - 512) as f32 / 512.0)
            * (60.0 * PI * 2.0 * (base as f32) * (1.0 / 44100.0)).sin()
            * (1.0 * PI * 2.0 * (base as f32) * (1.0 / 44100.0)).sin();
        let fade = (44100.0 * 10.0 - (base as f32)) / (44100.0 * 10.0);
        buf[i] *= fade * fade;

        i += 1;
        base += 1;
    }
    *count = base;
}
