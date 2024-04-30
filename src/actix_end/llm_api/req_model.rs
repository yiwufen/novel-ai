use reqwest::header::{self, HeaderMap};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub accept: String,

    pub authorization: Option<String>,

    pub content_type: String,
}

impl Header {
    /// Convert the custom Header struct into a HeaderMap that reqwest can use
    pub fn to_reqwest_header_map(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT, self.accept.parse().unwrap());
        if let Some(auth) = &self.authorization {
            headers.insert(header::AUTHORIZATION, auth.parse().unwrap());
        }
        headers.insert(header::CONTENT_TYPE, self.content_type.parse().unwrap());
        headers
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            accept: "application/json".to_string(),
            authorization: None,
            content_type: "application/json".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Body {
    /// -2.0 和 2.0 之间的数字。正值会根据新标记在文本中的现有频率对其进行惩罚，从而降低模型逐字重复同一行的可能性。
    /// [查看有关频率和存在惩罚的更多信息。](https://platform.openai.com/docs/api-reference/parameter-details)
    pub frequency_penalty: Option<f64>,

    /// 修改指定标记出现在完成中的可能性。  接受一个 json 对象，该对象将标记（由标记器中的标记 ID 指定）映射到从 -100 到 100
    /// 的关联偏差值。从数学上讲，偏差会在采样之前添加到模型生成的 logits 中。确切的效果因模型而异，但 -1 和 1 之间的值应该会减少或增加选择的可能性；像 -100 或
    /// 100 这样的值应该导致相关令牌的禁止或独占选择。
    pub logit_bias: Option<serde_json::Value>,

    /// 聊天完成时生成的最大令牌数。  输入标记和生成标记的总长度受模型上下文长度的限制。
    pub max_tokens: Option<i64>,

    /// 以[聊天格式](https://platform.openai.com/docs/guides/chat/introduction)生成聊天完成的消息。
    pub messages: Vec<Message>,

    /// 要使用的模型的 ID。有关哪些模型适用于聊天 API
    /// 的详细信息，请参阅[模型端点兼容性表。](https://platform.openai.com/docs/models/model-endpoint-compatibility)
    /// claude-3-opus-20240229；gpt-4-gizmo-g-2rmYtozlo
    pub model: String,

    /// 为每个输入消息生成多少个聊天完成选项。
    pub n: Option<i64>,

    /// -2.0 和 2.0 之间的数字。正值会根据到目前为止是否出现在文本中来惩罚新标记，从而增加模型谈论新主题的可能性。
    /// [查看有关频率和存在惩罚的更多信息。](https://platform.openai.com/docs/api-reference/parameter-details)
    pub presence_penalty: Option<f64>,

    /// API 将停止生成更多令牌的最多 4 个序列。
    pub stop: Option<String>,

    /// 如果设置，将发送部分消息增量，就像在 ChatGPT
    /// 中一样。当令牌可用时，令牌将作为纯数据[服务器发送事件](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format)`data:
    /// [DONE]`发送，流由消息终止。[有关示例代码](https://github.com/openai/openai-cookbook/blob/main/examples/How_to_stream_completions.ipynb)，请参阅
    /// OpenAI Cookbook 。
    pub stream: Option<bool>,

    /// 使用什么采样温度，介于 0 和 2 之间。较高的值（如 0.8）将使输出更加随机，而较低的值（如 0.2）将使输出更加集中和确定。
    /// 我们通常建议改变这个或`top_p`但不是两者。
    pub temperature: Option<f32>,

    /// 一种替代温度采样的方法，称为核采样，其中模型考虑具有 top_p 概率质量的标记的结果。所以 0.1 意味着只考虑构成前 10% 概率质量的标记。
    /// 我们通常建议改变这个或`temperature`但不是两者。
    pub top_p: Option<f32>,

    /// 代表您的最终用户的唯一标识符，可以帮助 OpenAI
    /// 监控和检测滥用行为。[了解更多](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids)。
    pub user: Option<String>,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![],
            temperature: Some(0.5),
            max_tokens: None,
            n: Some(1),
            stream: Some(false),
            presence_penalty: Some(0f64),
            frequency_penalty: Some(0f64),
            logit_bias: None,
            top_p: None,
            user: None,
            stop: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatResp {
    pub choices: Vec<Choice>,

    pub created: i64,

    pub id: String,

    pub object: String,

    pub usage: Usage,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub finish_reason: Option<String>,

    pub index: Option<i64>,

    pub message: Option<Message>,
}


#[derive(Serialize, Deserialize)]
pub struct Usage {
    pub completion_tokens: i64,

    pub prompt_tokens: i64,

    pub total_tokens: i64,
}
