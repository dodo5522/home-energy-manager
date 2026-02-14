use super::errors::GenerationRepositoryError;
use layer_domain::entity::LabelRecord;

/// ラベル管理リポジトリインターフェース
#[async_trait::async_trait]
pub trait LabelRepositoryTrait {
    /// ラベルを追加する
    ///
    /// # Arguments
    /// * `new` - 新規登録する発電状況
    /// # Returns
    /// * `Result<SubSystem, GenerationRepositoryError>` - 成功時は登録後のラベルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 記録に失敗した場合のエラー
    async fn add(&self, new: &LabelRecord) -> Result<String, GenerationRepositoryError>;

    /// ラベルを取得する
    ///
    /// # Returns
    /// * `Result<Vec<LabelRecord>, GenerationRepositoryError>` - 成功時はラベルのエンティティを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn get(&self) -> Result<Vec<LabelRecord>, GenerationRepositoryError>;

    /// ラベルが存在するか確認する
    ///
    /// # Arguments
    /// * `system` - 削除するラベル
    /// # Returns
    /// * `Result<bool, GenerationRepositoryError>` - 成功時は存在するかどうかを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn has(&self, label: &str) -> Result<bool, GenerationRepositoryError>;

    /// ラベルを削除する
    ///
    /// # Arguments
    /// * `system` - 削除するラベル
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 削除に失敗した場合のエラー
    async fn delete(&self, label: &str) -> Result<(), GenerationRepositoryError>;
}
