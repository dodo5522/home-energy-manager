use super::errors::GenerationRepositoryError;
use layer_domain::{entity::SourceRecord, value_object::EnergySource};

/// 発電元を記録するためのリポジトリインターフェース
#[async_trait::async_trait]
pub trait ISourceRepository {
    /// 発電元を追加する
    ///
    /// # Arguments
    /// * `new` - 新規登録する発電状況
    /// # Returns
    /// * `Result<EnergySource, GenerationRepositoryError>` - 成功時は登録後の発電元を返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 記録に失敗した場合のエラー
    async fn add(&self, new: &SourceRecord) -> Result<EnergySource, GenerationRepositoryError>;

    /// 発電元を取得する
    ///
    /// # Returns
    /// * `Result<Vec<SourceRecord>, GenerationRepositoryError>` - 成功時は発電元のエンティティを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn get(&self) -> Result<Vec<SourceRecord>, GenerationRepositoryError>;

    /// 発電元が存在するか確認する
    ///
    /// # Arguments
    /// * `system` - 削除する発電元
    /// # Returns
    /// * `Result<bool, GenerationRepositoryError>` - 成功時は存在するかどうかを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn has(&self, system: &EnergySource) -> Result<bool, GenerationRepositoryError>;

    /// 発電元を削除する
    ///
    /// # Arguments
    /// * `system` - 削除する発電元
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 削除に失敗した場合のエラー
    async fn delete(&self, system: &EnergySource) -> Result<(), GenerationRepositoryError>;
}
