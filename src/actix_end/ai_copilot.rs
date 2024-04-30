use leptos::*;
#[cfg(feature="ssr")]
use super::llm_api::{gpts::request_chat, req_model::Message};

#[server(CWrite, "/api")]
pub async fn c_write_api(text: String)  -> Result<String, ServerFnError> {
    let system_content = Message {
        content: "你是一位世界级的小说家。".to_string(),
        role: "system".to_string()
    };
    let user_content = Message {
        content: format!("续写这个小说片段：{} 输出的语言必须与小说文本片段保持一致", text),
        role: "user".to_string()
    };
    let result = request_chat(Some(system_content), user_content).await
        .map_err(|e| -> ServerFnError {ServerFnError::ServerError(e.to_string())})?;
    Ok(result)
}

#[server(ReWrite, "/api")]
pub async fn re_write_api(text: String)  -> Result<String, ServerFnError> {
    let system_content = Message {
        content: "你是一位世界级的小说家。".to_string(),
        role: "system".to_string()
    };
    let user_content = Message {
        content: format!("重写这个小说片段：{} 输出的语言必须与小说文本片段保持一致", text),
        role: "user".to_string()
    };
    let result = request_chat(Some(system_content), user_content).await
        .map_err(|e| -> ServerFnError {ServerFnError::ServerError(e.to_string())})?;
    Ok(result)
}
