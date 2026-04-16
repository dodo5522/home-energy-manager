use super::errors::GenerationError;
use layer_domain::entity::LabelEntity;

/// ラベル管理リポジトリインターフェース
#[async_trait::async_trait]
pub trait LabelRepositoryTrait<Tx> {
    /// ラベルを追加する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `e` - 新規登録するラベルエンティティ
    /// # Returns
    /// * `Result<String, GenerationRepositoryError>` - 成功時は登録後のラベルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 記録に失敗した場合のエラー
    async fn add(&self, tx: &Tx, e: &LabelEntity) -> Result<String, GenerationError>;

    /// ラベルを取得する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `label` - 取得するラベルの名前（オプション）
    /// # Returns
    /// * `Result<Vec<LabelRecord>, GenerationRepositoryError>` - 成功時はラベルのエンティティを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn get(
        &self,
        tx: &Tx,
        label: Option<impl AsRef<str> + Send>,
    ) -> Result<Vec<LabelEntity>, GenerationError>;

    /// ラベルを更新する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `entity` - 更新するラベルのエンティティ
    /// # Returns
    /// * `Result<LabelEntity, GenerationRepositoryError>` - 成功時は値を返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 取得に失敗した場合のエラー
    async fn update(&self, tx: &Tx, e: &LabelEntity) -> Result<LabelEntity, GenerationError>;

    /// ラベルを削除する
    ///
    /// # Arguments
    /// * `tx` - データベーストランザクション
    /// * `label` - 削除するラベルの名前
    /// # Returns
    /// * `Result<(), GenerationRepositoryError>` - 成功時は空のタプルを返し、失敗時はエラーを返す
    /// # Errors
    /// * `GenerationRepositoryError` - 削除に失敗した場合のエラー
    async fn delete(&self, tx: &Tx, label: impl AsRef<str> + Send) -> Result<(), GenerationError>;
}
