use super::errors::GenerationError;
use layer_domain::entity::SubSystemEntity;

/// グループ（サブシステム）を記録するためのリポジトリインターフェース
#[async_trait::async_trait]
pub trait SubSystemRepositoryTrait<Tx> {
    /// グループ（サブシステム）を追加する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `new` - 新規登録する発電状況
    /// # Returns
    /// * `Result<SubSystem, GenerationRepositoryError>` - 成功時は登録後のグループ（サブシステム）を返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 記録に失敗した場合のエラー
    async fn add(&self, tx: &Tx, new: &SubSystemEntity) -> Result<String, GenerationError>;

    /// グループ（サブシステム）を取得する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// # Returns
    /// * `Result<Vec<GroupRecord>, GenerationRepositoryError>` - 成功時はグループ（サブシステム）のエンティティを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn get(&self, tx: &Tx) -> Result<Vec<SubSystemEntity>, GenerationError>;

    /// グループ（サブシステム）が存在するか確認する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `system` - 削除するグループ（サブシステム）
    /// # Returns
    /// * `Result<bool, GenerationRepositoryError>` - 成功時は存在するかどうかを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn has(&self, tx: &Tx, system: &String) -> Result<bool, GenerationError>;

    /// グループ（サブシステム）を削除する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `system` - 削除するグループ（サブシステム）
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 削除に失敗した場合のエラー
    async fn delete(&self, tx: &Tx, system: &String) -> Result<(), GenerationError>;
}
