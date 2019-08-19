// TODO(mr): Document, or disable warnings and point to online docs

use soloud_rust_sys as sys;
use std::ffi::CStr;
use std::ptr::NonNull;
use libc::{c_int, c_uint, c_float};
use std::convert::TryInto;

// TODO(mr): These are always fatal right?
fn unwrap(soloud: &mut SoLoud, result: c_int) {
    unsafe {
        if result != 0 {
            let message = sys::Soloud_getErrorString(soloud.0.as_ptr(), result);
            panic!("Error {}: {}", result, CStr::from_ptr(message).to_str().unwrap());
            // TODO(mr): Free the message?
        }
    }
}

pub struct SoLoudBuilder {
    flags: c_uint,
    backend: c_uint, // TODO: Expose this option?
    sample_rate: c_uint,
    buffer_size: c_uint,
    channels: c_uint,
}

impl SoLoudBuilder {
    pub fn new() -> Self {
        Self {
            flags: sys::SOLOUD_CLIP_ROUNDOFF,
            backend: sys::SOLOUD_AUTO,
            sample_rate: sys::SOLOUD_AUTO,
            buffer_size: sys::SOLOUD_AUTO,
            channels: 2,
        }
    }

    // TODO(mr): Make type safe?
    pub fn flags(&mut self, flags: c_uint) -> &mut Self {
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
                self.flags,
                self.backend,
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

    // TODO(mr): Immutable correct? (same for other play method)
    pub fn play(&mut self, audio_source: &AudioSource) -> c_uint {
        unsafe {
            sys::Soloud_play(self.0.as_ptr(), audio_source.raw())
        }
    }

    // TODO(mr): Is there a max voice count? if so, switch to an explicitly sized type here. Sort out
    // the floats in the API as well.
    // TODO(mr): naming "get_" or no?
    pub fn get_voice_count(&self) -> c_uint {
        unsafe {
            sys::Soloud_getVoiceCount(self.0.as_ptr())
        }
    }

    // TODO(mr): What happens if passing in a channel that doesn't exist?
    pub fn get_approximate_volume(&self, channel: c_uint) -> c_float {
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
    // TODO(mr): It's not unsafe to have it uninitialized right?
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
    // TODO(mr): It's not unsafe to have it uninitialized right?
    pub fn new() -> Self {
        unsafe {
            Self(NonNull::new(sys::Queue_create()).unwrap())
        }
    }

    // TODO(mr): int vs uint for other play..?
    pub fn play(&mut self, audio_source: &AudioSource) -> c_int {
        unsafe {
            sys::Queue_play(self.0.as_ptr(), audio_source.raw())
        }
    }

    // TODO(mr): Naming...
    pub fn get_queue_count(&mut self) -> c_uint {
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
    // TODO(mr): safe before initialized?
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
            // TODO(mr): cast to mutable okay?
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

// TODO(mr): ...
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
