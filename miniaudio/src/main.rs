//TEMP TEST FOR AUDIO

use miniaudio::engine::Engine;


fn main() {



    let result_engine = Engine::init(miniaudio::engine::EngineConfig::new());


    assert_eq!(0, if let Err(error_code) = result_engine {error_code} else {0});

    let mut engine = result_engine.unwrap();



    engine.play_sound("C:/Users/dev/Documents/code/rust/miniaudio-rs-new/miniaudio/examples/assets/exit.wav", None);



    std::thread::sleep(std::time::Duration::from_secs(5));

    //Manual test here. Sadly, I don't know how to automate this. Other than compiler errors / panic
    assert_eq!(1, 1);

}