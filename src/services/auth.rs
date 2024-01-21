pub mod auth_service {
    use crate::structs::auth::TokenClaims;
    use crate::structs::{auth::AuthServicePayload, user::UserFull};
    use crate::utils::connection::internarnal_error;
    use axum::http::{header, Response, StatusCode};
    use axum::response::IntoResponse;
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use bcrypt::{hash, verify, DEFAULT_COST};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde_json::json;

    async fn queue_user(
        args: &AuthServicePayload,
    ) -> Result<Option<UserFull>, (StatusCode, String)> {
        let raw = sqlx::query!("SELECT * FROM users WHERE email = $1", args.payload.email)
            .fetch_optional(&args.state.db)
            .await
            .map_err(internarnal_error)?;

        match raw {
            Some(res) => {
                let result = UserFull {
                    id: res.id,
                    email: res.email.unwrap(),
                    password_hash: res.password_hash.unwrap(),
                };

                Ok(Some(result))
            }
            None => Ok(None),
        }
    }

    pub async fn register(args: AuthServicePayload) -> Result<(), (StatusCode, String)> {
        let user = queue_user(&args).await?;

        match user {
            Some(_) => {
                return Err((
                    StatusCode::IM_A_TEAPOT,
                    "Такой пользователь уже существует".to_string(),
                ));
            }
            None => {
                let password_hash =
                    hash(args.payload.password, DEFAULT_COST).map_err(internarnal_error)?;

                let _ = sqlx::query!(
                    "INSERT INTO users (email, password_hash) VALUES ($1, $2)",
                    args.payload.email,
                    password_hash,
                )
                .execute(&args.state.db)
                .await;
            }
        }

        Ok(())
    }

    pub async fn login(
        args: AuthServicePayload,
    ) -> Result<impl IntoResponse, (StatusCode, String)> {
        let user = queue_user(&args).await?;

        println!("{:?}",user);

        match user {
            Some(user) => {
                let check = verify(args.payload.password, &user.password_hash)
                    .map_err(internarnal_error)?;

                match check {
                    true => {
                        let now = chrono::Utc::now();
                        let iat = now.timestamp() as usize;
                        let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
                        let claims: TokenClaims = TokenClaims {
                            sub: user.id.to_string(),
                            exp,
                            iat,
                        };

                        let token = encode(
                            &Header::default(),
                            &claims,
                            &EncodingKey::from_secret(args.state.env.jwt_secret.as_ref()),
                        )
                        .unwrap();

                        let cookie = Cookie::build(("token", token.to_owned()))
                            .path("/")
                            .max_age(time::Duration::hours(1))
                            .same_site(SameSite::None);

                        let mut response: Response<String> = Response::new(
                            json!({
                                "status": "success",
                                "token": token,
                            })
                            .to_string(),
                        );

                        response
                            .headers_mut()
                            .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

                        return Ok(response);
                    }
                    false => {
                        return Err((StatusCode::IM_A_TEAPOT, "Не правильный пароль".to_string()))
                    }
                }
            }
            None => {
                return Err((
                    StatusCode::IM_A_TEAPOT,
                    "Такого пользователя не существует".to_string(),
                ))
            }
        }
    }
}
