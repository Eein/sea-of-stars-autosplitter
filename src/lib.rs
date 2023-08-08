use asr::{future::next_tick, game_engine::unity::il2cpp::{Module, Version}, user_settings::Settings, Process};

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
        process
            .until_closes(async {
                // TODO: Change this to use the correct version of IL2CPP (or mono backend).
                let module = Module::wait_attach(&process, Version::V2020).await;
                let image = module.wait_get_default_image(&process).await;

                // TODO: Load some initial information from the process.
                loop {
                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}
