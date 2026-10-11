/// Command help translations for slash commands.
/// Returns Chinese explanation for the given command name if available,
/// otherwise falls back to `fallback_en`.
pub fn translate_command_help(name: &str, fallback_en: &'static str) -> &'static str {
    // Strip leading slash for robust matching if needed, or match with slash
    let cmd = name.trim();
    match cmd {
        "/help" | "/?" => "显示帮助与快捷键列表",
        "/commands" => "/help 的别名",
        "/model" => "列出或切换大模型",
        "/models" => "/model 的别名",
        "/provider-test-coverage" => "查看当前供应商/模型的实测验证数据",
        "/model-status" => "/provider-test-coverage 的别名",
        "/refresh-model-list" => "刷新供应商模型列表目录",
        "/agents" => "配置各个智能体角色的模型",
        "/swarm-prompt" => "在编辑器中打开活跃的群体调度提示词",
        "/subagent" => "手动启动子智能体",
        "/observe" => "在侧边栏显示最新的工具上下文",
        "/todos" => "在对话中以卡片形式显示待办清单",
        "/todo" => "/todos 的别名",
        "/splitview" => "在侧边栏镜像显示当前对话",
        "/split-view" => "/splitview 的别名",
        "/btw" => "在侧边栏发起侧边提问",
        "/ssh" => "使用系统 SSH 连接远程机器",
        "/git" => "显示当前会话工作目录的 Git 状态",
        "/colors" => "查看、配置并评分终端所有颜色主题",
        "/color" => "/colors 的别名",
        "/hotkeys" => "查看快捷键列表及个人使用频次",
        "/terminal-setup" => "修复 Shift+Enter 换行设置",
        "/commit" => "基于当前更改生成逻辑合理的 Git 提交",
        "/merge" => "将当前分支合并到主分支并切换（不推送）",
        "/commit-push" => "生成 Git 提交并推送到远端",
        "/commit-and-push" => "/commit-push 的别名",
        "/fast-release" => "通过本地 selfdev 缓存立即发布 Linux 版本",
        "/fast-macos-release" => "立即发布准备好的 macOS arm64 构建",
        "/remote" => "允许从其他机器连接此会话",
        "/merge-remote-release" => "合并入主分支、验证、推送并远程发布",
        "/remote-release" => "立即推送发布标签；由 CI 构建并发布各平台",
        "/cut-release" => "/fast-release 的别名",
        "/commit-push-release" => "/cut-release 的别名",
        "/triage" => "分流 GitHub Issue 并自动修复安全问题",
        "/transcript" => "打开当前会话的完整记录文件",
        "/subagent-model" => "查看或更改子智能体的模型策略",
        "/autoreview" => "查看或切换每轮结束时的自动代码审查",
        "/autojudge" => "查看或切换每轮结束时的自动评判判定",
        "/review" => "启动单次前端交互式审查会话",
        "/judge" => "启动单次前端交互式评判会话",
        "/effort" => "设置推理深度/思考强度 (none|low|medium|high|max)",
        "/fast" => "切换极速模式开关",
        "/transport" => "查看或更改连接传输协议 (auto|https|websocket)",
        "/alignment" => "查看或更改默认文本对齐方式",
        "/compact-notifications" => "切换单行蜂群/文件活动紧凑通知",
        "/show-agentgrep-output" => "在对话中内联展开完整的 agentgrep 搜索输出",
        "/tool-call-details" => "在带有意图的工具行显示暗淡的技术详情",
        "/thinking-display" => "显示或隐藏模型的思考推理过程 (off/full/current)",
        "/thinking" => "/thinking-display 的别名",
        "/reasoning" => "/thinking-display 的别名",
        "/cancel" => "取消当前提示词或正在执行的操作",
        "/clear" => "清空对话历史并重新开始",
        "/cls" => "仅清空当前屏幕显示，保留上下文记忆",
        "/clear-view" => "/cls 的别名",
        "/rewind" => "倒回会话历史到上一步消息",
        "/poke" => "唤醒模型以继续推进未完成的待办事项",
        "/plan" => "生成仅规划响应并显示为计划卡片",
        "/improve" => "自主持续改进优化当前代码仓库",
        "/refactor" => "启动带独立审查的安全重构循环",
        "/compact" => "压缩提炼上下文以释放记忆空间",
        "/fix" => "在模型无法继续时尝试自动恢复诊断",
        "/voice" => "语音输入：说话后发送 (Ctrl+Space)",
        "/dictate" => "执行已配置的外部听写命令",
        "/dictation" => "/dictate 的别名",
        "/memory" => "切换记忆库功能开关",
        "/test" => "使用分层测试验证修改与断言",
        "/initiatives" => "打开计划概览 / 恢复跟踪的开发行动",
        "/goals" => "/initiatives 的旧版别名",
        "/swarm" => "切换蜂群多智能体协作开关",
        "/overnight" => "运行无人值守的通宵任务协调器",
        "/context" => "显示当前会话的完整上下文快照",
        "/skills" => "查看已加载技能与推荐扩展",
        "/version" => "查看当前版本及构建信息",
        "/changelog" => "查看当前版本的更新日志与更新详情",
        "/info" => "查看会话详情与 Token 用量统计",
        "/reset" => "审查并确认 OpenAI 用量重置状态",
        "/usage" => "查看已连接供应商的用量额度",
        "/productivity" => "生成可分享的生产力用量报告与看板图",
        "/wrapped" => "/productivity 的别名",
        "/feedback" => "发送关于 jcode 的使用反馈",
        "/telemetry" => "查看或更改遥测数据上报策略",
        "/support" => "发送预填诊断信息的支持邮件",
        "/subscription" => "查看 jcode 订阅状态",
        "/subscribe" => "了解并订阅 jcode 专业版",
        "/config" => "查看或编辑系统配置",
        "/log" => "在 jcode 日志中标记当前操作锚点",
        "/keys" => "检测终端与操作系统的快捷键冲突情况",
        "/keybindings" => "/keys 的别名",
        "/diff" => "切换或设置差异对比模式 (off/inline/full/file)",
        "/onboarding-preview" => "预览首次启动新手引导屏幕",
        "/onboarding-sim" => "以新用户身份模拟演练新手引导流程",
        "/reload" => "重载至可用的最新程序版本",
        "/restart" => "使用当前可执行文件快速重启",
        "/rebuild" => "后台重新编译构建并自动重载",
        "/selfdev" => "开启自我迭代自举开发会话",
        "/update" => "后台下载最新版本并自动重载",
        "/update-sim" => "安全预览版本更新界面 (Alt+_)",
        "/update-rehearsal" => "自我开发：在沙盒中模拟真实更新",
        "/resume" => "打开会话选择器恢复历史对话",
        "/sessions" => "/resume 的别名",
        "/session" => "/resume 的别名",
        "/active" => "管理活跃会话（进行中 vs 已就绪）",
        "/catchup" => "打开追赶选择器速览已完成会话",
        "/back" => "返回上一个追赶会话来源",
        "/save" => "收藏/书签当前会话以便快速访问",
        "/unsave" => "取消当前会话的书签收藏",
        "/rename" => "重命名当前会话",
        "/fork" => "将会话分叉并在新窗口打开",
        "/split" => "/fork 的别名",
        "/desktop" => "在 Jcode Desktop 桌面端打开此会话",
        "/transfer" => "将上下文压缩并交接至全新纯净会话",
        "/cloud" => "将当前会话迁移至云端机器继续执行",
        "/local" => "将云端会话拉回本机继续执行",
        "/workspace" => "Niri 风格的多工作区管理",
        "/quit" => "退出 jcode",
        "/auth" => "查看身份验证与登录状态",
        "/login" => "登录大模型供应商",
        "/logout" => "退出登录指定供应商",
        "/account" => "打开多账号切换选择器",
        "/accounts" => "/account 的别名",
        "/cache" => "查看缓存命中统计与 TTL 保持",
        "/debug-visual" => "切换可视化调试覆盖层",
        "/screenshot-mode" => "切换屏幕截图捕获模式",
        "/screenshot" => "捕获当前界面的调试状态快照",
        "/record" => "录制演示捕获",
        "/client-reload" => "强制重载客户端二进制程序",
        "/server-reload" => "强制重载服务端二进制程序",
        "/continue" => "继续所有被中断的可自动恢复会话",
        "/resumeall" => "/continue 的别名",
        "/resume-all" => "/continue 的别名",
        "/lang" => "切换界面语言 (zh/en)",
        "/scrollback" => "切换 Scrollback 原生回滚模式与全屏模式 (丝滑原生滚动)",
        "/tui" => "/scrollback 的别名 (兼容 Codex CLI 模式切换)",
        _ => fallback_en,
    }
}

/// Detailed multi-line help documentation for `/help <topic>`
pub fn translate_detailed_help(topic: &str) -> Option<&'static str> {
    let help = match topic.trim().trim_start_matches('/').to_ascii_lowercase().as_str() {
        "scrollback" | "tui" => {
            "/scrollback [on|off]\n\
             切换终端原生滚动回滚模式（Scrollback Native Mode）与全屏备用屏模式（Fullscreen Mode）。\n\n\
             • on / scrollback: 禁用备用屏幕（Alternate Screen Buffer）与鼠标捕获，直接使用终端原生历史回滚缓冲区，SSH 远程开发丝滑流畅、零网络重绘延迟，终端原生选中文本与鼠标滚轮 100% 正常生效！\n\
             • off / fullscreen: 恢复全屏备用屏幕模式。\n\n\
             持久化配置：\n\
             在配置文件 `config.toml` 中添加：\n\
             [display]\n\
             fullscreen_transcript = false\n\n\
             环境变量：\n\
             `export JCODE_SCROLLBACK=1` 或 `export JCODE_FULLSCREEN_TRANSCRIPT=0`"
        }
        "help" | "commands" => {
            "/help\n显示命令列表与快捷键指南。\n\n/help <command>\n显示指定命令的详细说明文档。"
        }
        "compact" => {
            "/compact\n立即强制压缩当前对话上下文。\n启动后台总结提炼，完成后自动应用并释放 Token 空间。\n\n/compact mode\n查看当前会话的上下文压缩模式。\n\n/compact mode <reactive|proactive|semantic>\n修改压缩模式 (被动/主动/语义压缩)。"
        }
        "cache" => {
            "/cache 或 /cache stats\n查看当前会话的 KV 缓存命中统计数据。\n\n/cache extend 或 /cache 1h\n对后续请求启用 1 小时 Anthropic 缓存保持。\n\n/cache 5m\n设置 5 分钟缓存保持。"
        }
        "fix" => {
            "/fix\n在模型受阻中断、无法继续推进时执行诊断修复。\n修复丢失的工具输出、重置供应商会话状态，并适时触发上下文压缩。"
        }
        "rewind" => {
            "/rewind\n显示带数字编号的对话历史。\n\n/rewind N\n倒回到第 N 条消息 (丢弃其后的所有交互并重置会话)。\n\n/rewind undo\n撤销上一次倒回操作，恢复被移除的历史消息。"
        }
        "clear" => {
            "/clear\n清空当前对话、执行队列和屏幕显示，全新开启会话。"
        }
        "cls" | "clear-view" => {
            "/cls\n仅清空当前屏幕渲染内容。模型的上下文记忆完全保留，不发送也不遗忘任何信息。(Ctrl+L 清屏但历史仍留在滚动缓冲中)。"
        }
        "model" => {
            "/model\n打开大模型选择器。\n\n/model <名称>\n切换至指定大模型。\n\n/model <名称>@<供应商>\n固定 OpenRouter 路由 (@auto 解除固定)。"
        }
        "provider-test-coverage"
        | "provider test coverage"
        | "model-status"
        | "model status" => {
            "/provider-test-coverage\n查看当前供应商与模型的 jcode 实测验证数据与检查点。\n\n/provider-test-coverage <供应商> <模型>\n查询特定组合的测试覆盖台账。"
        }
        "refresh-model-list" => {
            "/refresh-model-list\n强制刷新所有供应商的模型目录，更新 /model 列表并持久化缓存。"
        }
        "agents" => {
            "/agents\n配置当前会话各个智能体角色 (Swarm/审查/评判/记忆/后台) 的大模型分配。\n\n/agents default [角色]\n编辑所有未单独指定会话的全局默认角色模型。"
        }
        "swarm-prompt" => {
            "/swarm-prompt\n在外部编辑器中打开当前活跃的 Swarm 路由提示词文件。"
        }
        "subagent" => {
            "/subagent <提示词>\n立即启动一个独立的子智能体执行子任务。\n\n可选参数：\n  --type <类型>         设置子智能体类型 (默认 general)\n  --model <名称>        覆盖本次执行使用的模型\n  --continue <id>       恢复已有的子智能体工作会话"
        }
        "observe" => {
            "/observe\n切换侧边栏的瞬态观察模式开关 (on/off/status)。\n显示最近加入上下文的工具调用或工具结果。"
        }
        "todos" | "todo" => {
            "/todos\n在对话中以交互式卡片形式展示当前会话的待办清单。\n\n/todos panel\n切换侧边栏独立待办屏幕。\n\n/todos pin\n固定待办清单至聊天顶部。"
        }
        "splitview" | "split-view" => {
            "/splitview\n在侧边栏镜像显示当前对话。\n\n/splitview on/off/status\n启用/禁用或查看侧边栏镜像状态。"
        }
        "btw" => {
            "/btw <问题>\n在不打断当前主任务的前提下发起侧边提问。\n将会话克隆并分叉到新窗口，新窗口专心回答此问题，原会话继续不间断工作。"
        }
        "lang" | "language" => {
            "/lang\n查看当前界面语言。\n\n/lang zh\n切换为中文界面。\n\n/lang en\n切换为英文界面。"
        }
        _ => return None,
    };
    Some(help)
}
