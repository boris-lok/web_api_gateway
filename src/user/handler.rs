pub mod v1 {
    use std::sync::Arc;

    use warp::Reply;

    use crate::core::error::AppError;
    use crate::core::util::hash_password;
    use crate::user::json::{CreateUserRequest, SimpleUser};
    use crate::user::repo::UserRepository;
    use crate::{Config, Environment, WebResult};

    pub async fn create_user_handler(
        req: CreateUserRequest,
        env: Environment,
    ) -> WebResult<impl Reply> {
        create_user(req, env.user_repo, &env.config)
            .await
            .map(|simple_user| warp::reply::json(&simple_user))
            .map_err(warp::reject::custom)
    }

    async fn create_user(
        req: CreateUserRequest,
        user_repo: Arc<impl UserRepository>,
        config: &Config,
    ) -> Result<SimpleUser, AppError> {
        let encrypt_password = hash_password(req.password.as_str(), config);

        if encrypt_password.is_none() {
            return Err(AppError::HashPasswordFailed);
        }

        user_repo
            .create(
                req.name.as_str(),
                encrypt_password.unwrap().as_str(),
                req.role,
            )
            .await
    }

    #[cfg(test)]
    mod test {
        use crate::user::repo::MockUserRepository;
        use crate::Config;

        use super::*;

        #[test]
        fn it_can_create_user() {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            let req = CreateUserRequest {
                name: "boris".to_string(),
                password: "123".to_string(),
                role: 0,
            };
            let mut user_mock_repo = MockUserRepository::new();
            let config = Config::new();

            user_mock_repo.expect_create().returning(|name, _, role| {
                Ok(SimpleUser {
                    id: uuid::Uuid::new_v4(),
                    name: name.to_string(),
                    role,
                    created_at: Some(chrono::Utc::now()),
                    updated_at: Some(chrono::Utc::now()),
                })
            });

            let response = runtime.block_on(create_user(req, Arc::new(user_mock_repo), &config));

            assert!(response.is_ok());

            let created_user = response.unwrap();
            assert_eq!(created_user.name, "boris");
            assert_eq!(created_user.role, 0);
        }
    }
}
