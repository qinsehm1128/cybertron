use anyhow::Result;
use rmcp::{
    Error as McpError, ServerHandler, ServiceExt, RoleServer,
    model::*,
    transport::stdio,
    service::RequestContext,
};
use std::collections::HashMap;

use super::tools::{InteractionTool, MemoryTool, AcemcpTool};
use super::types::{ZhiRequest, JiyiRequest};
use crate::config::load_standalone_config;
use crate::constants::themes::get_theme;
use crate::{log_important, log_debug};

#[derive(Clone)]
pub struct ZhiServer {
    enabled_tools: HashMap<String, bool>,
}

impl Default for ZhiServer {
    fn default() -> Self {
        Self::new()
    }
}

impl ZhiServer {
    pub fn new() -> Self {
        // 尝试加载配置，如果失败则使用默认配置
        let enabled_tools = match load_standalone_config() {
            Ok(config) => config.mcp_config.tools,
            Err(e) => {
                log_important!(warn, "无法加载配置文件，使用默认工具配置: {}", e);
                crate::config::default_mcp_tools()
            }
        };

        Self { enabled_tools }
    }

    /// 检查工具是否启用 - 动态读取最新配置
    fn is_tool_enabled(&self, tool_name: &str) -> bool {
        // 每次都重新读取配置，确保获取最新状态
        match load_standalone_config() {
            Ok(config) => {
                let enabled = config.mcp_config.tools.get(tool_name).copied().unwrap_or(true);
                log_debug!("工具 {} 当前状态: {}", tool_name, enabled);
                enabled
            }
            Err(e) => {
                log_important!(warn, "读取配置失败，使用缓存状态: {}", e);
                // 如果读取失败，使用缓存的配置
                self.enabled_tools.get(tool_name).copied().unwrap_or(true)
            }
        }
    }
}

impl ServerHandler for ZhiServer {
    fn get_info(&self) -> ServerInfo {
        let theme = get_theme();
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: theme.messages.server_name.clone(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some(theme.messages.server_intro.clone()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<ServerInfo, McpError> {
        Ok(self.get_info())
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        use std::sync::Arc;
        use std::borrow::Cow;

        let theme = get_theme();
        let mut tools = Vec::new();

        // 交互工具（领袖）- 必需工具，永不退场
        let interaction_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": format!("{}要传达的信息", theme.tool_interaction.display_name)
                },
                "predefined_options": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "预设的选项列表（可选）"
                },
                "is_markdown": {
                    "type": "boolean",
                    "description": "信息是否为Markdown格式，默认为true"
                }
            },
            "required": ["message"]
        });

        if let serde_json::Value::Object(schema_map) = interaction_schema {
            tools.push(Tool {
                name: Cow::Owned(theme.tool_interaction.id.clone()),
                description: Some(Cow::Owned(theme.tool_interaction.description.clone())),
                input_schema: Arc::new(schema_map),
                annotations: None,
            });
        }

        // 记忆工具（仅在启用时出战）
        if self.is_tool_enabled(&theme.tool_memory.id) {
            let memory_schema = serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "description": "任务类型：记忆(存储), 回忆(提取)"
                    },
                    "project_path": {
                        "type": "string",
                        "description": "项目路径（必需）"
                    },
                    "content": {
                        "type": "string",
                        "description": "内容（存储时必需）"
                    },
                    "category": {
                        "type": "string",
                        "description": "分类：rule(规则), preference(偏好), pattern(模式), context(上下文)"
                    }
                },
                "required": ["action", "project_path"]
            });

            if let serde_json::Value::Object(schema_map) = memory_schema {
                tools.push(Tool {
                    name: Cow::Owned(theme.tool_memory.id.clone()),
                    description: Some(Cow::Owned(theme.tool_memory.description.clone())),
                    input_schema: Arc::new(schema_map),
                    annotations: None,
                });
            }
        }

        // 搜索工具（仅在启用时出战）
        if self.is_tool_enabled(&theme.tool_search.id) {
            tools.push(AcemcpTool::get_tool_definition_with_theme(theme));
        }

        log_debug!("工具列表: {:?}", tools.iter().map(|t| &t.name).collect::<Vec<_>>());

        Ok(ListToolsResult {
            tools,
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let theme = get_theme();
        log_debug!("收到工具调用: {}", request.name);

        let tool_name = request.name.as_ref();

        // 交互工具
        if tool_name == theme.tool_interaction.id {
            let arguments_value = request.arguments
                .map(serde_json::Value::Object)
                .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

            let zhi_request: ZhiRequest = serde_json::from_value(arguments_value)
                .map_err(|e| McpError::invalid_params(
                    theme.format_msg(&theme.messages.param_parse_error_msg, &theme.tool_interaction.display_name, Some(&e.to_string())),
                    None
                ))?;

            return InteractionTool::zhi(zhi_request).await;
        }

        // 记忆工具
        if tool_name == theme.tool_memory.id {
            if !self.is_tool_enabled(&theme.tool_memory.id) {
                return Err(McpError::internal_error(
                    theme.format_msg(&theme.messages.tool_disabled_msg, &theme.tool_memory.display_name, None),
                    None
                ));
            }

            let arguments_value = request.arguments
                .map(serde_json::Value::Object)
                .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

            let ji_request: JiyiRequest = serde_json::from_value(arguments_value)
                .map_err(|e| McpError::invalid_params(
                    theme.format_msg(&theme.messages.param_parse_error_msg, &theme.tool_memory.display_name, Some(&e.to_string())),
                    None
                ))?;

            return MemoryTool::jiyi(ji_request).await;
        }

        // 搜索工具
        if tool_name == theme.tool_search.id {
            if !self.is_tool_enabled(&theme.tool_search.id) {
                return Err(McpError::internal_error(
                    theme.format_msg(&theme.messages.tool_disabled_msg, &theme.tool_search.display_name, None),
                    None
                ));
            }

            let arguments_value = request.arguments
                .map(serde_json::Value::Object)
                .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

            let acemcp_request: crate::mcp::tools::acemcp::types::AcemcpRequest = serde_json::from_value(arguments_value)
                .map_err(|e| McpError::invalid_params(
                    theme.format_msg(&theme.messages.param_parse_error_msg, &theme.tool_search.display_name, Some(&e.to_string())),
                    None
                ))?;

            return AcemcpTool::search_context(acemcp_request).await;
        }

        // 未知工具
        Err(McpError::invalid_request(
            theme.format_msg(&theme.messages.unknown_tool_msg, tool_name, None),
            None
        ))
    }
}



/// 启动MCP服务器
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // 加载主题（会打印日志）
    let theme = get_theme();
    log_important!(info, "MCP服务器主题: {} - {}", theme.name, theme.description);

    // 创建并运行服务器
    let service = ZhiServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            log_important!(error, "启动服务器失败: {}", e);
        })?;

    // 等待服务器关闭
    service.waiting().await?;
    Ok(())
}
