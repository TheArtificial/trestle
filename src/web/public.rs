// ROUTES: When modifying routes in this file, update /src/web/ROUTES.md
use axum::{
    Router,
    extract::State,
    http::{StatusCode, Uri},
    response::{Html, Response},
    routing::get,
};
use axum_extra::routing::RouterExt;

use crate::{
    auth_backend::Backend,
    errors::CustomError,
    web::{self, AppState},
};
use axum_login::AuthSession;

use minijinja::context;

pub fn router() -> Router<AppState> {
    Router::new()
        .route_with_tsr("/docs/design-system", get(docs_design_system))
        .fallback(get(serve_404))
}

async fn docs_design_system(
    State(state): State<AppState>,
    auth_session: AuthSession<Backend>,
) -> Result<Response, CustomError> {
    web::render_template(
        state.template_env,
        "docs/design-system.html",
        auth_session,
        context! {},
    )
    .await
}

pub async fn serve_404(State(state): State<AppState>, uri: Uri) -> (StatusCode, Html<String>) {
    let template = state.template_env.get_template("404.html").unwrap();
    let r = template
        .render(context! { error_code => 404, uri => format!("{}", uri) })
        .unwrap();
    (StatusCode::NOT_FOUND, Html(r))
}
