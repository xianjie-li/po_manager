# PO_MANAGER

自用的简单项目管理工具, 用于简化工作流, 本仓库仅包含后端服务代码

设计: 
- 使用 axum 框架, 项目基本上只会在本机运行, 数据直接以json形式存储就可满足, 这样还可以顺便熟悉一些多线程下的数据竞争处理
- 杂项: serde anyhow chrono tokio uuid
- 前端: 采用 flutter , 同样处于练习目的