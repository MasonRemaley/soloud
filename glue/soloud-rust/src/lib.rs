use soloud_rust_sys as sys;
use std::ffi::CStr;
use std::ptr::NonNull;
use libc::{c_int, c_uint, c_float};
use std::convert::TryInto;

#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct BuilderFlags: c_uint {
        const CLIP_ROUNDOFF = sys::SOLOUD_CLIP_ROUNDOFF;
        const ENABLE_VISUALIZATION = sys::SOLOUD_ENABLE_VISUALIZATION;
        const LEFT_HANDED_3D = sys::SOLOUD_LEFT_HANDED_3D;
    }
}

// NOTE: I'm disabling the dead code warning for this enum. It's never exposed to users (see the
// NOTE on the builder's backend field), but I left this code here so you can see how to make a Rust
// enum that lines up w/ your constants.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum Backend {
    Auto = sys::SOLOUD_AUTO as isize,
    Sdl1 = sys::SOLOUD_SDL1 as isize,
    Sdl2 = sys::SOLOUD_SDL2 as isize,
    Portaudio = sys::SOLOUD_PORTAUDIO as isize,
    Winmm = sys::SOLOUD_WINMM as isize,
    Xaudio2 = sys::SOLOUD_XAUDIO2 as isize,
    Wasapi = sys::SOLOUD_WASAPI as isize,
    Alsa = sys::SOLOUD_ALSA as isize,
    Oss = sys::SOLOUD_OSS as isize,
    Openal = sys::SOLOUD_OPENAL as isize,
    Coreaudio = sys::SOLOUD_COREAUDIO as isize,
    Opensles = sys::SOLOUD_OPENSLES as isize,
    VitaHomebrew = sys::SOLOUD_VITA_HOMEBREW as isize,
    Nulldriver = sys::SOLOUD_NULLDRIVER as isize,
}

fn unwrap(soloud: &mut SoLoud, result: c_int) {
    unsafe {
        if result != 0 {
            let message = sys::Soloud_getErrorString(soloud.0.as_ptr(), result);
            panic!("Error {}: {}", result, CStr::from_ptr(message).to_str().unwrap());
        }
    }
}

pub struct Builder {
    flags: BuilderFlags,
    // NOTE: I'm not exposing this option right now, since you need to decide which backends you
    // want available while building. If you'd like to allow having multiple available at once lmk.
    backend: Backend,
    sample_rate: c_uint,
    buffer_size: c_uint,
    channels: c_uint,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            flags: BuilderFlags::CLIP_ROUNDOFF,
            backend: Backend::Auto,
            sample_rate: sys::SOLOUD_AUTO,
            buffer_size: sys::SOLOUD_AUTO,
            channels: 2,
        }
    }

    pub fn flags(&mut self, flags: BuilderFlags) -> &mut Self {
        self.flags = flags;
        self
    }

    pub fn sample_rate(&mut self, sample_rate: c_uint) -> &mut Self {
        self.sample_rate = sample_rate;
        self
    }

    pub fn buffer_size(&mut self, buffer_size: c_uint) -> &mut Self {
        self.buffer_size = buffer_size;
        self
    }

    pub fn channels(&mut self, channels: c_uint) -> &mut Self {
        self.channels = channels;
        self
    }

    pub fn init(&self) -> SoLoud {
        unsafe {
            let mut soloud = SoLoud(NonNull::new(sys::Soloud_create()).unwrap());
            let result = sys::Soloud_initEx(
                soloud.0.as_ptr(),
                self.flags.bits(),
                self.backend as c_uint,
                self.sample_rate,
                self.buffer_size,
                self.channels,
            );
            unwrap(&mut soloud, result);
            soloud
        }
    }
}

pub struct SoLoud(NonNull<sys::Soloud>);

impl SoLoud {
    pub fn set_global_volume(&mut self, volume: f32) {
        unsafe {
            sys::Soloud_setGlobalVolume(self.0.as_ptr(), volume);
        }
    }

    // NOTE: The Rust API is treating the audio source as immutable in all play methods, even though
    // the C code isn't, as I think that's accurately describing the semantics.
    pub fn play(&mut self, audio_source: &dyn AudioSource) -> c_uint {
        unsafe {
            sys::Soloud_play(self.0.as_ptr(), audio_source.raw())
        }
    }

    // NOTE: If there's a max possible voice count, it would be friendlier to other Rust code to
    // replace `c_uint` here with an explicitly sized type (e.g. `u8`).
    pub fn voice_count(&self) -> c_uint {
        unsafe {
            sys::Soloud_getVoiceCount(self.0.as_ptr())
        }
    }

