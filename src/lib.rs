// #![no_std]
#![feature(type_alias_impl_trait, const_async_blocks, ascii_char)]
#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::undocumented_unsafe_blocks,
    rust_2018_idioms
)]

use asr::{
    future::next_tick,
    game_engine::unity::il2cpp::{Module, Version},
    time::Duration,
    timer::{self, TimerState},
    watcher::Watcher,
    Address, Process,
};

use std::collections::HashSet;

// use libc_alloc::LibcAlloc;

// #[global_allocator]
// static ALLOCATOR: LibcAlloc = LibcAlloc;

// asr::panic_handler!();
asr::async_main!(nightly);

async fn main() {
    // Settings
    let settings = Settings::register();

    loop {
        // Hook to the target process
        let process = Process::wait_attach("SeaOfStars.exe").await;

        process
            .until_closes(async {
                // Once the target process has been found and attached to, set up the watchers in their default state
                let mut watchers = Watchers::default();
                let mut splits = HashSet::<String>::new();
                // Perform our memory scraping here
                let unity = Module::wait_attach(&process, Version::V2020).await;
                asr::print_limited::<128>(&"=> Found Unity module");

                let assembly = unity.wait_get_default_image(&process).await;
                asr::print_limited::<128>(&"=> Found Assembly-CSharp.exe");

                let title_sequence_manager = assembly
                    .wait_get_class(&process, &unity, "TitleSequenceManager")
                    .await;
                asr::print_limited::<128>(&"=> Found class: \"TitleSequenceManager\"");
                let title_sequence_manager_parent = title_sequence_manager
                    .wait_get_parent(&process, &unity)
                    .await;
                asr::print_limited::<128>(&"   => Found parent");
                let mut title_sequence_manager_parent_instance: Option<u32> = None;
                let mut title_sequence_manager_character_selection_screen: Option<u32> = None;
                let mut title_sequence_manager_parent_static_table: Option<Address> = None;

                let character_selection_screen = assembly
                    .wait_get_class(&process, &unity, "CharacterSelectionScreen")
                    .await;
                asr::print_limited::<128>(&"=> Found class: \"CharacterSelectionScreen\"");
                let mut character_selection_screen_character_selected: Option<u32> = None;

                let combat_manager_class = assembly
                    .wait_get_class(&process, &unity, "CombatManager")
                    .await;
                asr::print_limited::<128>(&"=> Found class: \"CombatManager\"");
                let combat_manager_class_parent =
                    combat_manager_class.wait_get_parent(&process, &unity).await;
                asr::print_limited::<128>(&"   => Found parent");
                let mut combat_manager_class_parent_instance: Option<u32> = None;
                let mut combat_manager_class_parent_static_table: Option<Address> = None;
                let mut combat_manager_class_current_encounter: Option<u32> = None;

                let level_manager_class = assembly
                    .wait_get_class(&process, &unity, "LevelManager")
                    .await;
                asr::print_limited::<128>(&"=> Found class: \"LevelManager\"");
                let level_manager_class_parent =
                    level_manager_class.wait_get_parent(&process, &unity).await;
                asr::print_limited::<128>(&"   => Found parent");
                let mut level_manager_class_parent_instance: Option<u32> = None;
                let mut level_manager_class_loading: Option<u32> = None;
                let mut level_manager_class_parent_static_table: Option<Address> = None;
                let mut current_enemy = 0;

                asr::print_limited::<128>(&"=> Autosplitter ready");

                loop {
                    // Looking for offsets
                    if title_sequence_manager_parent_instance.is_none() {
                        title_sequence_manager_parent_instance =
                            title_sequence_manager_parent.get_field(&process, &unity, "instance");
                    }

                    if title_sequence_manager_character_selection_screen.is_none() {
                        title_sequence_manager_character_selection_screen = title_sequence_manager
                            .get_field(&process, &unity, "characterSelectionScreen");
                    }

                    if title_sequence_manager_parent_static_table.is_none() {
                        title_sequence_manager_parent_static_table =
                            title_sequence_manager_parent.get_static_table(&process, &unity);
                    }

                    if character_selection_screen_character_selected.is_none() {
                        character_selection_screen_character_selected = character_selection_screen
                            .get_field(&process, &unity, "characterSelected");
                    }

                    if combat_manager_class_parent_instance.is_none() {
                        combat_manager_class_parent_instance =
                            combat_manager_class_parent.get_field(&process, &unity, "instance");
                    }

                    if combat_manager_class_parent_static_table.is_none() {
                        combat_manager_class_parent_static_table =
                            combat_manager_class_parent.get_static_table(&process, &unity);
                    }
                    if combat_manager_class_current_encounter.is_none() {
                        combat_manager_class_current_encounter =
                            combat_manager_class.get_field(&process, &unity, "currentEncounter");
                    }

                    if level_manager_class_parent_instance.is_none() {
                        level_manager_class_parent_instance =
                            level_manager_class_parent.get_field(&process, &unity, "instance");
                    }

                    if level_manager_class_loading.is_none() {
                        level_manager_class_loading =
                            level_manager_class.get_field(&process, &unity, "loadingLevel");
                    }

                    if level_manager_class_parent_static_table.is_none() {
                        level_manager_class_parent_static_table =
                            level_manager_class_parent.get_static_table(&process, &unity);
                    }

                    // Your update logic
                    let start_autosplitter = if title_sequence_manager_parent_instance.is_some()
                        && title_sequence_manager_parent_static_table.is_some()
                        && title_sequence_manager_character_selection_screen.is_some()
                        && character_selection_screen_character_selected.is_some()
                    {
                        process
                            .read_pointer_path64::<u8>(
                                title_sequence_manager_parent_static_table.unwrap_or_default(),
                                &[
                                    title_sequence_manager_parent_instance
                                        .unwrap_or_default()
                                        .into(),
                                    title_sequence_manager_character_selection_screen
                                        .unwrap_or_default()
                                        .into(),
                                    character_selection_screen_character_selected
                                        .unwrap_or_default()
                                        .into(),
                                ],
                            )
                            .unwrap_or_default()
                            > 0
                    } else {
                        false
                    };
                    watchers
                        .start_autosplitter
                        .update_infallible(start_autosplitter);

                    let loading = if level_manager_class_parent_instance.is_some()
                        && level_manager_class_loading.is_some()
                        && level_manager_class_parent_static_table.is_some()
                    {
                        process
                            .read_pointer_path64::<u8>(
                                level_manager_class_parent_static_table.unwrap_or_default(),
                                &[
                                    level_manager_class_parent_instance
                                        .unwrap_or_default()
                                        .into(),
                                    level_manager_class_loading.unwrap_or_default().into(),
                                ],
                            )
                            .unwrap_or_default()
                            > 0
                    } else {
                        true
                    };
                    watchers.loading.update_infallible(loading);

                    if combat_manager_class_parent_instance.is_some()
                        && combat_manager_class_current_encounter.is_some()
                        && combat_manager_class_parent_static_table.is_some()
                    {
                        let enemy = process
                            .read_pointer_path64::<u64>(
                                combat_manager_class_parent_static_table.unwrap_or_default(),
                                &[
                                    combat_manager_class_parent_instance
                                        .unwrap_or_default()
                                        .into(),
                                    combat_manager_class_current_encounter
                                        .unwrap_or_default()
                                        .into(),
                                    0x130,
                                    0x10,
                                    0x20,
                                    0x58,
                                    0xF0,
                                    0xD8,
                                    0x18,
                                    0x14,
                                ],
                            )
                            .unwrap_or_default();
                        if enemy != 0 {
                            current_enemy = enemy
                        }

                        // In the future read the utf8 string here - requires nightly for .as_str()
                    };

                    let enemy_0_hp = if combat_manager_class_parent_instance.is_some()
                        && combat_manager_class_current_encounter.is_some()
                        && combat_manager_class_parent_static_table.is_some()
                    {
                        let hp = process
                            .read_pointer_path64::<u32>(
                                combat_manager_class_parent_static_table.unwrap_or_default(),
                                &[
                                    combat_manager_class_parent_instance
                                        .unwrap_or_default()
                                        .into(),
                                    combat_manager_class_current_encounter
                                        .unwrap_or_default()
                                        .into(),
                                    0x130,
                                    0x10,
                                    0x20,
                                    0x6C,
                                ],
                            )
                            .unwrap_or_default();
                        hp
                    } else {
                        99999
                    };

                    watchers.enemy_0_hp.update_infallible(enemy_0_hp);
                    // The update logic ends here

                    // Splitting logic
                    let timer_state = timer::state();
                    if timer_state == TimerState::Running || timer_state == TimerState::Paused {
                        if let Some(is_loading) = is_loading(&watchers, &settings) {
                            if is_loading {
                                timer::pause_game_time()
                            } else {
                                timer::resume_game_time()
                            }
                        }

                        if let Some(game_time) = game_time(&watchers, &settings) {
                            timer::set_game_time(game_time)
                        }

                        if reset(&watchers, &settings) {
                            timer::reset()
                        } else {
                            if settings.chromatic_apparition
                                && current_enemy == 27866233151488101
                                && watchers.enemy_0_hp.pair.unwrap().current == 0
                            {
                                split(&mut splits, "final_split")
                            }
                        }
                    }

                    if timer::state() == TimerState::NotRunning && start(&watchers, &settings) {
                        splits = HashSet::<String>::new();
                        timer::start();
                        timer::pause_game_time();

                        if let Some(is_loading) = is_loading(&watchers, &settings) {
                            if is_loading {
                                timer::pause_game_time()
                            } else {
                                timer::resume_game_time()
                            }
                        }
                    }

                    next_tick().await;
                }
            })
            .await;
    }
}

