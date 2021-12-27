use crate::auth::json::Claims;
use crate::core::error::AppError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};

fn create_token(claims: Claims, secret_key: &str) -> String {
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    );
    token.unwrap()
}

fn decode_token(token: &str, secret_key: &str) -> Result<TokenData<Claims>, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    )
    .map_err(AppError::DecodeClaimsFailed)
}

mod v1 {
    use crate::auth::handler::create_token;
    use crate::auth::json::Claims;
    use crate::auth::repo::AuthRepository;
    use crate::core::config::Config;
    use crate::core::error::AppError;
    use crate::user::repo::UserRepository;
    use uuid::Uuid;

    async fn login(
        user_repo: &dyn UserRepository,
        auth_repo: &dyn AuthRepository,
        config: &Config,
        username: &str,
        password: &str,
    ) -> Result<String, AppError> {
        let simple_user = user_repo.check_user_is_exist(username, password).await;
        if let Ok(Some(simple_user)) = simple_user {
            let expired_at = chrono::Utc::now() + chrono::Duration::days(30);
            let claims = Claims::new(
                simple_user.id.to_string(),
                expired_at.clone().timestamp() as usize,
                simple_user.role as u8,
            );

            let token = create_token(claims, config.secret_key.as_str());

            return auth_repo
                .create(simple_user.id, token.as_str(), expired_at)
                .await
                .map(move |_| token);
        }

        Err(AppError::AuthorizeFailed)
    }

    async fn logout(auth_repo: &dyn AuthRepository, id: Uuid) -> Result<(), AppError> {
        auth_repo.expire(id).await
    }

    async fn renew(auth_repo: &dyn AuthRepository, id: Uuid) -> Result<(), AppError> {
        let expired_at = chrono::Utc::now() + chrono::Duration::days(30);
        auth_repo.renew(id, expired_at).await
    }

    #[cfg(test)]
    mod test {
        use crate::auth::handler::decode_token;
        use mockall::mock;

        use crate::auth::json::Token;
        use crate::auth::repo::MockAuthRepository;
        use crate::user::json::SimpleUser;
        use crate::user::repo::MockUserRepository;

        use super::*;

        #[test]
        fn it_can_log_in_when_user_is_exist() {
            let runtime = tokio::runtime::Runtime::new().unwrap();

            let mut user_mock_repo = MockUserRepository::new();
            let mut auth_mock_repo = MockAuthRepository::new();

            user_mock_repo
                .expect_check_user_is_exist()
                .returning(|_, _| {
                    Ok(Some(SimpleUser {
                        id: uuid::Uuid::new_v4(),
                        name: "boris".to_string(),
                        role: 0,
                        created_at: Some(chrono::Utc::now()),
                        updated_at: Some(chrono::Utc::now()),
                    }))
                });

            auth_mock_repo.expect_create().returning(|id, token, _| {
                Ok(Token {
                    user_id: id,
                    token: token.to_string(),
                    expired_at: chrono::Utc::now() + chrono::Duration::days(30 as i64),
                })
            });

            let config = Config::new();

            let res = runtime.block_on(login(
                &user_mock_repo,
                &auth_mock_repo,
                &config,
                "boris",
                "password",
            ));

            assert!(res.is_ok());
        }

        #[test]
        fn it_can_not_log_in_when_user_is_not_exist() {
            let runtime = tokio::runtime::Runtime::new().unwrap();

            let mut user_mock_repo = MockUserRepository::new();
            let mut auth_mock_repo = MockAuthRepository::new();

            user_mock_repo
                .expect_check_user_is_exist()
                .returning(|_, _| Ok(None));

            let config = Config::new();

            let res = runtime.block_on(login(
                &user_mock_repo,
                &auth_mock_repo,
                &config,
                "boris",
                "password",
            ));

            assert_eq!(res.unwrap_err(), AppError::AuthorizeFailed);
        }

        #[test]
        fn it_can_create_token() {
            let config = Config::new();
            let claims = Claims::new(
                uuid::Uuid::new_v4().to_string(),
                chrono::Utc::now().timestamp() as usize,
                0,
            );
            let token = create_token(claims, config.secret_key.as_str());

            assert!(!token.is_empty());
        }

        #[test]
        fn it_can_decode_token() {
            let config = Config::new();
            let claims = Claims::new(
                uuid::Uuid::new_v4().to_string(),
                chrono::Utc::now().timestamp() as usize,
                0,
            );
            let token = create_token(claims, config.secret_key.as_str());

            assert!(!token.is_empty());

            let claims = decode_token(token.as_str(), config.secret_key.as_str());

            assert!(claims.is_ok());
        }

        #[test]
        fn it_can_not_decode_token_when_token_is_expired() {
            let config = Config::new();
            let claims = Claims::new(
                uuid::Uuid::new_v4().to_string(),
                (chrono::Utc::now() - chrono::Duration::seconds(1)).timestamp() as usize,
                0,
            );
            let token = create_token(claims, config.secret_key.as_str());

            assert!(!token.is_empty());

            let claims = decode_token(token.as_str(), config.secret_key.as_str());

            assert!(claims.is_err());
        }
    }
}
