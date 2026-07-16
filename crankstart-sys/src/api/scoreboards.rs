use crate::{
    ctypes::c_char, define_crankstart_api, playdate_scoreboards, AddScoreCallback,
    BoardsListCallback, PDBoardsList, PDScore, PDScoresList, PersonalBestCallback, ScoresCallback,
};

define_crankstart_api! {
    #[allow(dead_code)]
    pub(crate) struct ScoreboardsAPI => playdate_scoreboards {
        ; // No sub-API fields
        pub addScore: unsafe extern "C" fn(
            boardId: *const c_char,
            value: u32,
            callback: AddScoreCallback,
        ) -> i32,
        pub getPersonalBest: unsafe extern "C" fn(
            boardId: *const c_char,
            callback: PersonalBestCallback,
        ) -> i32,
        pub freeScore: unsafe extern "C" fn(score: *mut PDScore),
        pub getScoreboards: unsafe extern "C" fn(callback: BoardsListCallback) -> i32,
        pub freeBoardsList: unsafe extern "C" fn(boardsList: *mut PDBoardsList),
        pub getScores: unsafe extern "C" fn(
            boardId: *const c_char,
            callback: ScoresCallback,
        ) -> i32,
        pub freeScoresList: unsafe extern "C" fn(scoresList: *mut PDScoresList),
    }
}
