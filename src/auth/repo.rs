use async_trait::async_trait;

#[async_trait]
trait AuthRepository {
    async fn auth(&self) -> bool;
}