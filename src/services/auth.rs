use std::sync::Arc;

use anyhow::Result;

use crate::{
    auth::password,
    db::repositories::user_repo,
    dto::auth::RegisterRequest,
    errors::AppError,
    state::AppState,
};

use chrono::{Duration, Utc};

use crate::{
    auth::jwt,
    db::repositories::{
        session_repo,
    },
    dto::auth::{
        LoginRequest,
        TokenResponse,
    },
};

use uuid::Uuid;

pub async fn register(
    state: Arc<AppState>,
    req: RegisterRequest,
) -> Result<(), AppError> {
    let existing =
        user_repo::find_by_email(
            &state.db,
            &req.email,
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if existing.is_some() {
        return Err(
            AppError::Conflict(
                "email already exists"
                    .into(),
            ),
        );
    }

    let password_hash =
        password::hash_password(
            &req.password,
        )
        .map_err(|_| AppError::Internal)?;

    user_repo::create(
        &state.db,
        &req.email,
        &password_hash,
        &req.name,
    )
    .await
    .map_err(|_| AppError::Internal)?;

    Ok(())
}

pub async fn login(
    state: Arc<AppState>,
    req: LoginRequest,
) -> Result<TokenResponse, AppError> {
    let user =
        user_repo::find_by_email(
            &state.db,
            &req.email,
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(
            AppError::Unauthorized,
        )?;

    let valid =
        password::verify_password(
            &req.password,
            &user.password_hash,
        )
        .map_err(|_| AppError::Internal)?;

    if !valid {
        return Err(
            AppError::Unauthorized,
        );
    }

    let refresh =
        jwt::generate_refresh_token();

    let refresh_hash =
        jwt::hash_refresh_token(
            &refresh,
        )
        .map_err(|_| AppError::Internal)?;

    let expires =
        Utc::now()
            + Duration::days(30);

    let session =
        session_repo::create(
            &state.db,
            user.id,
            &refresh_hash,
            None,
            None,
            expires,
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let access =
        jwt::generate_access_token(
            user.id,
            session.id,
            &state.config.jwt_secret,
        )
        .map_err(|_| AppError::Internal)?;

    Ok(TokenResponse {
        access_token: access,
        refresh_token: refresh,
    })
}

pub async fn refresh(
    state: Arc<AppState>,
    session_id: Uuid,
    refresh_token: String,
) -> Result<TokenResponse, AppError> {
    let session =
        session_repo::find_by_id(
            &state.db,
            session_id,
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(
            AppError::Unauthorized,
        )?;

    let valid =
        jwt::verify_refresh_token(
            &refresh_token,
            &session.refresh_token_hash,
        )
        .map_err(|_| AppError::Internal)?;

    if !valid {
        return Err(
            AppError::Unauthorized,
        );
    }

    let new_refresh =
        jwt::generate_refresh_token();

    let new_hash =
        jwt::hash_refresh_token(
            &new_refresh,
        )
        .map_err(|_| AppError::Internal)?;

    let expires =
        Utc::now()
            + Duration::days(30);

    session_repo::update_refresh_token(
        &state.db,
        session.id,
        &new_hash,
        expires,
    )
    .await
    .map_err(|_| AppError::Internal)?;

    let access =
        jwt::generate_access_token(
            session.user_id,
            session.id,
            &state.config.jwt_secret,
        )
        .map_err(|_| AppError::Internal)?;

    Ok(TokenResponse {
        access_token: access,
        refresh_token: new_refresh,
    })
}

pub async fn logout(
    state: Arc<AppState>,
    session_id: Uuid,
) -> Result<(), AppError> {
    session_repo::delete(
        &state.db,
        session_id,
    )
    .await
    .map_err(|_| AppError::Internal)?;

    Ok(())
}