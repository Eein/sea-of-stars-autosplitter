use asr::{
    future::next_tick,
    timer::{self, TimerState},
    watcher::Pair,
    watcher::Watcher,
    Process,
};
mod settings;
use settings::Settings;
use std::collections::HashSet;

asr::async_main!(stable);

async fn main() {
    // TODO: Set up some general state and settings.
    let mut start_watcher = Watcher::<u8>::new(); //
    let mut final_boss_watcher = Watcher::<u64>::new(); //
                                                        // let mut loading_watcher = Watcher::<u8>::new(); // CurrentGameChapterID
                                                        // let mut near_future_scenario_progress_watcher = Watcher::<u32>::new(); // CurrentGameChapterID
    let mut splits = HashSet::<String>::new();
    let settings = Settings::register();
    loop {
        let process = match asr::get_os().ok().unwrap().as_str() {
            _ => Process::wait_attach("SeaOfStars.exe").await,
        };
        let (main_module_base, _main_module_size) =
            process.wait_module_range("GameAssembly.dll").await;

        asr::print_message("Captured gameassembly");

        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                loop {
                    let start_lookup = start_watcher.update(
                        process
                            .read_pointer_path64(
                                main_module_base,
                                &vec![0x2EAA630, 0xB8, 0x10, 0x70, 0x0 + 0xDA],
                            )
                            .ok(),
                    );
                    let start = match start_lookup {
                        Some(start_value) => start_value,
                        None => &Pair { old: 0, current: 0 },
                    };

                    //                     "GameAssembly.dll"+02EAAB30
                    // B8
                    // 10
                    // 118
                    // 10
                    // 20
                    // 100
                    // 18+14 (String)

                    let final_boss_name_lookup: Option<&Pair<u64>> = final_boss_watcher.update(
                        process
                            .read_pointer_path64(
                                main_module_base,
                                &vec![
                                    0x2EAAB30, 0xB8, 0x10, 0xF0, 0x118, 0x10, 0x20, 0x100, 0x18,
                                    0x14,
                                ],
                            )
                            .ok(),
                    );
                    let final_boss_name = match final_boss_name_lookup {
                        Some(final_boss_name) => {
                            asr::print_message(&final_boss_name.to_string());
                            final_boss_name
                        }
                        None => &Pair { old: 0, current: 0 },
                    };
                    // asr::print_message(&start.old.to_string());
                    // let loading = loading_watcher
                    //     .update(
                    //         process
                    //             .read_pointer_path64(
                    //                 main_module_base,
                    //                 &vec![0x5092A98, 0x8, 0x10, 0x50, 0x30, 0x3FA],
                    //             )
                    //             .ok(),
                    //     )
                    //     .unwrap();

                    // Scenario Progress

                    match timer::state() {
                        TimerState::NotRunning => {
                            if settings.start && start.old == 0 && start.current == 1 {
                                // asr::print_message("Clearing Splits and Starting");
                                splits = HashSet::<String>::new();
                                timer::start();
                            }
                        }
                        TimerState::Running => {
                            // CHAPTER SPLITS

                            if settings.chromatic_apparition
                                && final_boss_name.old == 14918388517371959
                            {
                                split(&mut splits, "final_split")
                            }

                            // if settings.load_removal {
                            //     // load/save removal

                            //     if loading.old == 0 && loading.current == 1 {
                            //         // asr::print_message("resuming game time");
                            //         timer::resume_game_time()
                            //     }

                            //     if loading.old == 1 && loading.current == 0 {
                            //         // asr::print_message("pausing game time");
                            //         timer::pause_game_time()
                            //     }
                            // }
                        }
                        _ => {}
                    }
                    // TODO: Do something on every tick.
                    next_tick().await;
                }
            })
            .await;
    }
}

pub fn split(splits: &mut HashSet<String>, key: &str) {
    if !splits.contains(key) {
        splits.insert(key.to_string());
        asr::print_message(&key.to_string());
        timer::split()
    }
}
