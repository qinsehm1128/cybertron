use anyhow::Result;
use rmcp::{Error as McpError, model::*};

use crate::mcp::{ZhiRequest, PopupRequest};
use crate::mcp::handlers::{create_tauri_popup, parse_mcp_response};
use crate::mcp::utils::{generate_request_id, popup_error};

/// 🚛 擎天柱 - 领袖级交互核心
///
/// 负责与人类盟友建立通信链路，支持战术选项、自由指令输入和图像情报上传
/// 「自由是所有智慧生命的权利」
#[derive(Clone)]
pub struct InteractionTool;

impl InteractionTool {
    pub async fn zhi(
        request: ZhiRequest,
    ) -> Result<CallToolResult, McpError> {
        let popup_request = PopupRequest {
            id: generate_request_id(),
            message: request.message,
            predefined_options: if request.predefined_options.is_empty() {
                None
            } else {
                Some(request.predefined_options)
            },
            is_markdown: request.is_markdown,
        };

        match create_tauri_popup(&popup_request) {
            Ok(response) => {
                // 解析响应内容，支持文本和图像情报
                let content = parse_mcp_response(&response)?;
                Ok(CallToolResult::success(content))
            }
            Err(e) => {
                Err(popup_error(format!("擎天柱通信链路故障: {}", e)).into())
            }
        }
    }
}
