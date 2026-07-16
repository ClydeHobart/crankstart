use {
    self::{
        display::DisplayAPI, file::FileAPI, graphics::GraphicsAPI, json::JSONAPI, lua::LuaAPI,
        scoreboards::ScoreboardsAPI, sound::SoundAPI, sprite::SpriteAPI, sys::SysAPI,
    },
    crate::{define_crankstart_api, PlaydateAPI},
};

pub mod display;
pub mod file;
pub mod graphics;
pub mod json;
pub mod lua;
pub mod scoreboards;
pub mod sound;
pub mod sprite;
pub mod sys;

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct API => PlaydateAPI {
        pub display: DisplayAPI,
        pub file: FileAPI,
        pub graphics: GraphicsAPI,
        pub json: JSONAPI,
        pub lua: LuaAPI,
        pub scoreboards: ScoreboardsAPI,
        pub sound: SoundAPI,
        pub sprite: SpriteAPI,
        pub system: SysAPI;
        // No fn fields.
    }
}
