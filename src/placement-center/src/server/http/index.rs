use axum::extract::State;
use common_base::http_response::success_response;

use super::server::HttpServerState;

pub async fn index(State(_): State<HttpServerState>)->String{
    return success_response("{}");
}