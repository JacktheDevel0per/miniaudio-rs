
use miniaudio::engine::Engine;



fn main() {

}


#[test]
fn simple_engine_sound_load() {

    let result_engine = Engine::init(miniaudio::engine::EngineConfig::new());


    assert_eq!(0, if let Err(error_code) = result_engine {error_code} else {0});

    let mut engine = result_engine.unwrap();

    
    let raw_sound = engine.init_sound_from_file("./miniaudio/examples/assets/exit.wav", 0, None, None);

    assert_eq!(0, if let Err(error_code) = raw_sound {error_code} else {0});

    
    engine.play_sound("miniaudio/examples/assets/exit.wav", None);



    std::thread::sleep(std::time::Duration::from_secs(5));

    //Manual test here. Sadly, I don't know how to automate this. Other than compiler errors / panic
    assert_eq!(1, 1);
}