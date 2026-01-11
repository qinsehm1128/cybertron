// MCP 工具主题配置系统
// 支持用户自定义工具名称、描述和提示文本

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::OnceLock;

/// 全局主题缓存
static CURRENT_THEME: OnceLock<McpTheme> = OnceLock::new();

/// 单个工具的主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolTheme {
    /// 工具 ID（MCP 协议中使用的名称）
    pub id: String,
    /// 显示名称（UI 中显示）
    pub display_name: String,
    /// 工具描述
    pub description: String,
    /// 图标（UnoCSS 类名）
    pub icon: String,
    /// 图标背景色
    pub icon_bg: String,
}

/// 消息主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageTheme {
    /// 服务器名称
    pub server_name: String,
    /// 服务器介绍
    pub server_intro: String,
    /// 继续提示词
    pub continue_prompt: String,
    /// 工具禁用提示模板（{tool} 会被替换为工具名）
    pub tool_disabled_msg: String,
    /// 领袖工具不可禁用提示
    pub leader_cannot_disable_msg: String,
    /// 参数解析失败提示模板
    pub param_parse_error_msg: String,
    /// 未知工具提示模板
    pub unknown_tool_msg: String,
}

/// 完整主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTheme {
    /// 主题名称
    pub name: String,
    /// 主题描述
    pub description: String,
    /// 交互工具（领袖）
    pub tool_interaction: ToolTheme,
    /// 记忆工具
    pub tool_memory: ToolTheme,
    /// 搜索工具
    pub tool_search: ToolTheme,
    /// 消息配置
    pub messages: MessageTheme,
}

impl Default for McpTheme {
    fn default() -> Self {
        Self::transformers()
    }
}

impl McpTheme {
    /// 变形金刚主题（默认）
    pub fn transformers() -> Self {
        Self {
            name: "transformers".to_string(),
            description: "变形金刚主题 - 赛博坦军团".to_string(),
            tool_interaction: ToolTheme {
                id: "optimus".to_string(),
                display_name: "擎天柱".to_string(),
                description: "🚛 汽车人领袖！负责与人类盟友建立通信链路，支持战术选项、自由指令输入和图像情报上传".to_string(),
                icon: "i-carbon-chat text-lg text-blue-600 dark:text-blue-400".to_string(),
                icon_bg: "bg-blue-100 dark:bg-blue-900".to_string(),
            },
            tool_memory: ToolTheme {
                id: "bumblebee".to_string(),
                display_name: "大黄蜂".to_string(),
                description: "🚗 忠诚的记忆守护者！负责存储和管理重要的作战规范、盟友偏好和最佳战术".to_string(),
                icon: "i-carbon-data-base text-lg text-yellow-600 dark:text-yellow-400".to_string(),
                icon_bg: "bg-yellow-100 dark:bg-yellow-900".to_string(),
            },
            tool_search: ToolTheme {
                id: "megatron".to_string(),
                display_name: "威震天".to_string(),
                description: "🔫 强大的代码搜索引擎！掌控全局的霸主，支持语义搜索和增量索引".to_string(),
                icon: "i-carbon-search text-lg text-purple-600 dark:text-purple-400".to_string(),
                icon_bg: "bg-purple-100 dark:bg-purple-900".to_string(),
            },
            messages: MessageTheme {
                server_name: "Cybertron-MCP".to_string(),
                server_intro: "🤖 赛博坦军团 - 变形金刚代码战队！擎天柱领衔，大黄蜂守护记忆，威震天掌控搜索。汽车人，变形出发！".to_string(),
                continue_prompt: "汽车人，变形出发！继续执行任务".to_string(),
                tool_disabled_msg: "{tool}正在休眠中，请先激活！".to_string(),
                leader_cannot_disable_msg: "擎天柱是汽车人领袖，永不退场！".to_string(),
                param_parse_error_msg: "{tool}无法解析指令: {error}".to_string(),
                unknown_tool_msg: "未知的战士: {tool}，不属于赛博坦军团！".to_string(),
            },
        }
    }

