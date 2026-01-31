mod generation;
mod swagger;
pub use swagger::Api;

pub fn get() -> poem::Route {
    poem::Route::new().nest("/generation", generation::get_router())
}