    // NOTE: If you expect floats to always be 32 bit, it would be friendlier to other Rust code to
    // replace all public mentions of `c_float` in the `soloud-rust` crate with `f32`.
    // NOTE: I'm not sure what happens if you pass in a channel that doesn't exist here
    pub fn approximate_volume(&self, channel: c_uint) -> c_float {
        unsafe {
            sys::Soloud_getApproximateVolume(self.0.as_ptr(), channel)
        }
    }
}

impl Drop for SoLoud {
    fn drop(&mut self) {
        unsafe {
            sys::Soloud_deinit(self.0.as_ptr());
            sys::Soloud_destroy(self.0.as_ptr());
        }
    }
}

pub struct Speech(NonNull<sys::Speech>);

impl Speech {
    // NOTE: This API is assuming that creating a sound and not initializing it does something
    // predictable, like have it be empty. Same for `Queue`, etc.
    pub fn new() -> Self {
        unsafe {
            Self(NonNull::new(sys::Speech_create()).unwrap())
        }
    }

    pub fn set_text(&mut self, text: &CStr) {
        unsafe {
            assert_eq!(sys::Speech_setText(self.0.as_ptr(), text.as_ptr()), 0);
        }
    }
}

impl Drop for Speech {
    fn drop(&mut self) {
        unsafe {
            sys::Speech_destroy(self.0.as_ptr());
        }
    }
}

pub struct Queue(NonNull<sys::Queue>);

impl Queue {
    pub fn new() -> Self {
        unsafe {
            Self(NonNull::new(sys::Queue_create()).unwrap())
        }
    }

    // NOTE: I noticed that `Queue_play` returns an int, and `Soloud_play` returns a signed one. Is
    // this intentional? I'd also switch all of these to fixed size integer types in the Rust API if
    // possible.
    pub fn play(&mut self, audio_source: &dyn AudioSource) -> c_int {
        unsafe {
            sys::Queue_play(self.0.as_ptr(), audio_source.raw())
        }
    }

    pub fn queue_count(&mut self) -> c_uint {
        unsafe {
            sys::Queue_getQueueCount(self.0.as_ptr())
        }
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            sys::Queue_destroy(self.0.as_ptr());
        }
    }
}

pub struct Wav(NonNull<sys::Wav>);

impl Wav {
    pub fn new() -> Self {
        unsafe {
            Wav(NonNull::new(sys::Wav_create()).unwrap())
        }
    }

    // TODO(mr): copy/take ownership? should be possible to allow variants with and without those
    // safely, just gotta make sure we correctly understand them first so lets first get the basics
    // working. Also naming/such?.
    pub fn load_raw_wave_ex(&mut self, buffer: &[c_float], sample_rate: c_float, channels: c_uint) {
        unsafe {
            assert_eq!(sys::Wav_loadRawWaveEx(self.0.as_ptr(), buffer.as_ptr() as *mut c_float, buffer.len().try_into().unwrap(), sample_rate, channels, 1, 0), 0);
        }
    }
}

impl Drop for Wav {
    fn drop(&mut self) {
        unsafe {
            sys::Wav_destroy(self.0.as_ptr());
        }
    }
}



// TODO: Organize better...
/// Private to prevent access to implementation details, and new implementations of the traits.
mod private {
    use super::*;
    pub trait AudioSourceSealed {
        fn raw(&self) -> *mut sys::AudioSource;
    }
}

pub trait AudioSource: private::AudioSourceSealed {
    // TODO(mr): Implement some of the public methods here...
}

impl AudioSource for Speech {}
impl private::AudioSourceSealed for Speech {
    fn raw(&self) -> *mut sys::AudioSource {
        self.0.as_ptr() as *mut sys::AudioSource
    }
}

impl AudioSource for Queue {}
impl private::AudioSourceSealed for Queue {
    fn raw(&self) -> *mut sys::AudioSource {
        self.0.as_ptr() as *mut sys::AudioSource
    }
}

impl AudioSource for Wav {}
impl private::AudioSourceSealed for Wav {
    fn raw(&self) -> *mut sys::AudioSource {
        self.0.as_ptr() as *mut sys::AudioSource
    }
}

// NOTE: If you want to set up tests for the Rust API, they'd go here (or alternatively in a file
// called tests). I'm happy to help with this if it's something you want and you know what the tests
// should be.
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
