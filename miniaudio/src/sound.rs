
use crate::ffi;

pub struct Sound(ffi::ma_sound);


impl Sound {

    #[inline]
    pub fn from_raw(sound: ffi::ma_sound) -> Self {
        Self(sound)
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
