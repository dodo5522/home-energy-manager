use super::errors::GenerationError;
use layer_domain::{entity::UnitEntity, value_object::Unit};

/// 単位を管理するためのリポジトリインターフェース
#[async_trait::async_trait]
pub trait UnitRepositoryTrait {
    /// 単位を追加する
    ///
    /// # Arguments
    /// * `new` - 新規登録する発電状況
    /// # Returns
    /// * `Result<Unit, GenerationRepositoryError>` - 成功時は登録後の単位を返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 記録に失敗した場合のエラー
    async fn add(&self, new: &UnitEntity) -> Result<Unit, GenerationError>;

    /// 単位を取得する
    ///
    /// # Returns
    /// * `Result<Vec<UnitRecord>, GenerationRepositoryError>` - 成功時は単位のエンティティを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn get(&self) -> Result<Vec<UnitEntity>, GenerationError>;

    /// 単位が存在するか確認する
    ///
    /// # Arguments
    /// * `system` - 削除する単位
    /// # Returns
    /// * `Result<bool, GenerationRepositoryError>` - 成功時は存在するかどうかを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn has(&self, system: &Unit) -> Result<bool, GenerationError>;

    /// 単位を削除する
    ///
    /// # Arguments
    /// * `system` - 削除する単位
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 削除に失敗した場合のエラー
    async fn delete(&self, system: &Unit) -> Result<(), GenerationError>;
}
