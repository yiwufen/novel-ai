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

use crate::app_models::novel::{ChapterEdit};


#[cfg(feature="ssr")]
use super::llm_api::{gpts::request_chat, req_model::Message};

#[server(CWriteAgent, "/api")]
pub async fn c_write_agent_api(chapter_edit: ChapterEdit) -> Result<String, ServerFnError> {
    let system_content = Message {
        content: "你是一位世界级的小说家。".to_string(),
        role: "system".to_string(),
    };
    let user_content = Message {
        content: format!(
            "故事背景：{} 

            目前情节： {}

            续写起点： {}

            关键角色：{}

            情节发展：{}

            风格和语气：{}

            字数：约1000字。",
            chapter_edit.bg_novel,chapter_edit.current_plot,chapter_edit.chapter_start,chapter_edit.key_role,chapter_edit.plot_development,chapter_edit.writing_style
        ),
        role: "user".to_string(),
    };
    let result = request_chat(Some(system_content), user_content)
        .await
        .map_err(|e| -> ServerFnError { log!("续写章节：{}",e.to_string()); ServerFnError::ServerError(e.to_string()) })?;
        Ok(result)
}

#[server(GetNovelEdit, "/api")]
pub async fn get_chapters_api(novel_name: String) -> Result<Vec<ChapterEdit>, ServerFnError> {
    log!("get_chapters_edit_api");
    let db_client = extract::<web::Data<Database>>().await?.into_inner();
    let collection = db_client.collection::<ChapterEdit>("chapters");
    let mut res = collection
        .find(doc! {"novel_name": novel_name}, None)
        .await
        .map_err(|err| -> ServerFnError { ServerFnError::ServerError(err.to_string()) })?;
    let mut chapters = vec![];
    while let Ok(Some(doc)) = res.try_next().await {
        chapters.push(doc.clone());
    }
    Ok(chapters)
}

#[server(GetChapterEdit, "/api")]
pub async fn get_chapter_edit_api(
    novel_name: String,
    chapter_title: String,
) -> Result<ChapterEdit, ServerFnError> {
    log!(
        "get chapter name: {:?}  chapter title: {:?}",
        novel_name,
        chapter_title
    );
    let db_client = extract::<web::Data<Database>>().await?.into_inner();
    let collection = db_client.collection::<ChapterEdit>("chapters");
    let res = collection
        .find_one(
            doc! {"novel_name": &novel_name, "chapter_title": chapter_title},
            None,
        )
        .await
        .map_err(|err| -> ServerFnError { ServerFnError::ServerError(err.to_string()) })?;
    // log!("查询对应章节: {:?}", res);
    match res {
        Some(doc) => Ok(doc),
        None => Ok(ChapterEdit {
            novel_name,
            ..Default::default()
        }),
    }
}
#[server(UpsertChapterEdit, "/api")]
pub async fn upsert_chapter_edit_api(chapter_edit: ChapterEdit) -> Result<String, ServerFnError> {
    // log!("upsert_novel_edit_api{:?}", novel_edit);
    let db_client = extract::<web::Data<Database>>().await?.into_inner();
    let collection = db_client.collection::<ChapterEdit>("chapters");
    let filter = doc! {
        "novel_name": &chapter_edit.novel_name,
        "chapter_title": &chapter_edit.chapter_title
    };
    let update = doc! {
        "$set": bson::to_bson(&chapter_edit).unwrap()
    };
    let options = mongodb::options::UpdateOptions::builder().upsert(true).build();
    let res = collection.update_one(filter, update, options).await;
    log!("更新插入：{:?}", res);
    match res {
        Ok(_res) => Ok("upsert success".to_string()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(DeleteChapterEdit, "/api")]
pub async fn delete_chapter_edit_api(
    novel_name: String,
    chapter_title: String,
) -> Result<String, ServerFnError> {
    let db_client = extract::<web::Data<Database>>().await?.into_inner();
    let collection = db_client.collection::<ChapterEdit>("chapters");
    let res = collection
        .delete_one(doc! {"novel_name": novel_name, "chapter_title": chapter_title}, None)
        .await
        .map_err(|err| -> ServerFnError { ServerFnError::ServerError(err.to_string()) })?;
    log!("删除章节：{:?}", res);
    Ok("delete success".to_string())
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use futures::TryStreamExt;
    use mongodb::{
        bson::{self, doc, oid::ObjectId, Bson, Document}, options::UpdateOptions, Client
    };

    use crate::{
        actix_end::chapter_api::get_chapters_api,
        app_models::novel::{ChapterEdit, Novel},
    };

    #[actix_web::test]
    async fn t_get_novel_edit_api() {
        dotenv::dotenv().ok();
        let mongodb_uri =
            std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

        let mongo_client = Client::with_uri_str(mongodb_uri)
            .await
            .expect("failed to connect");
        let db = mongo_client.database("novel");
        let collection = db.collection::<ChapterEdit>("chapters");
        let mut res = collection
            .find(doc! {"novel_name": "云边有个小卖部"}, None)
            .await
            .expect("failed to find");
        let mut result = vec![];
        while let Ok(Some(doc)) = res.try_next().await {
            result.push(doc.clone());
        }
        println!("{:#?}", result);
    }
    #[actix_web::test]
    async fn t_get_chapter_edit_api() {
        dotenv::dotenv().ok();
        let mongodb_uri =
            std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

        let mongo_client = Client::with_uri_str(mongodb_uri)
            .await
            .expect("failed to connect");
        let db = mongo_client.database("novel");
        let collection = db.collection::<ChapterEdit>("chapters");
        let res = collection
            .find_one(
                doc! {"novel_name": "云边有个小卖部", "chapter_title": "第一章"},
                None,
            )
            .await
            .expect("failed to find");
        println!("{:#?}", res);
    }
    #[actix_web::test]
    async fn t_upsert_chapter_edit_api() {
        dotenv::dotenv().ok();
        let mongodb_uri =
            std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

        let mongo_client = Client::with_uri_str(mongodb_uri)
            .await
            .expect("failed to connect");
        let db = mongo_client.database("novel");
        let chapter_edit = ChapterEdit {
            novel_name: "云边有个小卖部".to_string(),
            chapter_title: "第三章".to_string(),
            bg_novel: "背景".to_string(),
            current_plot: "当前情节".to_string(),
            plot_development: "情节发展".to_string(),
            key_role: "关键角色".to_string(),
            writing_style: "写作风格".to_string(),
            chapter_start: "章节开头".to_string(),
            content: "".to_string()
        };
        let filter = doc! {
            "novel_name": &chapter_edit.novel_name,
            "chapter_title": &chapter_edit.chapter_title
        };
    
        let update = doc! {
            "$set": bson::to_bson(&chapter_edit).unwrap()
        };
    
        let options = UpdateOptions::builder().upsert(true).build();
        let collection = db.collection::<ChapterEdit>("chapters");
        let res = collection.update_one(filter, update, options).await;
        println!("{:#?}", res);
        assert_eq!(res.is_ok(), true);
    }
}
