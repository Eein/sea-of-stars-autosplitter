use asr::{future::next_tick, game_engine::unity::il2cpp::{Class, Module, Version}, user_settings::Settings, Address, Process};
use bytemuck::AnyBitPattern;

asr::async_main!(stable);

#[derive(Settings)]
struct Settings {
    /// My Setting
    #[default = true]
    my_setting: bool,
    // TODO: Change these settings.
}


async fn main() {
    // TODO: Set up some general state and settings.
    let settings = Settings::register();

    asr::print_message("Hello, World!");



    loop {
        let process = Process::wait_attach("SeaOfStars.exe").await;
                // We first attach to the Mono module. Here we know that the game is using IL2CPP 2020.
        let module = Module::wait_attach(&process, Version::V2020).await;
        // We access the .NET DLL that the game code is in.
// Use the current time.
        
        process
            .until_closes(async {
                // TODO: Change this to use the correct version of IL2CPP (or mono backend).
                let module = Module::wait_attach(&process, Version::V2020).await;
                let image = module.wait_get_default_image(&process).await;
                asr::print_message("Got Assembly-CSharp");
        
                let character_stats_class = image.wait_get_class(&process, &module, "CharacterStatsManager").await;
                asr::print_message("Got CharacterStatsManager");

                let instance = character_stats_class.wait_get_static_instance(&process, &module, "instance").await;
                asr::print_message("Got instance");
                
                let party_progress_data_address = character_stats_class.wait_get_field(&process, &module, "partyProgressData").await;
                asr::print_message("Got party progress data");
                
                if let Ok(party_data) = process.read::<u8>(instance + party_progress_data_address) {
                    asr::print_message(&party_data.to_string());
                 }
                // TODO: Load some initial information from the process.
                loop {
                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}
