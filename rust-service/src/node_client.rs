use std::{sync::Arc, time::Duration};

use reqwest::{
    header::{HeaderMap, SET_COOKIE},
    Client, StatusCode,
};
use serde_json::json;
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::{
    config::Config,
    error::{AppError, AppResult},
    models::StudentReportData,
};

#[derive(Clone, Debug)]
struct Session {
    access_token: String,
    refresh_token: String,
    csrf_token: String,
}

#[derive(Clone)]
pub struct NodeApiClient {
    http: Client,
    base_url: String,
    username: String,
    password: String,
    session: Arc<Mutex<Option<Session>>>,
}

impl NodeApiClient {
    pub fn new(config: &Config) -> AppResult<Self> {
        let http = Client::builder().timeout(Duration::from_secs(10)).build()?;

        Ok(Self {
            http,
            base_url: config.node_api_base_url.clone(),
            username: config.node_api_username.clone(),
            password: config.node_api_password.clone(),
            session: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn get_student(&self, student_id: i64) -> AppResult<StudentReportData> {
        let response = self.fetch_student(student_id, false).await?;

        if response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::FORBIDDEN
            || response.status() == StatusCode::BAD_REQUEST
        {
            warn!(
                status = %response.status(),
                "student fetch unauthorized/forbidden; re-authenticating"
            );
            let response = self.fetch_student(student_id, true).await?;
            return self.parse_student_response(response).await;
        }

        self.parse_student_response(response).await
    }

    async fn fetch_student(
        &self,
        student_id: i64,
        force_login: bool,
    ) -> AppResult<reqwest::Response> {
        let session = self.ensure_session(force_login).await?;
        let url = format!("{}/api/v1/students/{}", self.base_url, student_id);
        info!(%url, "fetching student from Node API");

        let cookie = format!(
            "accessToken={}; refreshToken={}; csrfToken={}",
            session.access_token, session.refresh_token, session.csrf_token
        );

        self.http
            .get(&url)
            .header("Cookie", cookie)
            .header("x-csrf-token", &session.csrf_token)
            .send()
            .await
            .map_err(AppError::from)
    }

    async fn parse_student_response(
        &self,
        response: reqwest::Response,
    ) -> AppResult<StudentReportData> {
        let status = response.status();

        if status == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound);
        }

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::Upstream(format!(
                "Node API returned {status}: {body}"
            )));
        }

        response
            .json::<StudentReportData>()
            .await
            .map_err(|err| AppError::Upstream(format!("invalid student JSON: {err}")))
    }

    /// Holds the lock across the login so concurrent cold requests share a
    /// single round trip instead of each authenticating separately.
    async fn ensure_session(&self, force_login: bool) -> AppResult<Session> {
        let mut guard = self.session.lock().await;
        if !force_login {
            if let Some(session) = guard.as_ref() {
                return Ok(session.clone());
            }
        }

        let session = self.login().await?;
        *guard = Some(session.clone());
        Ok(session)
    }

    async fn login(&self) -> AppResult<Session> {
        let url = format!("{}/api/v1/auth/login", self.base_url);
        info!(%url, username = %self.username, "logging into Node API");

        let response = self
            .http
            .post(&url)
            .json(&json!({
                "username": self.username,
                "password": self.password,
            }))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::Auth(format!(
                "login failed with {status}: {body}"
            )));
        }

        // Cookies are marked Secure; reqwest cookie jar may skip them on http://
        // so we parse Set-Cookie headers manually.
        let headers = response.headers().clone();
        let access_token = cookie_value(&headers, "accessToken")
            .ok_or_else(|| AppError::Auth("login response missing accessToken cookie".into()))?;
        let refresh_token = cookie_value(&headers, "refreshToken")
            .ok_or_else(|| AppError::Auth("login response missing refreshToken cookie".into()))?;
        let csrf_token = cookie_value(&headers, "csrfToken")
            .ok_or_else(|| AppError::Auth("login response missing csrfToken cookie".into()))?;

        // Consume body so the connection can be reused.
        let _ = response.bytes().await;

        Ok(Session {
            access_token,
            refresh_token,
            csrf_token,
        })
    }
}

fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    let mut found = None;

    for value in headers.get_all(SET_COOKIE) {
        let Ok(raw) = value.to_str() else {
            continue;
        };
        let Some(pair) = raw.split(';').next() else {
            continue;
        };
        let Some((key, val)) = pair.split_once('=') else {
            continue;
        };
        if key.trim() == name {
            // Login clears cookies then sets them; keep the last non-empty value.
            if !val.is_empty() {
                found = Some(val.to_string());
            }
        }
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    #[test]
    fn cookie_value_keeps_the_last_non_empty_value() {
        // Login clears the cookies before setting them, so both arrive in one response.
        let mut headers = HeaderMap::new();
        headers.append(SET_COOKIE, HeaderValue::from_static("accessToken=; Path=/"));
        headers.append(
            SET_COOKIE,
            HeaderValue::from_static("accessToken=jwt; Path=/; HttpOnly; Secure"),
        );
        headers.append(SET_COOKIE, HeaderValue::from_static("csrfToken=csrf"));

        assert_eq!(
            cookie_value(&headers, "accessToken").as_deref(),
            Some("jwt")
        );
        assert_eq!(cookie_value(&headers, "csrfToken").as_deref(), Some("csrf"));
        assert_eq!(cookie_value(&headers, "refreshToken"), None);
    }
}
