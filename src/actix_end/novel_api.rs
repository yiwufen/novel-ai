use leptos::{logging::log, *};

#[cfg(feature = "ssr")]
use actix_web::web;
#[cfg(feature = "ssr")]
use futures::stream::TryStreamExt;
#[cfg(feature = "ssr")]
use leptos_actix::extract;
#[cfg(feature = "ssr")]
use mongodb::bson;
#[cfg(feature = "ssr")]
use mongodb::bson::doc;

#[cfg(feature = "ssr")]
use mongodb::Database;

use crate::app_models::novel::Novel;

#[server(GetNovel)]
pub async fn get_novel(novel_name: String) -> Result<Novel, ServerFnError> {
    let db_client = extract::<web::Data<Database>>().await?.into_inner();
    let collection = db_client.collection::<Novel>("novels");
    let res = collection.find_one( doc! { "novel_name": &novel_name}, None)
        .await
        .map_err(|err| -> ServerFnError { ServerFnError::ServerError(err.to_string()) })?;

    match res {
        Some(novel) => Ok(novel),
        None => Ok(
            Novel {
                novel_name,
                ..Default::default()
            }
        )
    }
}

#[server(UpsertNovel)]
pub async fn upsert_novel(novel: Novel) -> Result<String, ServerFnError> {
    let db_client = extract::<web::Data<Database>>().await?.into_inner();
    let collection = db_client.collection::<Novel>("novels");
    let filter = doc! {
        "novel_name": &novel.novel_name,
    };
    let update = doc! {
        "$set": bson::to_bson(&novel).unwrap()
    };
    let options = mongodb::options::UpdateOptions::builder().upsert(true).build();
    let res = collection.update_one(filter, update, options).await;
    log!("更新插入：{:?}", res);
    match res {
        Ok(_res) => Ok("upsert success".to_string()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(DeleteNovel)]
pub async fn delete_novel(novel_name: String) -> Result<String, ServerFnError> {
    let db_client = extract::<web::Data<Database>>().await?.into_inner();
    let collection = db_client.collection::<Novel>("novels");
    let res = collection.delete_one( doc! {"novel_name": novel_name }, None).await;
    log!("删除小说：{:?}", res);
    match res {
        Ok(_res) => Ok("delete success".to_string()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(GetNovelList)]
pub async fn get_novel_list() -> Result<Vec<Novel>, ServerFnError> {
    let db_client = extract::<web::Data<Database>>().await?.into_inner();
    let collection = db_client.collection::<Novel>("novels");
    let mut cursor = collection.find(None, None).await?;
    let mut novels = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        novels.push(result);
    }
    Ok(novels)
}