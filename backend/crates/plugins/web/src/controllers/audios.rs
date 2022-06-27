use axum::routing::get;
use axum::Extension;
use axum::Router;
use axum::{extract::Path, response::Html};
use domain::logic::file::find_all_audios::FindAllAudiosAction;
use domain::{logic::file::find_by_hash::FindByHashAction, persistence::repos::file::File};
use persistence::Database;
use serde::Serialize;
use tera::Context;

pub fn setup() -> Router {
    Router::new()
        .route("/:hash", get(player))
        .route("/", get(list))
}

#[derive(Debug, Serialize)]
struct TemplateContext {
    audios: Vec<File>,
}

async fn list(Extension(action): Extension<FindAllAudiosAction<Database>>) -> Html<String> {
    let audios = action.execute().await.unwrap();
    let context = Context::from_serialize(TemplateContext { audios }).unwrap();
    return crate::templates::parse("views/audios/list.html", &context);
}

#[derive(Debug, Serialize)]
struct PlayerTemplateContext {
    audio: File,
}

async fn player(
    Extension(action): Extension<FindByHashAction<Database>>,
    Path(hash): Path<u64>,
) -> Html<String> {
    let audio = action.execute(hash).await.unwrap().unwrap();
    let context = Context::from_serialize(PlayerTemplateContext { audio }).unwrap();
    return crate::templates::parse("views/audios/player.html", &context);
}
