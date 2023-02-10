use axum::{
    extract::{Extension, Json as JsonForm, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sea_orm::{prelude::*, DatabaseConnection, Set};

use crate::my_error::CustomError;
use entity::{
    family::{self, Entity as Family},
    post::Entity as Post,
};

pub async fn get_families(
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, CustomError> {
    let families = Family::find().find_with_related(Post).all(conn).await?;

    Ok((StatusCode::OK, Json(families)))
}

pub async fn get_family(
    Extension(ref conn): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, CustomError> {
    let family = Family::find_by_id(id)
        .find_with_related(Post)
        .one(conn)
        .await?
        .ok_or(CustomError::NoneError(id))?;

    Ok((StatusCode::OK, Json(family)))
}

pub async fn create_family(
    form: JsonForm<family::Model>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, CustomError> {
    let model = form.0;

    let family = family::ActiveModel {
        name: Set(model.name),
        age: Set(model.age),
        ..Default::default()
    };

    Family::insert(family).exec(conn).await?;

    Ok((
        StatusCode::OK,
        Json("ユーザーの作成に成功しました".to_owned()),
    ))
}

pub async fn update_family(
    form: JsonForm<family::UpdateModal>,
    Path(id): Path<i32>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, CustomError> {
    let model = form.0;

    let family = Family::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(CustomError::NoneError(id))?;

    let updated_family = family::ActiveModel {
        id: Set(id),
        name: Set(model.name.unwrap_or(family.name)),
        age: Set(model.age.unwrap_or(family.age)),
    };

    Family::update(updated_family).exec(conn).await?;

    Ok((
        StatusCode::OK,
        Json("ユーザーの更新に成功しました".to_owned()),
    ))
}

pub async fn delete_family(
    Path(id): Path<i32>,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, CustomError> {
    let family = Family::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(CustomError::NoneError(id))?;

    family.delete(conn).await?;

    Ok((StatusCode::OK, Json("ユーザーの削除に成功しました")))
}
