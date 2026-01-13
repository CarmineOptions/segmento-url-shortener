use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use common::{
    Link,
    validation::{is_valid_code, is_valid_url},
};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct CreateLinkRequest {
    target_url: String,
}

pub async fn health_check() -> Response {
    (StatusCode::OK, "API is healthy").into_response()
}

pub async fn create_link(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<CreateLinkRequest>,
) -> Result<Json<Link>, Response> {
    if !is_valid_url(&payload.target_url) {
        tracing::warn!("not a valid URL: {}", &payload.target_url);
        return Err((StatusCode::BAD_REQUEST).into_response());
    }

    match state.link_service.create_link(&payload.target_url) {
        Ok(inserted_link) => {
            tracing::info!("created link: {:?}", inserted_link);
            return Ok(Json(inserted_link));
        }
        Err(e) => {
            tracing::error!("failed creating link: {:?}", e);
            return Err(
                (StatusCode::INTERNAL_SERVER_ERROR, "failed to create link").into_response()
            );
        }
    }
}

pub async fn get_link(
    State(state): State<std::sync::Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<Json<Link>, Response> {
    if !is_valid_code(&code) {
        tracing::warn!("not a valid code: {}", code);
        return Err((StatusCode::BAD_REQUEST, "short code not valid").into_response());
    }
    match state.link_service.get_link(&code) {
        Ok(link_maybe) => match link_maybe {
            Some(link) => return Ok(Json(link)),
            None => {
                tracing::warn!("not a valid code: {}", code);
                return Err((StatusCode::NOT_FOUND, "short code does not exist").into_response());
            }
        },
        Err(e) => {
            tracing::warn!("failed getting code: {:?}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "failed getting code").into_response());
        }
    };
}

pub async fn code_redirect(
    State(state): State<std::sync::Arc<AppState>>,
    Path(code): Path<String>,
) -> Response {
    if !is_valid_code(&code) {
        tracing::warn!("not a valid code: {}", code);
        return (StatusCode::BAD_REQUEST, "short code not valid").into_response();
    }
    let link = match state.link_service.get_link(&code) {
        Ok(link_maybe) => match link_maybe {
            Some(link) => link,
            None => {
                tracing::warn!("code does not exist: {}", code);
                return (StatusCode::NOT_FOUND, "short code does not exist").into_response();
            }
        },
        Err(e) => {
            tracing::error!("failed getting code: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "failed getting code").into_response();
        }
    };

    Redirect::temporary(&link.target_url).into_response()
}
