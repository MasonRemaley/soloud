use libc::{c_int, c_uint, c_char, c_float};
use std::ffi::CString;
// #include <stdlib.h>
// #include <stdio.h>
// #include <math.h>
// #include "soloud_c.h"

// TODO(mr): Move this out into demo.rs
pub fn run_demo() {
    unsafe {
        let soloud = Soloud_create();
        assert!(!soloud.is_null()); // TODO(mr): Return option instead? same for others that return pointers

        assert_eq!(Soloud_initEx(soloud, SOLOUD_CLIP_ROUNDOFF | SOLOUD_ENABLE_VISUALIZATION, SOLOUD_AUTO, SOLOUD_AUTO, SOLOUD_AUTO, SOLOUD_AUTO), 0);

        speech_test(soloud);
        // queue_test(soloud); // TODO(mr): implement...

        Soloud_deinit(soloud);
            
        Soloud_destroy(soloud);

        println!("Cleanup done.");
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
        for i in 0..p {
            print!("=");
        }
        for i in p..60 {
            print!(" ");
        }
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

// void generate_sample(float *buf, int *count)
// {
//     int i;
//     int base = *count;
//     for (i = 0; i < 2048; i++, base++)
//     {
//         buf[i] = (float)sin(220 * 3.14 * 2 * base * (1 / 44100.0)) -
//                  (float)sin(230 * 3.14 * 2 * base * (1 / 44100.0));
//         buf[i] += (((rand() % 1024) - 512) / 512.0f) *
//                   (float)sin(60 * 3.14 * 2 * base * (1 / 44100.0)) *
//                   (float)sin(1 * 3.14 * 2 * base * (1 / 44100.0));
//         float fade = (44100 * 10 - base) / (44100 * 10.0f);
//         buf[i] *= fade * fade;
//     }
//     *count = base;
// }

// void queue_test(Soloud *soloud)
// {
//     int i;
//     int count = 0;
//     int cycle = 0;
//     Queue *queue = Queue_create();  
//     Wav *wav[4];
//     float buf[2048];
//     for (i = 0; i < 4; i++)
//         wav[i] = Wav_create();
//     for (i = 0; i < 2048; i++)
//         buf[i] = 0;

//     Soloud_play(soloud, queue);

//     for (i = 0; i < 4; i++)
//     {
//         generate_sample(buf, &count);
//         Wav_loadRawWaveEx(wav[i], buf, 2048, 44100, 1, 1, 0);
//         Queue_play(queue, wav[i]);
//     }

//     printf("Playing queue / wav generation test..\n");

//     while (count < 44100 * 10 && Soloud_getVoiceCount(soloud) > 0)
//     {
//         if (Queue_getQueueCount(queue) < 3)
//         {
//             generate_sample(buf, &count);
//             Wav_loadRawWaveEx(wav[cycle], buf, 2048, 44100, 1, 1, 0);
//             Queue_play(queue, wav[cycle]);
//             cycle = (cycle + 1) % 4;
//         }
//         visualize_volume(soloud);
//     }

//     while (Soloud_getVoiceCount(soloud) > 0)
//     {
//         visualize_volume(soloud);
//     }

//     printf("\nFinished.\n");

//     for (i = 0; i < 4; i++)
//         Wav_destroy(wav[i]);
//     Queue_destroy(queue);
// }


// TODO(mr): Or can we get these as statics?
pub const SOLOUD_AUTO: c_uint = 0;
pub const SOLOUD_SDL1: c_uint = 1;
pub const SOLOUD_SDL2: c_uint = 2;
pub const SOLOUD_PORTAUDIO: c_uint = 3;
pub const SOLOUD_WINMM: c_uint = 4;
pub const SOLOUD_XAUDIO2: c_uint = 5;
pub const SOLOUD_WASAPI: c_uint = 6;
pub const SOLOUD_ALSA: c_uint = 7;
pub const SOLOUD_OSS: c_uint = 8;
pub const SOLOUD_OPENAL: c_uint = 9;
pub const SOLOUD_COREAUDIO: c_uint = 10;
pub const SOLOUD_OPENSLES: c_uint = 11;
pub const SOLOUD_VITA_HOMEBREW: c_uint = 12;
pub const SOLOUD_NULLDRIVER: c_uint = 13;
pub const SOLOUD_BACKEND_MAX: c_uint = 14;
pub const SOLOUD_CLIP_ROUNDOFF: c_uint = 1;
pub const SOLOUD_ENABLE_VISUALIZATION: c_uint = 2;
pub const SOLOUD_LEFT_HANDED_3D: c_uint = 4;
pub const BASSBOOSTFILTER_WET: c_uint = 0;
pub const BASSBOOSTFILTER_BOOST: c_uint = 1;
pub const BIQUADRESONANTFILTER_NONE: c_uint = 0;
pub const BIQUADRESONANTFILTER_LOWPASS: c_uint = 1;
pub const BIQUADRESONANTFILTER_HIGHPASS: c_uint = 2;
pub const BIQUADRESONANTFILTER_BANDPASS: c_uint = 3;
pub const BIQUADRESONANTFILTER_WET: c_uint = 0;
pub const BIQUADRESONANTFILTER_SAMPLERATE: c_uint = 1;
pub const BIQUADRESONANTFILTER_FREQUENCY: c_uint = 2;
pub const BIQUADRESONANTFILTER_RESONANCE: c_uint = 3;
pub const FLANGERFILTER_WET: c_uint = 0;
pub const FLANGERFILTER_DELAY: c_uint = 1;
pub const FLANGERFILTER_FREQ: c_uint = 2;
pub const LOFIFILTER_WET: c_uint = 0;
pub const LOFIFILTER_SAMPLERATE: c_uint = 1;
pub const LOFIFILTER_BITDEPTH: c_uint = 2;
pub const MONOTONE_SQUARE: c_uint = 0;
pub const MONOTONE_SAW: c_uint = 1;
pub const MONOTONE_SIN: c_uint = 2;
pub const MONOTONE_SAWSIN: c_uint = 3;
pub const ROBOTIZEFILTER_WET: c_uint = 0;
pub const SFXR_COIN: c_uint = 0;
pub const SFXR_LASER: c_uint = 1;
pub const SFXR_EXPLOSION: c_uint = 2;
pub const SFXR_POWERUP: c_uint = 3;
pub const SFXR_HURT: c_uint = 4;
pub const SFXR_JUMP: c_uint = 5;
pub const SFXR_BLIP: c_uint = 6;
pub const SPEECH_KW_SAW: c_uint = 0;
pub const SPEECH_KW_TRIANGLE: c_uint = 1;
pub const SPEECH_KW_SIN: c_uint = 2;
pub const SPEECH_KW_SQUARE: c_uint = 3;
pub const SPEECH_KW_PULSE: c_uint = 4;
pub const SPEECH_KW_NOISE: c_uint = 5;
pub const SPEECH_KW_WARBLE: c_uint = 6;
pub const VIC_PAL: c_uint = 0;
pub const VIC_NTSC: c_uint = 1;
pub const VIC_BASS: c_uint = 0;
pub const VIC_ALTO: c_uint = 1;
pub const VIC_SOPRANO: c_uint = 2;
pub const VIC_NOISE: c_uint = 3;
pub const VIC_MAX_REGS: c_uint = 4;

// TODO(mr): Note to self to file an issue on my own repos to stop using empty enums for this since
// it's no longer recommended
macro_rules! opaque_struct {
    ($name:ident) => {
        #[repr(C)]
        struct $name {
            do_not_instantiate: [u8; 0],
        }
    }
}

opaque_struct!(Soloud);
opaque_struct!(Speech);
opaque_struct!(AudioSource);

// TODO(mr): Specify which library we're linking to here
// TODO(mr): I normally do this stuff manually but it might be worth mentioning Bindgen to Jari.
extern "C" {
    #[must_use]
    fn Soloud_create() -> *mut Soloud;
    // TODO(mr): must use because I assume the result is an error code
    #[must_use]
    fn Soloud_initEx(
        soloud: *mut Soloud,
        aFlags: c_uint /* = Soloud::CLIP_ROUNDOFF */,
        aBackend: c_uint /* = Soloud::AUTO */,
        aSamplerate: c_uint /* = Soloud::AUTO */,
        aBufferSize: c_uint /* = Soloud::AUTO */,
        aChannels: c_uint /* = 2 */
    ) -> c_int;
    #[must_use]
    fn Speech_create() -> *mut Speech;
    #[must_use]
    fn Speech_setText(speech: *mut Speech, text: *const c_char) -> c_int;
    fn Soloud_setGlobalVolume(soloud: *mut Soloud, volume: c_float);
    #[must_use]
    fn Soloud_play(soloud: *mut Soloud, sound: *mut AudioSource) -> c_uint; // XXX: Maybe make a typedef or such to make clear it's a handle
    #[must_use]
    fn Soloud_getVoiceCount(soloud: *mut Soloud) -> c_uint;
    fn Speech_destroy(speech: *mut Speech);
    fn Soloud_deinit(soloud: *mut Soloud);
    fn Soloud_destroy(soloud: *mut Soloud);
    #[must_use]
    fn Soloud_getApproximateVolume(soloud: *mut Soloud, channel: c_uint) -> c_float;
}

// TODO(mr): Add some sort of basic tests or just rely on whatever has already been set up?
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
