mod history;

pub fn get_router() -> poem::Route {
    poem::Route::new().at("/", poem::get(history::add_history))
}
