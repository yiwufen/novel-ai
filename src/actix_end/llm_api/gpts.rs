use reqwest;

use crate::actix_end::llm_api::req_model::ChatResp;

use super::req_model::{Body, Header, Message};


pub async fn request_chat(
    system_content: Option<Message>,
    user_content: Message,
) -> Result<String, String> {
    let mut messages = vec![];
    if let Some(system_content) = system_content {
        messages.push(system_content);
    }
    // /g/g-2rmYtozlo-xiao-shuo-chuang-zuo-jia
    messages.push(user_content);
    let body = Body {
        model: "gpt-4-gizmo-g-2rmYtozlo".to_string(),
        messages: messages,
        ..Default::default()
    };
    let url = "https://api.aigcbest.top/v1/chat/completions";
    let mut header = Header::default();
    if let Ok(token) = std::env::var("API_KEY"){
        header.authorization = Some(format!("Bearer {}", token));
    }
    let client = reqwest::Client::new();
    let resp = client.post(url)
        .json(&body)
        .headers(header.to_reqwest_header_map())
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let chat_resp: ChatResp = resp.json().await.map_err(|e| e.to_string())?;
    let resp = &chat_resp.choices[0].message.as_ref().unwrap().content;
    println!("resp: {:?}", chat_resp.choices[0].message.as_ref().unwrap().content);
    Ok(resp.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_request_chat() {
        let system_content = Message {
            content: "Hello".to_string(),
            role: "system".to_string()
        };
        let user_content = Message {
            content: "Hi, say this is a test".to_string(),
            role: "user".to_string()
        };
        let result = request_chat(Some(system_content), user_content).await;
        assert_eq!(result, Ok("...".to_string()));
    }
}