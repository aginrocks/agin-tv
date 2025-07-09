/// Get a movie
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        // (status = OK, description = "Success", body = Vec<Organization>),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Organizations"
)]
pub async fn get_movie(
    Path(movie_id): Path<String>,
    Extension(state): Extension<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let movie_id = mongo_id::parse_object_id(&movie_id)?;
    let movie = state
        .movies
        .find_one(doc! { "_id": movie_id }, None)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(movie))
}
