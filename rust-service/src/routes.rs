use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tracing::info;

use crate::{error::AppResult, node_client::NodeApiClient, pdf};

#[derive(Clone)]
pub struct AppState {
    pub node: NodeApiClient,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/v1/students/{id}/report", get(student_report))
        .with_state(state)
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

/// GET /api/v1/students/:id/report
///
/// The id is typed, so a non-numeric segment is rejected by axum with a 400
/// before any upstream call, and the filename below is always header-safe.
async fn student_report(State(state): State<AppState>, Path(id): Path<i64>) -> AppResult<Response> {
    info!(student_id = id, "student report requested");

    let student = state.node.get_student(id).await?;
    let pdf_bytes = pdf::generate_student_report(&student)?;

    Ok((
        [
            (header::CONTENT_TYPE, "application/pdf".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"student-{id}-report.pdf\""),
            ),
        ],
        pdf_bytes,
    )
        .into_response())
}