fn is_loading(watchers: &Watchers, settings: &Settings) -> Option<bool> {
    if !settings.load_removal {
        Some(false)
    } else {
        Some(watchers.loading.pair?.current)
    }
}

fn game_time(_watchers: &Watchers, _settings: &Settings) -> Option<Duration> {
    None
}

fn reset(_watchers: &Watchers, _settings: &Settings) -> bool {
    false
}

fn start(watchers: &Watchers, settings: &Settings) -> bool {
    if !settings.start_autosplitter {
        return false;
    }
    let Some(start_autosplitter) = &watchers.start_autosplitter.pair else {
        return false;
    };
    !start_autosplitter.old && start_autosplitter.current
}

fn split(splits: &mut HashSet<String>, key: &str) {
    if !splits.contains(key) {
        splits.insert(key.to_string());
        let split_message = format!("splitting: {}", &key.to_string());
        asr::print_message(&split_message);
        timer::split()
    }
}
#[derive(asr::user_settings::Settings)]
struct Settings {
    #[default = true]
    /// Load Removal
    load_removal: bool,
    #[default = true]
    /// Automatic Start on character select
    start_autosplitter: bool,
    #[default = true]
    /// Split on defeating Chromatic Apparition
    chromatic_apparition: bool,
}

#[derive(Default)]
struct Watchers {
    start_autosplitter: Watcher<bool>,
    loading: Watcher<bool>,
    enemy_0_hp: Watcher<u32>,
}
