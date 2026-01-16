use super::errors::GenerationRepositoryError;
use crate::entity::{EnergyRecord, GenerationId};

/// 発電状況を記録するためのリポジトリインターフェース
#[async_trait::async_trait]
pub trait IGenerationRepository {
    /// 発電状況を記録する
    ///
    /// # Arguments
    /// * `new` - 新規登録する発電状況
    /// # Returns
    /// * `Result<EnergyRecord, GenerationRepositoryError>` - 成功時は登録後のエンティティを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 記録に失敗した場合のエラー
    async fn add(&self, new: &EnergyRecord) -> Result<EnergyRecord, GenerationRepositoryError>;

    /// 発電状況を取得する
    ///
    /// # Arguments
    /// * `id` - 取得する発電状況のID
    /// # Returns
    /// * `Result<Generation, GenerationRepositoryError>` - 成功時は発電状況のエンティティを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn get(&self, id: GenerationId) -> Result<EnergyRecord, GenerationRepositoryError>;

    /// 発電状況を削除する
    ///
    /// # Arguments
    /// * `id` - 削除する発電状況のID
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 削除に失敗した場合のエラー
    async fn delete(&self, id: GenerationId) -> Result<(), GenerationRepositoryError>;
}
