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
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "Cybertron-MCP".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some("🤖 赛博坦军团 - 变形金刚代码战队！擎天柱领衔，大黄蜂守护记忆，威震天掌控搜索。汽车人，变形出发！".to_string()),
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

        let mut tools = Vec::new();

        // 擎天柱 - 领袖级交互核心（必需工具，永不退场）
        let optimus_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "擎天柱要传达给人类盟友的信息"
                },
                "predefined_options": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "预设的战术选项列表（可选）"
                },
                "is_markdown": {
                    "type": "boolean",
                    "description": "信息是否为Markdown格式，默认为true"
                }
            },
            "required": ["message"]
        });

        if let serde_json::Value::Object(schema_map) = optimus_schema {
            tools.push(Tool {
                name: Cow::Borrowed("optimus"),
                description: Some(Cow::Borrowed("🚛 擎天柱 - 汽车人领袖！负责与人类盟友建立通信链路，支持战术选项、自由指令输入和图像情报上传。「自由是所有智慧生命的权利」")),
                input_schema: Arc::new(schema_map),
                annotations: None,
            });
        }

        // 大黄蜂 - 忠诚的记忆守护者（仅在启用时出战）
        if self.is_tool_enabled("bumblebee") {
            let bumblebee_schema = serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "description": "任务类型：记忆(存储情报), 回忆(提取战场信息)"
                    },
                    "project_path": {
                        "type": "string",
                        "description": "作战基地路径（必需）"
                    },
                    "content": {
                        "type": "string",
                        "description": "情报内容（存储任务时必需）"
                    },
                    "category": {
                        "type": "string",
                        "description": "情报分类：rule(作战规则), preference(盟友偏好), pattern(战术模式), context(战场背景)"
                    }
                },
                "required": ["action", "project_path"]
            });

            if let serde_json::Value::Object(schema_map) = bumblebee_schema {
                tools.push(Tool {
                    name: Cow::Borrowed("bumblebee"),
                    description: Some(Cow::Borrowed("🚗 大黄蜂 - 忠诚的记忆守护者！负责存储和管理重要的作战规范、盟友偏好和最佳战术。虽然声带受损，但记忆永不磨灭！")),
                    input_schema: Arc::new(schema_map),
                    annotations: None,
                });
            }
        }

        // 威震天 - 强大的代码搜索引擎（仅在启用时苏醒）
        if self.is_tool_enabled("megatron") {
            tools.push(AcemcpTool::get_tool_definition());
        }

        log_debug!("赛博坦军团出战名单: {:?}", tools.iter().map(|t| &t.name).collect::<Vec<_>>());

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
        log_debug!("收到作战指令: {}", request.name);

        match request.name.as_ref() {
            "optimus" => {
                // 解析作战参数
                let arguments_value = request.arguments
                    .map(serde_json::Value::Object)
                    .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

                let zhi_request: ZhiRequest = serde_json::from_value(arguments_value)
                    .map_err(|e| McpError::invalid_params(format!("擎天柱无法解析指令: {}", e), None))?;

                // 擎天柱出击
                InteractionTool::zhi(zhi_request).await
            }
            "bumblebee" => {
                // 检查大黄蜂是否已激活
                if !self.is_tool_enabled("bumblebee") {
                    return Err(McpError::internal_error(
                        "大黄蜂正在休眠中，请先激活！".to_string(),
                        None
                    ));
                }

                // 解析情报参数
                let arguments_value = request.arguments
                    .map(serde_json::Value::Object)
                    .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

                let ji_request: JiyiRequest = serde_json::from_value(arguments_value)
                    .map_err(|e| McpError::invalid_params(format!("大黄蜂无法解析情报: {}", e), None))?;

                // 大黄蜂执行记忆任务
                MemoryTool::jiyi(ji_request).await
            }
            "megatron" => {
                // 检查威震天是否已苏醒
                if !self.is_tool_enabled("megatron") {
                    return Err(McpError::internal_error(
                        "威震天尚未苏醒，请先唤醒！".to_string(),
                        None
                    ));
                }

                // 解析搜索参数
                let arguments_value = request.arguments
                    .map(serde_json::Value::Object)
                    .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

                // 使用acemcp模块中的AcemcpRequest类型
                let acemcp_request: crate::mcp::tools::acemcp::types::AcemcpRequest = serde_json::from_value(arguments_value)
                    .map_err(|e| McpError::invalid_params(format!("威震天无法解析目标: {}", e), None))?;

                // 威震天发动搜索攻势
                AcemcpTool::search_context(acemcp_request).await
            }
            _ => {
                Err(McpError::invalid_request(
                    format!("未知的战士: {}，不属于赛博坦军团！", request.name),
                    None
                ))
            }
        }
    }
}



/// 启动MCP服务器
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
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
