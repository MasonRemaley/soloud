use libc::rand;
use soloud::*;
use std::f32::consts::PI;
use std::ffi::{CStr, CString};
use libc::c_int;

fn unwrap(soloud: *mut Soloud, result: c_int) {
    unsafe {
        if result != 0 {
            let message = Soloud_getErrorString(soloud, result);
            panic!("Error {}: {}", result, CStr::from_ptr(message).to_str().unwrap());
            // TODO(mr): Free the message?
        }
    }
}

pub fn main() {
    unsafe {
        // Init the backend, if necessary
        #[cfg(any(feature = "sdl1_dynamic", feature = "sdl2_dynamic"))]
        unsafe {
            use libc::{c_int, uint32_t};
            extern "C" {
                fn SDL_Init(flags: uint32_t) -> c_int;
            }
            assert_eq!(SDL_Init(0x00000010), 0); // TODO(mr): Correct flags?
        }

        let soloud = Soloud_create();
        // TODO(mr): Return option instead? same for others that return pointers
        assert!(!soloud.is_null());

        unwrap(soloud, Soloud_initEx(
            soloud,
            SOLOUD_CLIP_ROUNDOFF | SOLOUD_ENABLE_VISUALIZATION,
            SOLOUD_AUTO,
            SOLOUD_AUTO,
            SOLOUD_AUTO,
            SOLOUD_AUTO
        ));

        speech_test(soloud);
        queue_test(soloud);

        Soloud_deinit(soloud);

        Soloud_destroy(soloud);

        println!("Cleanup done.");
    }
}

fn speech_test(soloud: *mut Soloud) {
    unsafe {
        let speech = Speech_create();
        assert!(!speech.is_null());

        let string = CString::new("1 2 3       A B C        Doooooo    Reeeeee    Miiiiii    Faaaaaa    Soooooo    Laaaaaa    Tiiiiii    Doooooo!").unwrap();
        assert_eq!(Speech_setText(speech, string.as_ptr()), 0);

        Soloud_setGlobalVolume(soloud, 4.0);
        Soloud_play(soloud, speech as *mut AudioSource);

        println!("Playing speech test..");

        let mut spin = 0;
        while Soloud_getVoiceCount(soloud) > 0 {
            visualize_volume(soloud, &mut spin);
        }
        println!();
        println!("Finished.");
        Speech_destroy(speech);
    }
}

fn queue_test(soloud: *mut Soloud) {
    unsafe {
        let queue = Queue_create();
        let mut wav = Vec::new();
        for _ in 0..4 {
            let temp = Wav_create();
            assert!(!temp.is_null());
            wav.push(temp);
        }
        let mut buf = vec![0.0; 2048];

        Soloud_play(soloud, queue as *mut AudioSource);

        let mut count = 0;
        for i in 0..4 {
            generate_sample(&mut buf, &mut count);
            assert_eq!(
                Wav_loadRawWaveEx(wav[i], buf.as_mut_ptr(), 2048, 44100.0, 1, 1, 0),
                0
            );
            assert_eq!(Queue_play(queue, wav[i] as *mut AudioSource), 0);
        }

        println!("Playing queue / wav generation test..");

        let mut spin = 0;
        let mut cycle = 0;
        while count < 44100 * 10 && Soloud_getVoiceCount(soloud) > 0 {
            if Queue_getQueueCount(queue) < 3 {
                generate_sample(&mut buf, &mut count);
                assert_eq!(
                    Wav_loadRawWaveEx(wav[cycle], buf.as_mut_ptr(), 2048, 44100.0, 1, 1, 0),
                    0
                );
                assert_eq!(Queue_play(queue, wav[cycle] as *mut AudioSource), 0);
                cycle = (cycle + 1) % 4;
            }
            visualize_volume(soloud, &mut spin);
        }

        while Soloud_getVoiceCount(soloud) > 0 {
            visualize_volume(soloud, &mut spin);
        }

        println!("\nFinished.");

        for w in wav {
            Wav_destroy(w);
        }
        Queue_destroy(queue);
    }
}

// TODO(mr): Clean up the spin thing
fn visualize_volume(soloud: *mut Soloud, spin: &mut i32) {
    unsafe {
        let v = Soloud_getApproximateVolume(soloud, 0);
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
