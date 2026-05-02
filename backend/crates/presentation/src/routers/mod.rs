use sea_orm::DatabaseConnection;

mod consumption;
mod generation;
mod health;
pub(crate) mod root;

#[derive(Clone)]
struct RouterState {
    db: DatabaseConnection,
}
