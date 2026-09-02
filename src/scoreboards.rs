use crate::{
    define_crankstart_api,
    pd_api::{
        ctypes::c_char, playdate_scoreboards, AddScoreCallback, BoardsListCallback, PDBoardsList,
        PDScore, PDScoresList, PersonalBestCallback, ScoresCallback,
    },
};

define_crankstart_api! {
    #[allow(dead_code, non_snake_case)]
    pub struct ScoreboardsAPI => playdate_scoreboards {
        ; // No sub-API fields
        pub(crate) addScore: unsafe extern "C" fn(
            boardId: *const c_char,
            value: u32,
            callback: AddScoreCallback,
        ) -> i32,
        pub(crate) getPersonalBest: unsafe extern "C" fn(
            boardId: *const c_char,
            callback: PersonalBestCallback,
        ) -> i32,
        pub(crate) freeScore: unsafe extern "C" fn(score: *mut PDScore),
        pub(crate) getScoreboards: unsafe extern "C" fn(callback: BoardsListCallback) -> i32,
        pub(crate) freeBoardsList: unsafe extern "C" fn(boardsList: *mut PDBoardsList),
        pub(crate) getScores: unsafe extern "C" fn(
            boardId: *const c_char,
            callback: ScoresCallback,
        ) -> i32,
        pub(crate) freeScoresList: unsafe extern "C" fn(scoresList: *mut PDScoresList),
    }
}