    /// 经典主题（原版寸止风格）
    pub fn classic() -> Self {
        Self {
            name: "classic".to_string(),
            description: "经典主题 - 原版寸止风格".to_string(),
            tool_interaction: ToolTheme {
                id: "zhi".to_string(),
                display_name: "寸止".to_string(),
                description: "智能代码审查交互工具，支持预定义选项、自由文本输入和图片上传".to_string(),
                icon: "i-carbon-chat text-lg text-blue-600 dark:text-blue-400".to_string(),
                icon_bg: "bg-blue-100 dark:bg-blue-900".to_string(),
            },
            tool_memory: ToolTheme {
                id: "ji".to_string(),
                display_name: "记忆管理".to_string(),
                description: "全局记忆管理工具，用于存储和管理重要的开发规范、用户偏好和最佳实践".to_string(),
                icon: "i-carbon-data-base text-lg text-purple-600 dark:text-purple-400".to_string(),
                icon_bg: "bg-green-100 dark:bg-green-900".to_string(),
            },
            tool_search: ToolTheme {
                id: "sou".to_string(),
                display_name: "代码搜索".to_string(),
                description: "基于查询在特定项目中搜索相关的代码上下文，支持语义搜索和增量索引".to_string(),
                icon: "i-carbon-search text-lg text-green-600 dark:text-green-400".to_string(),
                icon_bg: "bg-green-100 dark:bg-green-900".to_string(),
            },
            messages: MessageTheme {
                server_name: "Zhi-mcp".to_string(),
                server_intro: "Zhi 智能代码审查工具，支持交互式对话和记忆管理".to_string(),
                continue_prompt: "请按照最佳实践继续".to_string(),
                tool_disabled_msg: "{tool}工具已被禁用".to_string(),
                leader_cannot_disable_msg: "寸止工具是必需的，无法禁用".to_string(),
                param_parse_error_msg: "参数解析失败: {error}".to_string(),
                unknown_tool_msg: "未知的工具: {tool}".to_string(),
            },
        }
    }

    /// 海贼王主题
    pub fn one_piece() -> Self {
        Self {
            name: "one_piece".to_string(),
            description: "海贼王主题 - 草帽海贼团".to_string(),
            tool_interaction: ToolTheme {
                id: "luffy".to_string(),
                display_name: "路飞".to_string(),
                description: "🏴‍☠️ 草帽海贼团船长！负责与伙伴们沟通，支持战斗选项、自由对话和图像情报".to_string(),
                icon: "i-carbon-chat text-lg text-red-600 dark:text-red-400".to_string(),
                icon_bg: "bg-red-100 dark:bg-red-900".to_string(),
            },
            tool_memory: ToolTheme {
                id: "robin".to_string(),
                display_name: "罗宾".to_string(),
                description: "📚 考古学家！负责记录和管理重要的历史文本、航海日志和冒险记忆".to_string(),
                icon: "i-carbon-data-base text-lg text-purple-600 dark:text-purple-400".to_string(),
                icon_bg: "bg-purple-100 dark:bg-purple-900".to_string(),
            },
            tool_search: ToolTheme {
                id: "nami".to_string(),
                display_name: "娜美".to_string(),
                description: "🗺️ 航海士！精准定位代码宝藏，支持语义搜索和智能导航".to_string(),
                icon: "i-carbon-search text-lg text-orange-600 dark:text-orange-400".to_string(),
                icon_bg: "bg-orange-100 dark:bg-orange-900".to_string(),
            },
            messages: MessageTheme {
                server_name: "StrawHat-MCP".to_string(),
                server_intro: "🏴‍☠️ 草帽海贼团代码冒险！路飞领航，罗宾守护记忆，娜美掌控搜索。出发，去找 One Piece！".to_string(),
                continue_prompt: "我要成为代码之王！继续冒险".to_string(),
                tool_disabled_msg: "{tool}正在休息中，请先叫醒！".to_string(),
                leader_cannot_disable_msg: "路飞是船长，不能下船！".to_string(),
                param_parse_error_msg: "{tool}看不懂这个指令: {error}".to_string(),
                unknown_tool_msg: "这个人不是我们的伙伴: {tool}".to_string(),
            },
        }
    }

