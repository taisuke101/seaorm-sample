use axum::{
    extract::{Extension, Json as JsonForm, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sea_orm::{prelude::*, DatabaseConnection, Set};

use crate::my_error::CustomError;
use entity::post::{self, Entity as Post, Model, UpdateModal};

pub async fn get_posts(
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, CustomError> {
    let posts = Post::find().all(conn).await?;

    Ok((StatusCode::OK, Json(posts)))
}

pub async fn get_post(
    Extension(ref conn): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, CustomError> {
    let post = Post::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(CustomError::NoneError(id))?;

    Ok((StatusCode::OK, Json(post)))
}

pub async fn create_post(
    form: JsonForm<Model>,
    Extension(ref conn): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, CustomError> {
    let model = form.0;

    let post = post::ActiveModel {
        title: Set(model.title),
        text: Set(model.text),
        family_id: Set(id),
        ..Default::default()
    };

    Post::insert(post).exec(conn).await?;

    Ok((StatusCode::OK, Json("記事の作成に成功しました".to_owned())))
}

pub async fn update_post(
    form: JsonForm<UpdateModal>,
    Path(id): Path<i32>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, CustomError> {
    let model = form.0;

    let post = Post::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(CustomError::NoneError(id))?;

    let updated_post = post::ActiveModel {
        id: Set(id),
        title: Set(model.title.unwrap_or(post.title)),
        text: Set(model.text.unwrap_or(post.text)),
        family_id: Set(id),
    };

    Post::update(updated_post).exec(conn).await?;

    Ok((StatusCode::OK, Json("記事の更新に成功しました".to_owned())))
}

pub async fn delete_post(
    Path(id): Path<i32>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, CustomError> {
    let post = Post::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(CustomError::NoneError(id))?;

    post.delete(conn).await?;

    Ok((StatusCode::OK, Json("記事の削除に成功しました")))
}
