use std::sync::Arc;

use uuid::Uuid;
use warp::http::StatusCode;
use warp::Reply;

use crate::auth::handlers::{create_token, get_expired_seconds};
use crate::auth::json::claims::Claims;
use crate::auth::json::request::AuthRequest;
use crate::auth::repo::AuthRepository;
use crate::core::config::Config;
use crate::core::error::AppError;
use crate::user::repo::UserRepository;
use crate::{Environment, WebResult};

pub async fn login_handler(req: AuthRequest, env: Environment) -> WebResult<impl Reply> {
    login(
        env.user_repo,
        env.auth_repo,
        &env.config,
        req.username.as_str(),
        req.password.as_str(),
    )
    .await
    .map(|token| warp::reply::json(&token))
    .map_err(warp::reject::custom)
}

async fn login(
    user_repo: Arc<impl UserRepository>,
    auth_repo: Arc<impl AuthRepository>,
    config: &Config,
    username: &str,
    password: &str,
) -> Result<String, AppError> {
    let user_opt = user_repo.get_by_name(username).await;

    match user_opt {
        Ok(None) => Err(AppError::UserNotExist),
        Err(e) => Err(e),
        Ok(Some(user)) => {
            let verify = argon2::verify_encoded(user.password.as_str(), password.as_bytes())
                .ok()
                .unwrap_or(false);

            if !verify {
                return Err(AppError::AuthorizeFailed);
            }

            let claims = Claims::new(user.id.unwrap().to_string(), 0, user.role as u8);

            let token = create_token(claims, config.secret_key.as_str());

            auth_repo
                .create(user.id.unwrap(), token.as_str(), get_expired_seconds())
                .map_err(|_| AppError::TokenNotExist)
        }
    }
}

pub async fn logout_handler(claims: Claims, env: Environment) -> WebResult<impl Reply> {
    logout(
        env.auth_repo,
        uuid::Uuid::parse_str(claims.sub.as_str()).unwrap(),
    )
    .map(|_| warp::reply::with_status("", StatusCode::OK))
    .map_err(warp::reject::custom)
}

fn logout(auth_repo: Arc<impl AuthRepository>, id: Uuid) -> Result<(), AppError> {
    auth_repo.expire(id).map_err(|_| AppError::TokenNotExist)
}

pub async fn renew_handler(claims: Claims, env: Environment) -> WebResult<impl Reply> {
    renew(
        env.auth_repo,
        uuid::Uuid::parse_str(claims.sub.as_str()).unwrap(),
    )
    .map(|_| warp::reply::with_status("", StatusCode::OK))
    .map_err(warp::reject::custom)
}

fn renew(auth_repo: Arc<impl AuthRepository>, id: Uuid) -> Result<(), AppError> {
    auth_repo
        .renew(id, get_expired_seconds())
        .map_err(|_| AppError::TokenNotExist)
}

#[cfg(test)]
mod test {
    use crate::auth::handlers::create_token;
    use crate::auth::repo::MockAuthRepository;
    use crate::core::util::hash_password;
    use crate::user::json::user::User;
    use crate::user::repo::MockUserRepository;

    use super::*;

    #[test]
    fn it_can_login() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let config = Config::new();

        let mut user_mock_repo = MockUserRepository::new();
        let mut auth_mock_repo = MockAuthRepository::new();

        mock_returning(&mut user_mock_repo, &mut auth_mock_repo);

        let predict_token = runtime.block_on(login(
            Arc::new(user_mock_repo),
            Arc::new(auth_mock_repo),
            &config,
            "boris",
            "123",
        ));
        assert!(predict_token.is_ok());
    }

    #[test]
    fn it_cannot_login_because_password_is_wrong() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let config = Config::new();

        let mut user_mock_repo = MockUserRepository::new();
        let mut auth_mock_repo = MockAuthRepository::new();

        mock_returning(&mut user_mock_repo, &mut auth_mock_repo);

        let predict_token = runtime.block_on(login(
            Arc::new(user_mock_repo),
            Arc::new(auth_mock_repo),
            &config,
            "boris",
            "123456",
        ));
        assert!(predict_token.is_err());
        assert_eq!(predict_token.unwrap_err(), AppError::AuthorizeFailed)
    }

    #[test]
    fn it_cannot_login_because_user_not_found() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let config = Config::new();

        let mut user_mock_repo = MockUserRepository::new();
        let auth_mock_repo = MockAuthRepository::new();

        user_mock_repo.expect_get_by_name().returning(|_| Ok(None));

        let predict_token = runtime.block_on(login(
            Arc::new(user_mock_repo),
            Arc::new(auth_mock_repo),
            &config,
            "boris",
            "123456",
        ));
        assert!(predict_token.is_err());
        assert_eq!(predict_token.unwrap_err(), AppError::UserNotExist)
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

    fn mock_returning(user_repo: &mut MockUserRepository, auth_repo: &mut MockAuthRepository) {
        user_repo.expect_get_by_name().returning(|_| {
            let config = Config::new();
            let password = hash_password("123", &config).unwrap();
            Ok(Some(User {
                id: Some(uuid::Uuid::new_v4()),
                name: "boris".to_string(),
                password,
                role: 0,
                created_at: None,
                updated_at: None,
            }))
        });

        auth_repo
            .expect_create()
            .returning(|id, token, seconds| Ok(token.to_string()));
    }
}
