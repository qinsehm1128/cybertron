use std::collections::HashMap;
use tauri::{AppHandle, State};

use crate::config::{AppState, save_config};
use crate::constants::themes::get_theme;

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
    let theme = get_theme();
    
    // 动态构建工具配置列表
    let mut tools = Vec::new();
    
    // 交互工具（领袖）- 永不退场
    tools.push(MCPToolConfig {
        id: theme.tool_interaction.id.clone(),
        name: theme.tool_interaction.display_name.clone(),
        description: theme.tool_interaction.description.clone(),
        enabled: config.mcp_config.tools.get(&theme.tool_interaction.id).copied().unwrap_or(true),
        can_disable: false, // 领袖永不退场
        icon: theme.tool_interaction.icon.clone(),
        icon_bg: theme.tool_interaction.icon_bg.clone(),
        dark_icon_bg: "dark:bg-blue-800".to_string(),
        has_config: false,
    });
    
    // 记忆工具
    tools.push(MCPToolConfig {
        id: theme.tool_memory.id.clone(),
        name: theme.tool_memory.display_name.clone(),
        description: theme.tool_memory.description.clone(),
        enabled: config.mcp_config.tools.get(&theme.tool_memory.id).copied().unwrap_or(false),
        can_disable: true,
        icon: theme.tool_memory.icon.clone(),
        icon_bg: theme.tool_memory.icon_bg.clone(),
        dark_icon_bg: "dark:bg-yellow-800".to_string(),
        has_config: false,
    });
    
    // 搜索工具
    tools.push(MCPToolConfig {
        id: theme.tool_search.id.clone(),
        name: theme.tool_search.display_name.clone(),
        description: theme.tool_search.description.clone(),
        enabled: config.mcp_config.tools.get(&theme.tool_search.id).copied().unwrap_or(false),
        can_disable: true,
        icon: theme.tool_search.icon.clone(),
        icon_bg: theme.tool_search.icon_bg.clone(),
        dark_icon_bg: "dark:bg-purple-800".to_string(),
        has_config: true,
    });
    
    // 按启用状态排序，启用的在前
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
    let theme = get_theme();
    
    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        
        // 领袖工具不可禁用
        if tool_id == theme.tool_interaction.id && !enabled {
            return Err(theme.messages.leader_cannot_disable_msg.clone());
        }
        
        // 更新工具状态
        config.mcp_config.tools.insert(tool_id.clone(), enabled);
    }
    
    // 保存配置
    save_config(&state, &app).await
        .map_err(|e| format!("保存配置失败: {}", e))?;

    // 记录状态变更
    log::info!("工具 {} 状态已更新为: {}", tool_id, if enabled { "启用" } else { "禁用" });

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
    let theme = get_theme();
    
    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        config.mcp_config.tools.clear();
        // 领袖工具默认启用，其他默认禁用
        config.mcp_config.tools.insert(theme.tool_interaction.id.clone(), true);
        config.mcp_config.tools.insert(theme.tool_memory.id.clone(), false);
        config.mcp_config.tools.insert(theme.tool_search.id.clone(), false);
    }
    
    // 保存配置
    save_config(&state, &app).await
        .map_err(|e| format!("保存配置失败: {}", e))?;

    // 记录配置重置
    log::info!("MCP工具配置已重置为默认值");
    Ok(())
}

// acemcp 相关命令已迁移

// 已移除 Python Web 服务相关函数，完全使用 Rust 实现
// 如需调试配置，请直接查看本地配置文件
