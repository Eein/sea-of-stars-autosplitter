#[derive(asr::user_settings::Settings, Clone)]
pub struct Settings {
    /// Load Removal
    pub load_removal: bool,
    #[default = true]
    /// Automatic Start on character select
    pub start: bool,
    #[default = true]
    /// Split on defeating Chromatic Apparition
    pub chromatic_apparition: bool,
}
