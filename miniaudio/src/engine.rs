use crate::{ffi, sound::Sound};




pub struct EngineConfig(ffi::ma_engine_config);


impl EngineConfig {
    pub fn new() -> EngineConfig {
        unsafe {
        EngineConfig { 0: ffi::ma_engine_config_init() }
        }
    }
}


pub struct Fence(ffi::ma_fence);


pub struct Engine(ffi::ma_engine);




impl Engine {

    pub fn init(engine_config: EngineConfig) -> Result<Engine, i32> {
        unsafe {
            // Allocate memory for ma_engine on the heap
            let ma_engine: Box<ffi::ma_engine> = Box::new(std::mem::zeroed());
            
            

            // Get a raw pointer to the allocated memory
            let raw_ma_engine = Box::into_raw(ma_engine);

            // Pass the raw pointer to the allocated memory
            let result = ffi::ma_engine_init(&engine_config.0 as *const _, raw_ma_engine);

            if result != ffi::MA_SUCCESS {
                println!("miniaudio-rs: Failed to initialize engine. err: {}", result);

                // Reclaim ownership of the Box to ensure it gets deallocated
                let _ = Box::from_raw(raw_ma_engine);

                return Err(result);
            }

            // Move ownership of the Box into the Engine struct
            return Ok(Engine { 0: *Box::from_raw(raw_ma_engine) });
        }
    }


    #[inline]
    pub fn play_sound(&mut self, file_path: &str, group: Option<i8>) {
        // Convert the Rust string to a C-style string
        let c_file_path = std::ffi::CString::new(file_path).expect("miniaudio-rs: CString conversion failed");

        unsafe {
            ffi::ma_engine_play_sound(&mut self.0, c_file_path.as_ptr(), 
                if let Some(group_used) = group { group_used as *mut _ }  else { std::ptr::null_mut() }
        );
        }
    }


    ///Set the engine's volume. 0.0 is silent, 1.0 is full volume.
    /// To set the volume of a specific sound, use `Sound::set_volume()`. 
    /// If you want to set it for a group, use `SoundGroup::set_volume()`. (TODO)
    #[inline]
    pub fn set_volume(&mut self, volume: f32) {
        unsafe {
            ffi::ma_engine_set_volume(&mut self.0, volume);
        }
    }

    //manually start the engine.
    #[inline]
    pub fn start(&mut self) {
        unsafe {
            ffi::ma_engine_start(&mut self.0);
        }
    }

    //manually stop the engine.
    #[inline]
    pub fn stop(&mut self) {
        unsafe {
            ffi::ma_engine_stop(&mut self.0);
        }
    }


    #[inline]
    pub fn set_listener_position(&mut self, listener_index: u32, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::ma_engine_listener_set_position(&mut self.0, listener_index, x, y, z);
        }
    }


    //Implement
    //ma_engine_listener_set_direction(&engine, listenerIndex, forwardX, forwardY, forwardZ);
    //ma_engine_listener_set_world_up(&engine, listenerIndex, 0, 1, 0);


    #[inline]
    pub fn set_listener_velocity(&mut self, listener_index: u32, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::ma_engine_listener_set_velocity(&mut self.0, listener_index, x, y, z);
        }
    }

    #[inline]
    pub fn set_listener_direction(&mut self, listener_index: u32, forward_x: f32, forward_y: f32, forward_z: f32, ) {
        unsafe {
            ffi::ma_engine_listener_set_direction(&mut self.0, listener_index, forward_x, forward_y, forward_z);
        }
    }


    #[inline]
    pub fn set_listener_world_up(&mut self, listener_index: u32, up_x: f32, up_y: f32, up_z: f32, ) {
        unsafe {
            ffi::ma_engine_listener_set_world_up(&mut self.0, listener_index, up_x, up_y, up_z);
        }
    }

    #[inline]
    pub fn set_listener_cone(&mut self, listener_index: u32, inner_angle_in_degrees: f32, outer_angle_in_degrees: f32, outer_gain: f32) {
        unsafe {
            ffi::ma_engine_listener_set_cone(&mut self.0, listener_index, inner_angle_in_degrees, outer_angle_in_degrees, outer_gain);
        }
    }



    #[inline]
    pub fn get_device(&mut self) {
        unsafe {
            ffi::ma_engine_get_device(&mut self.0);
        }
    }



    /*
    




    Currenty this function does not correctly interface with the C api. causing a -2 (MA_INVALID_ARGS) error.
    
     */

    #[inline]
    pub fn init_sound_from_file(&mut self, file_path: &str, flags: u32, group: Option<i8>, fence: Option<&mut Fence>) -> Result<Sound, i32> {
        // Convert the Rust string to a C-style string
        let c_file_path = match std::ffi::CString::new(file_path) {
            Ok(cstr) => {cstr},
            Err(_) => { return Err(-1);},
        };



        unsafe {
            let ma_sound: Box<ffi::ma_sound> = Box::new(std::mem::zeroed());

            let ptr_ma_sound = Box::into_raw(ma_sound);

            let result = ffi::ma_sound_init_from_file(
                &mut self.0, 
                c_file_path.as_ptr(), 
                flags,
                if let Some(group_used) = group { group_used as *mut _ }  else { std::ptr::null_mut() },
                if let Some(fence_used) = fence { &mut fence_used.0 as *mut _ }  else { std::ptr::null_mut() },
                ptr_ma_sound
                
            );

            if result != ffi::MA_SUCCESS {
                println!("miniaudio-rs: Failed to initialize sound from file. err: {}", result);

                // Reclaim ownership of the Box to ensure it gets deallocated
                let _ = Box::from_raw(ptr_ma_sound);

                return Err(result);
            }
            //Should I leave this to `From`` (remove *), or remove `From` impl?
            return Ok((*ptr_ma_sound).into());
    }
    }




    





}


impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            ffi::ma_engine_uninit(&mut self.0);
        }
    }
}


