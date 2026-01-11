use anyhow::Result;
use rmcp::{Error as McpError, model::*};

use super::{MemoryManager, MemoryCategory};
use crate::mcp::{JiyiRequest, utils::{validate_project_path, project_path_error}};

/// 🚗 大黄蜂 - 忠诚的记忆守护者
///
/// 负责存储和管理重要的作战规范、盟友偏好和最佳战术
/// 虽然声带受损，但记忆永不磨灭！
#[derive(Clone)]
pub struct MemoryTool;

impl MemoryTool {
    pub async fn jiyi(
        request: JiyiRequest,
    ) -> Result<CallToolResult, McpError> {
        // 使用增强的路径验证功能
        if let Err(e) = validate_project_path(&request.project_path) {
            return Err(project_path_error(format!(
                "大黄蜂无法定位作战基地: {}\n原始路径: {}\n请检查路径格式是否正确，特别是 Windows 路径应使用正确的盘符格式（如 C:\\path）",
                e,
                request.project_path
            )).into());
        }

        let manager = MemoryManager::new(&request.project_path)
            .map_err(|e| McpError::internal_error(format!("大黄蜂记忆系统初始化失败: {}", e), None))?;

        let result = match request.action.as_str() {
            "记忆" => {
                if request.content.trim().is_empty() {
                    return Err(McpError::invalid_params("大黄蜂需要情报内容才能存储！".to_string(), None));
                }

                let category = match request.category.as_str() {
                    "rule" => MemoryCategory::Rule,
                    "preference" => MemoryCategory::Preference,
                    "pattern" => MemoryCategory::Pattern,
                    "context" => MemoryCategory::Context,
                    _ => MemoryCategory::Context,
                };

                let id = manager.add_memory(&request.content, category)
                    .map_err(|e| McpError::internal_error(format!("大黄蜂存储情报失败: {}", e), None))?;

                format!("🚗 大黄蜂已存储情报！\n📋 情报ID: {}\n📝 内容: {}\n📂 分类: {:?}", id, request.content, category)
            }
            "回忆" => {
                manager.get_project_info()
                    .map_err(|e| McpError::internal_error(format!("大黄蜂提取情报失败: {}", e), None))?
            }
            _ => {
                return Err(McpError::invalid_params(
                    format!("大黄蜂不理解这个指令: {}", request.action),
                    None
                ));
            }
        };

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }
}
