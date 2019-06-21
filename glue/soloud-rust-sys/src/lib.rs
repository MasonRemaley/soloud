use libc::{c_char, c_float, c_int, c_uint};

// TODO(mr): Or can we get these as statics?
// Collected enumerations
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
// Object handle types
#[macro_export]
macro_rules! opaque_struct {
    ($name:ident) => {
        #[repr(C)]
        pub struct $name {
            do_not_instantiate: [u8; 0],
        }
    };
}

opaque_struct!(Soloud);
opaque_struct!(AudioSource);
opaque_struct!(Queue);
opaque_struct!(Speech);
opaque_struct!(Wav);

// Soloud
extern "C" {
    pub fn Soloud_destroy(soloud: *mut Soloud);
    #[must_use]
    pub fn Soloud_create() -> *mut Soloud;
    // TODO(mr): must use because I assume the result is an error code
    #[must_use]
    pub fn Soloud_initEx(
        soloud: *mut Soloud,
        aFlags: c_uint,      /* = Soloud::CLIP_ROUNDOFF */
        aBackend: c_uint,    /* = Soloud::AUTO */
        aSamplerate: c_uint, /* = Soloud::AUTO */
        aBufferSize: c_uint, /* = Soloud::AUTO */
        aChannels: c_uint,   /* = 2 */
    ) -> c_int;
    pub fn Soloud_deinit(soloud: *mut Soloud);
    // XXX: Maybe make a typedef or such to make clear it's a handle
    pub fn Soloud_play(soloud: *mut Soloud, sound: *mut AudioSource) -> c_uint;
    #[must_use]
    pub fn Soloud_getVoiceCount(soloud: *mut Soloud) -> c_uint;
    pub fn Soloud_setGlobalVolume(soloud: *mut Soloud, volume: c_float);
    #[must_use]
    pub fn Soloud_getApproximateVolume(soloud: *mut Soloud, channel: c_uint) -> c_float;
}

// Queue
extern "C" {
    pub fn Queue_destroy(queue: *mut Queue);
    #[must_use]
    pub fn Queue_create() -> *mut Queue;
    pub fn Queue_play(queue: *mut Queue, sound: *mut AudioSource) -> c_int;
    pub fn Queue_getQueueCount(queue: *mut Queue) -> c_uint;
}

// Speech
extern "C" {
    pub fn Speech_destroy(speech: *mut Speech);
    #[must_use]
    pub fn Speech_create() -> *mut Speech;
    #[must_use]
    pub fn Speech_setText(speech: *mut Speech, text: *const c_char) -> c_int;
}

// Wav
extern "C" {
    pub fn Wav_destroy(wav: *mut Wav);
    #[must_use]
    pub fn Wav_create() -> *mut Wav;
    #[must_use]
    pub fn Wav_loadRawWaveEx(
        wav: *mut Wav,
        mem: *mut c_float,
        length: c_uint,
        sample_rate: c_float,  /* = 44100.0f */
        channels: c_uint,      /* = 1 */
        copy: c_int,           /* = false */
        aTakeOwnership: c_int, /* = true */
    ) -> c_int;
}

// TODO(mr): Add some sort of basic tests or just rely on whatever has already been set up?
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
