use super::errors::GenerationError;
use layer_domain::{entity::UnitEntity, value_object::Unit};

/// 単位を管理するためのリポジトリインターフェース
#[async_trait::async_trait]
pub trait UnitRepositoryTrait<Tx> {
    /// 単位を追加する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `e` - 新規登録する単位
    /// # Returns
    /// * `Result<Unit, GenerationRepositoryError>` - 成功時は登録後の単位を返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 記録に失敗した場合のエラー
    async fn add(&self, tx: &Tx, e: &UnitEntity) -> Result<Unit, GenerationError>;

    /// 単位を取得する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `unit` - 情報取得する対象の単位。指定なければ全て取得する。
    /// # Returns
    /// * `Result<Vec<UnitRecord>, GenerationRepositoryError>` - 成功時は単位のエンティティを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn get(&self, tx: &Tx, unit: Option<&Unit>) -> Result<Vec<UnitEntity>, GenerationError>;

    /// 単位を更新する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `e` - 更新する単位のエンティティ
    /// # Returns
    /// * `Result<UnitEntity, GenerationRepositoryError>` - 成功時は値を返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn update(&self, tx: &Tx, e: &UnitEntity) -> Result<UnitEntity, GenerationError>;

    /// 単位を削除する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `unit` - 削除する単位
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 削除に失敗した場合のエラー
    async fn delete(&self, tx: &Tx, unit: &Unit) -> Result<(), GenerationError>;
}
