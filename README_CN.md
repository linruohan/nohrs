# Nohrs
一款面向 macOS 系统的高速、灵活且可扩展的文件管理器

基于 Rust 🦀 与 gpui 构建，性能卓越

---
作为 macOS 访达（Finder）的现代化替代工具，它通过简洁流畅、高性能的操作界面，兼顾了日常使用的便捷性与高级用户的功能需求。
计划加入聚焦（Spotlight）式导航、云联动工作流与 AI 智能助手功能，让文件处理更高效、更智能。

开发指南

- 工具链：Rust（稳定版），通过 rust-toolchain.toml 文件锁定版本

- 构建核心库：cargo build

- 构建图形界面二进制文件（占位 UI）：cargo build --features gui

  - 运行图形界面二进制文件：cargo run --features gui --bin nohrs

注意事项

- 目前图形界面仅为占位入口，待选定 gpui 固定版本后，将完成与 gpui 的对接工作。

macOS 系统下 gpui 开发环境依赖

gpui 在 macOS 系统中基于 Metal 图形框架运行，需提前安装 Xcode 与 Metal 工具链。

1. 从 App Store 下载并安装 Xcode（首次启动需完成初始化配置）

2. 安装命令行工具：


  - xcode-select --install

3. 确保命令行工具使用已安装的 Xcode 版本：


  - sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer

4. 若构建过程提示缺失 Metal 工具链，执行以下命令获取：


  - xcodebuild -downloadComponent MetalToolchain

完成上述配置后，重新尝试构建图形界面版本。

计划功能

导航与界面

- [ ] 标签页与分屏视图，支持多目录并行操作

- [ ] 图片、PDF、纯文本与 Markdown 文件的内嵌预览

- [ ] 命令面板，支持快速操作搜索（类 VS Code 风格）

- [ ] 聚焦（Spotlight）式界面，实现高效的键盘驱动导航

- [ ] 文件图标自定义与表情标签功能

文件操作

- [ ] 支持 .txt与 .md 文件的即时编辑

- [ ] 批量重命名，支持正则表达式与元数据规则

- [ ] 高级拖拽功能（支持 S3 上传、Git 暂存区提交）

- [ ] 剪贴板历史记录，支持多文件复制

搜索与索引

- [ ] 基于 Tantivy + ripgrep 的高速全文搜索（支持模糊匹配）

- [ ] 智能文件夹，可按标签、文件类型或日期筛选

- [ ] 预览内容内搜索（支持 PDF、Markdown、代码文件）

- [ ] 基于文件打开频率、最近使用时间与相关度的排序机制

终端集成

- [ ] 内置伪终端（PTY），与当前目录自动关联

- [ ] 拖拽粘贴文件路径功能

- [ ] 任务运行器，支持一键执行命令

Git 集成

- [ ] 侧边栏显示仓库状态与分支信息

- [ ] 差异对比预览与代码溯源（Blame）视图

- [ ] 合并冲突可视化解决界面

云服务功能

- [ ] 云存储集成（支持各类 S3 兼容服务）

- [ ] 跨设备同步与离线优先工作流

- [ ] 安全共享功能，支持访问权限管控

S3 兼容存储

- [ ] 支持 MinIO、Wasabi、Cloudflare R2 等服务

- [ ] 传输队列与并行上传机制

- [ ] 元数据编辑与预签名 URL 生成

- [ ] 离线缓存与同步恢复功能

自动化与扩展

- [ ] 插件系统，支持自定义界面与功能扩展

- [ ] 文件夹监控触发动作（自动打标签、自动传输）

- [ ] 命令行接口 / HTTP 接口，支持外部程序控制

- [ ] 基于 SSH 的远程文件浏览

AI 智能助手功能

- [ ] AI 智能助手，辅助文件整理与工作流执行

- [ ] 自然语言指令支持（查找、移动、汇总、打标签）

- [ ] 基于使用场景的智能自动化建议

贡献指南

欢迎所有人参与贡献！直接提交 Pull Request 即可。

代码风格

- Rust 代码（稳定版，工具链版本通过 rust-toolchain.toml 锁定）：遵循社区标准规范

相关链接

- Discord 社区：https://discord.gg/dZM7fUtE94

- X（原 Twitter）：https://x.com/nohrsdotapp
