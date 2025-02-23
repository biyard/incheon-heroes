use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::QueryResponse;
use dto::*;

#[derive(Clone, Debug)]
pub struct ContentController {
    #[allow(dead_code)]
    repo: ContentRepository,
}

impl ContentController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Content::get_repository(pool);

        Self { repo }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_content), // .post(Self::act_content_by_id)
            )
            .with_state(self.clone())
            .route("/", post(Self::act_content).get(Self::list_content))
            .with_state(self.clone()))
    }

    pub async fn act_content(
        State(_ctrl): State<ContentController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<ContentAction>,
    ) -> Result<Json<Content>> {
        tracing::debug!("act_content {:?}", body);
        Ok(Json(Content::default()))
    }

    // pub async fn act_content_by_id(
    //     State(_ctrl): State<ContentController>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(id): Path<i64>,
    //     Json(body): Json<ContentByIdAction>,
    // ) -> Result<Json<Content>> {
    //     tracing::debug!("act_content_by_id {:?} {:?}", id, body);
    //     Ok(Json(Content::default()))
    // }

    pub async fn get_content(
        State(_ctrl): State<ContentController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<i64>,
    ) -> Result<Json<Content>> {
        tracing::debug!("get_content {:?}", id);
        Ok(Json(Content::default()))
    }

    pub async fn list_content(
        State(_ctrl): State<ContentController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<ContentParam>,
    ) -> Result<Json<ContentGetResponse>> {
        tracing::debug!("list_content {:?}", q);

        Ok(Json(ContentGetResponse::Query(QueryResponse::default())))
    }
}
