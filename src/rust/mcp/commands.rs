use std::collections::HashMap;
use tauri::{AppHandle, State};

use crate::config::{AppState, save_config};
use crate::constants::mcp;
// use crate::mcp::tools::acemcp; // 已迁移到独立模块

/// MCP工具配置
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MCPToolConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub can_disable: bool,
    pub icon: String,
    pub icon_bg: String,
    pub dark_icon_bg: String,
    pub has_config: bool, // 是否有配置选项
}

/// 获取MCP工具配置列表
#[tauri::command]
pub async fn get_mcp_tools_config(state: State<'_, AppState>) -> Result<Vec<MCPToolConfig>, String> {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    
    // 动态构建赛博坦军团配置列表
    let mut tools = Vec::new();
    
    // 擎天柱 - 领袖级交互核心，永不退场
    tools.push(MCPToolConfig {
        id: mcp::TOOL_OPTIMUS.to_string(),
        name: "擎天柱".to_string(),
        description: "🚛 汽车人领袖！负责与人类盟友建立通信链路，支持战术选项、自由指令输入和图像情报上传".to_string(),
        enabled: config.mcp_config.tools.get(mcp::TOOL_OPTIMUS).copied().unwrap_or(true),
        can_disable: false, // 领袖永不退场
        icon: "i-carbon-chat text-lg text-blue-600 dark:text-blue-400".to_string(),
        icon_bg: "bg-blue-100 dark:bg-blue-900".to_string(),
        dark_icon_bg: "dark:bg-blue-800".to_string(),
        has_config: false,
    });
    
    // 大黄蜂 - 忠诚的记忆守护者
    tools.push(MCPToolConfig {
        id: mcp::TOOL_BUMBLEBEE.to_string(),
        name: "大黄蜂".to_string(),
        description: "🚗 忠诚的记忆守护者！负责存储和管理重要的作战规范、盟友偏好和最佳战术".to_string(),
        enabled: config.mcp_config.tools.get(mcp::TOOL_BUMBLEBEE).copied().unwrap_or(false),
        can_disable: true,
        icon: "i-carbon-data-base text-lg text-yellow-600 dark:text-yellow-400".to_string(),
        icon_bg: "bg-yellow-100 dark:bg-yellow-900".to_string(),
        dark_icon_bg: "dark:bg-yellow-800".to_string(),
        has_config: false,
    });
    
    // 威震天 - 强大的代码搜索引擎
    tools.push(MCPToolConfig {
        id: mcp::TOOL_MEGATRON.to_string(),
        name: "威震天".to_string(),
        description: "🔫 强大的代码搜索引擎！掌控全局的霸主，支持语义搜索和增量索引".to_string(),
        enabled: config.mcp_config.tools.get(mcp::TOOL_MEGATRON).copied().unwrap_or(false),
        can_disable: true,
        icon: "i-carbon-search text-lg text-purple-600 dark:text-purple-400".to_string(),
        icon_bg: "bg-purple-100 dark:bg-purple-900".to_string(),
        dark_icon_bg: "dark:bg-purple-800".to_string(),
        has_config: true,
    });
    
    // 按启用状态排序，出战的在前
    tools.sort_by(|a, b| b.enabled.cmp(&a.enabled));
    
    Ok(tools)
}

/// 设置MCP工具启用状态
#[tauri::command]
pub async fn set_mcp_tool_enabled(
    tool_id: String,
    enabled: bool,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        
        // 擎天柱是领袖，不可禁用
        if tool_id == mcp::TOOL_OPTIMUS && !enabled {
            return Err("擎天柱是汽车人领袖，永不退场！".to_string());
        }
        
        // 更新战士状态
        config.mcp_config.tools.insert(tool_id.clone(), enabled);
    }
    
    // 保存配置
    save_config(&state, &app).await
        .map_err(|e| format!("保存配置失败: {}", e))?;

    // 记录状态变更
    log::info!("赛博坦战士 {} 状态已更新为: {}", tool_id, if enabled { "出战" } else { "待命" });

    Ok(())
}

/// 获取所有MCP工具状态
#[tauri::command]
pub async fn get_mcp_tools_status(state: State<'_, AppState>) -> Result<HashMap<String, bool>, String> {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.mcp_config.tools.clone())
}

/// 重置MCP工具配置为默认值
#[tauri::command]
pub async fn reset_mcp_tools_config(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        let default_config = mcp::get_default_mcp_config();
        config.mcp_config.tools.clear();
        for tool in &default_config.tools {
            config.mcp_config.tools.insert(tool.tool_id.clone(), tool.enabled);
        }
    }
    
    // 保存配置
    save_config(&state, &app).await
        .map_err(|e| format!("保存配置失败: {}", e))?;

    // 记录配置重置
    log::info!("赛博坦军团配置已重置为初始状态！汽车人，变形出发！");
    Ok(())
}

// acemcp 相关命令已迁移

// 已移除 Python Web 服务相关函数，完全使用 Rust 实现
// 如需调试配置，请直接查看本地配置文件
