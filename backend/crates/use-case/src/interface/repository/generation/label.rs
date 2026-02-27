use super::errors::GenerationError;
use layer_domain::entity::LabelEntity;

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
    async fn add(&self, e: &LabelEntity) -> Result<String, GenerationError>;

    /// ラベルを取得する
    ///
    /// # Arguments
    /// * `label` - 取得するラベルの名前（オプション）
    /// # Returns
    /// * `Result<Vec<LabelRecord>, GenerationRepositoryError>` - 成功時はラベルのエンティティを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn get(&self, label: Option<&str>) -> Result<Vec<LabelEntity>, GenerationError>;

    /// ラベルを更新する
    ///
    /// # Arguments
    /// * `entity` - 更新するラベルのエンティティ
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn update(&self, e: &LabelEntity) -> Result<LabelEntity, GenerationError>;

    /// ラベルを削除する
    ///
    /// # Arguments
    /// * `label` - 削除するラベルの名前
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 削除に失敗した場合のエラー
    async fn delete(&self, label: &str) -> Result<(), GenerationError>;
}
