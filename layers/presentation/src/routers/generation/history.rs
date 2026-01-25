use crate::di::db::get_connection;
use layer_domain::{entity, repository::IGenerationRepository};
use layer_infra_db::repository::generation::GenerationRepository;

#[poem::handler]
pub async fn add_history() -> &'static str {
    if let Ok(db) = get_connection().await
        && let Ok(repo) = GenerationRepository::new(db).await
    {
        let e = entity::EnergyRecord {
            id: None,
            unit: "kWh".try_into().unwrap(),
            sub_system: "Battery".try_into().unwrap(),
            energy_source: "Solar".try_into().unwrap(),
            label: "hoge".to_string(),
            value: 1.0,
            monitored_at: chrono::Local::now(),
        };
        println!("Inserting test record: {:?}", e);
        let res = repo.add(&e).await;

        match res {
            Ok(_) => "Successful.",
            Err(_) => "DB error.",
        }
    } else {
        "Environment variables for DB connection are not set."
    }
}
