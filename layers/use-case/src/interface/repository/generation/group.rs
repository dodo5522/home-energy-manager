use super::errors::GenerationRepositoryError;
use layer_domain::{entity::GroupRecord, value_object::SubSystem};

/// グループ（サブシステム）を記録するためのリポジトリインターフェース
#[async_trait::async_trait]
pub trait GroupRepositoryTrait {
    /// グループ（サブシステム）を追加する
    ///
    /// # Arguments
    /// * `new` - 新規登録する発電状況
    /// # Returns
    /// * `Result<SubSystem, GenerationRepositoryError>` - 成功時は登録後のグループ（サブシステム）を返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 記録に失敗した場合のエラー
    async fn add(&self, new: &GroupRecord) -> Result<SubSystem, GenerationRepositoryError>;

    /// グループ（サブシステム）を取得する
    ///
    /// # Returns
    /// * `Result<Vec<GroupRecord>, GenerationRepositoryError>` - 成功時はグループ（サブシステム）のエンティティを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn get(&self) -> Result<Vec<GroupRecord>, GenerationRepositoryError>;

    /// グループ（サブシステム）が存在するか確認する
    ///
    /// # Arguments
    /// * `system` - 削除するグループ（サブシステム）
    /// # Returns
    /// * `Result<bool, GenerationRepositoryError>` - 成功時は存在するかどうかを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn has(&self, system: &SubSystem) -> Result<bool, GenerationRepositoryError>;

    /// グループ（サブシステム）を削除する
    ///
    /// # Arguments
    /// * `system` - 削除するグループ（サブシステム）
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 削除に失敗した場合のエラー
    async fn delete(&self, system: &SubSystem) -> Result<(), GenerationRepositoryError>;
}
