
use crate::ffi;

pub struct Sound(ffi::ma_sound);





impl From<*mut miniaudio_sys::ma_sound> for Sound {
    #[inline]
    fn from(sound: *mut miniaudio_sys::ma_sound) -> Self {
        Sound(unsafe { *sound })
    }
}

impl Into<Sound> for ffi::ma_sound {
    #[inline]
    fn into(self) -> Sound {
        Sound(self)
    }
}


//For now, sounds can only be initialized from within an engine.




impl Drop for Sound {
    fn drop(&mut self) {
        unsafe {
            ffi::ma_sound_uninit(&mut self.0);
        }
    }
}
