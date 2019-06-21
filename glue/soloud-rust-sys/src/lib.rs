use libc::{c_int, c_uint, c_char, c_float, rand};
use std::ffi::CString;
use std::f32::consts::PI;

// TODO(mr): Move this out into demo.rs
pub fn run_demo() {
    unsafe {
        let soloud = Soloud_create();
        assert!(!soloud.is_null()); // TODO(mr): Return option instead? same for others that return pointers

        assert_eq!(Soloud_initEx(soloud, SOLOUD_CLIP_ROUNDOFF | SOLOUD_ENABLE_VISUALIZATION, SOLOUD_AUTO, SOLOUD_AUTO, SOLOUD_AUTO, SOLOUD_AUTO), 0);

        speech_test(soloud);
        queue_test(soloud);

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
        for _ in 0..p {
            print!("=");
        }
        for _ in p..60 {
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
            assert_eq!(Wav_loadRawWaveEx(wav[i], buf.as_mut_ptr(), 2048, 44100.0, 1, 1, 0), 0);
            assert_eq!(Queue_play(queue, wav[i] as *mut AudioSource), 0);
        }

        println!("Playing queue / wav generation test..");

        let mut spin = 0;
        let mut cycle = 0;
        while count < 44100 * 10 && Soloud_getVoiceCount(soloud) > 0 {
            if Queue_getQueueCount(queue) < 3 {
                generate_sample(&mut buf, &mut count);
                assert_eq!(Wav_loadRawWaveEx(wav[cycle], buf.as_mut_ptr(), 2048, 44100.0, 1, 1, 0), 0);
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

// TODO(mr): Build the c example, make sure it sounds the same
// TODO(mr): Clean up loop
// TODO(mr): Don't use libc for rand
fn generate_sample(buf: &mut [f32], count: &mut i32) {
    let mut i = 0;
    let mut base = *count;
    while i < 2048 {
        buf[i] = (220.0 * PI * 2.0 * base as f32 * (1.0 / 44100.0)).sin() -
                 (230.0 * PI * 2.0 * (base as f32) * (1.0 / 44100.0)).sin();
        buf[i] += (((unsafe { rand() } % 1024) - 512) as f32 / 512.0) *
                  (60.0 * PI * 2.0 * (base as f32) * (1.0 / 44100.0)).sin() *
                  (1.0 * PI * 2.0 * (base as f32) * (1.0 / 44100.0)).sin();
        let fade = (44100.0 * 10.0 - (base as f32)) / (44100.0 * 10.0);
        buf[i] *= fade * fade;

        i += 1;
        base += 1;
    }
    *count = base;
}


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
opaque_struct!(Queue);
opaque_struct!(Wav);

// TODO(mr): Specify which library we're linking to here
// TODO(mr): I normally do this stuff manually but it might be worth mentioning Bindgen to Jari.
// TODO(mr): Do we care if the pointers are mut or not?
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
    fn Soloud_play(soloud: *mut Soloud, sound: *mut AudioSource) -> c_uint; // XXX: Maybe make a typedef or such to make clear it's a handle
    #[must_use]
    fn Soloud_getVoiceCount(soloud: *mut Soloud) -> c_uint;
    fn Speech_destroy(speech: *mut Speech);
    fn Soloud_deinit(soloud: *mut Soloud);
    fn Soloud_destroy(soloud: *mut Soloud);
    #[must_use]
    fn Soloud_getApproximateVolume(soloud: *mut Soloud, channel: c_uint) -> c_float;
    #[must_use]
    fn Queue_create() -> *mut Queue;
    fn Queue_destroy(queue: *mut Queue);
    #[must_use]
    fn Wav_create() -> *mut Wav;
    #[must_use]
    fn Wav_loadRawWaveEx(
        wav: *mut Wav,
        mem: *mut c_float,
        length: c_uint,
        sample_rate: c_float /* = 44100.0f */,
        channels: c_uint /* = 1 */,
        copy: c_int /* = false */,
        aTakeOwnership: c_int /* = true */,
    ) -> c_int;
    #[must_use]
    fn Queue_play(queue: *mut Queue, sound: *mut AudioSource) -> c_int;
    fn Queue_getQueueCount(queue: *mut Queue) -> c_uint;
    fn Wav_destroy(wav: *mut Wav);
}

// TODO(mr): Add some sort of basic tests or just rely on whatever has already been set up?
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
