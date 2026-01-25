mod generation;

pub fn get() -> poem::Route {
    poem::Route::new().nest("/generation", generation::get_router())
}