    /// 火影忍者主题
    pub fn naruto() -> Self {
        Self {
            name: "naruto".to_string(),
            description: "火影忍者主题 - 木叶忍者".to_string(),
            tool_interaction: ToolTheme {
                id: "naruto".to_string(),
                display_name: "鸣人".to_string(),
                description: "🍥 七代目火影！负责与忍者们沟通，支持忍术选项、自由对话和影像情报".to_string(),
                icon: "i-carbon-chat text-lg text-orange-600 dark:text-orange-400".to_string(),
                icon_bg: "bg-orange-100 dark:bg-orange-900".to_string(),
            },
            tool_memory: ToolTheme {
                id: "shikamaru".to_string(),
                display_name: "鹿丸".to_string(),
                description: "🧠 天才军师！负责记录和管理重要的战术情报、任务记录和忍术心得".to_string(),
                icon: "i-carbon-data-base text-lg text-green-600 dark:text-green-400".to_string(),
                icon_bg: "bg-green-100 dark:bg-green-900".to_string(),
            },
            tool_search: ToolTheme {
                id: "sasuke".to_string(),
                display_name: "佐助".to_string(),
                description: "👁️ 写轮眼使者！精准洞察代码真相，支持语义搜索和深度分析".to_string(),
                icon: "i-carbon-search text-lg text-indigo-600 dark:text-indigo-400".to_string(),
                icon_bg: "bg-indigo-100 dark:bg-indigo-900".to_string(),
            },
            messages: MessageTheme {
                server_name: "Konoha-MCP".to_string(),
                server_intro: "🍃 木叶忍者代码任务！鸣人领队，鹿丸守护情报，佐助掌控搜索。这就是我的忍道！".to_string(),
                continue_prompt: "我绝不会放弃！继续执行任务".to_string(),
                tool_disabled_msg: "{tool}正在修炼中，请先召唤！".to_string(),
                leader_cannot_disable_msg: "鸣人是火影，不能退缩！".to_string(),
                param_parse_error_msg: "{tool}无法理解这个忍术: {error}".to_string(),
                unknown_tool_msg: "这个忍者不属于木叶: {tool}".to_string(),
            },
        }
    }

    /// 根据名称获取主题
    pub fn from_name(name: &str) -> Self {
        match name {
            "classic" => Self::classic(),
            "one_piece" => Self::one_piece(),
            "naruto" => Self::naruto(),
            _ => Self::transformers(),
        }
    }

    /// 获取所有可用主题名称
    pub fn available_themes() -> Vec<(&'static str, &'static str)> {
        vec![
            ("transformers", "变形金刚 - 赛博坦军团"),
            ("classic", "经典 - 原版寸止风格"),
            ("one_piece", "海贼王 - 草帽海贼团"),
            ("naruto", "火影忍者 - 木叶忍者"),
        ]
    }

    /// 从 JSON 文件加载主题
    pub fn from_file(path: &std::path::Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("无法读取主题文件: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("主题文件格式错误: {}", e))
    }

    /// 格式化消息模板（替换 {tool} 和 {error} 占位符）
    pub fn format_msg(&self, template: &str, tool: &str, error: Option<&str>) -> String {
        let mut result = template.replace("{tool}", tool);
        if let Some(err) = error {
            result = result.replace("{error}", err);
        }
        result
    }
}

/// 获取主题配置文件路径
fn get_theme_config_path() -> Option<PathBuf> {
    // 优先级：
    // 1. 与可执行文件同目录的 theme.json
    // 2. 用户配置目录 ~/.cunzhi/theme.json
    // 3. 与可执行文件同目录的 themes/ 目录下的主题文件

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // 检查同目录的 theme.json
            let theme_file = exe_dir.join("theme.json");
            if theme_file.exists() {
                return Some(theme_file);
            }
        }
    }

    // 检查用户配置目录
    if let Some(home) = dirs::home_dir() {
        let user_theme = home.join(".cunzhi").join("theme.json");
        if user_theme.exists() {
            return Some(user_theme);
        }
    }

    None
}

/// 加载主题配置（启动时调用一次）
pub fn load_theme() -> McpTheme {
    // 尝试从配置文件加载
    if let Some(config_path) = get_theme_config_path() {
        match McpTheme::from_file(&config_path) {
            Ok(theme) => {
                log::info!("已加载主题配置: {} ({})", theme.name, config_path.display());
                return theme;
            }
            Err(e) => {
                log::warn!("加载主题配置失败，使用默认主题: {}", e);
            }
        }
    }

    // 检查环境变量指定的主题名称
    if let Ok(theme_name) = std::env::var("CUNZHI_THEME") {
        log::info!("使用环境变量指定的主题: {}", theme_name);
        return McpTheme::from_name(&theme_name);
    }

    // 使用默认主题
    log::info!("使用默认主题: transformers");
    McpTheme::default()
}

/// 获取当前主题（全局单例）
pub fn get_theme() -> &'static McpTheme {
    CURRENT_THEME.get_or_init(load_theme)
}
