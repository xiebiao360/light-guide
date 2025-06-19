# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

Light Guide 是一个多环境运维管理工具，采用客户端-代理分离架构。客户端基于 Tauri 2.0 和 Vue 3 构建，代理端是纯 Rust 应用。通过 WebSocket 通信为管理物理机、Docker 容器和 Kubernetes 集群中的应用程序提供统一的仪表板。

## 技术栈

- **客户端**: Vue 3 + Vite + Tauri 2.0 (Rust)
- **代理端**: 纯 Rust 应用
- **共享代码**: Rust 类型定义和协议
- **包管理器**: pnpm (客户端), cargo (Rust)
- **通信协议**: WebSocket + Protobuf
- **安全**: TLS 1.3 + ED25519 签名

## 开发命令

### 项目级别命令

```bash
npm run dev              # 启动客户端开发服务器
npm run build:all        # 构建客户端和代理端
npm run build:client     # 仅构建客户端
npm run build:agent      # 仅构建代理端
npm run install:client   # 安装客户端依赖
npm run clean            # 清理所有构建产物
```

### 客户端开发

```bash
cd client
pnpm dev                 # 启动开发服务器 (Vue + Tauri)
pnpm build               # 构建前端生产版本
pnpm tauri dev           # 启动 Tauri 开发模式
pnpm tauri build         # 构建完整的 Tauri 应用程序
```

### 代理端开发

```bash
cd agent
cargo run                # 运行代理端
cargo build --release    # 构建代理端发布版本
cargo test               # 运行测试
```

## 系统架构

系统采用**分离式客户端-代理架构**：

### 客户端 (client/)

- **前端**: `client/src/` (Vue 3 前端)
- **后端**: `client/src-tauri/` (Tauri Rust 后端)
- **用途**: 统一仪表板和控制界面
- **组件**:
  - 实时指标仪表板
  - 终端模拟器
  - 应用管理界面
  - 实时日志查看器
  - 配置管理

### 代理端 (agent/)

- **位置**: `agent/src/` (纯 Rust 应用)
- **用途**: 部署在目标环境中执行操作
- **模块**:
  - `websocket/`: WebSocket 服务端通信
  - `system/`: 系统监控和指标收集
  - `apps/`: 应用管理和部署
  - `logs/`: 日志收集和实时传输
  - `packages/`: 软件包管理
  - `config/`: 配置文件管理

### 共享代码 (shared/)

- **类型定义**: `shared/types/` - 客户端和代理端共享的数据结构
- **协议定义**: `shared/proto/` - Protobuf 协议定义
- **工具函数**: `shared/utils/` - 共享的工具函数

## 项目结构

```text
light-guide/
├── README.md                    # 项目总体文档
├── CLAUDE.md                   # Claude Code 指导文档
├── package.json                # 项目级别的脚本和配置
├── shared/                     # 共享代码和协议定义
│   ├── proto/                  # Protobuf 协议定义
│   ├── types/                  # 共享类型定义
│   └── utils/                  # 共享工具函数
├── client/                     # 客户端 (Tauri 应用)
│   ├── package.json           # 前端依赖
│   ├── vite.config.js         # Vite 配置
│   ├── src/                   # Vue 前端源码
│   ├── src-tauri/            # Tauri Rust 后端
│   └── migrations/           # 数据库迁移
└── agent/                    # 代理端 (纯 Rust)
    ├── Cargo.toml           # Agent 依赖
    ├── src/                 # Rust 源码
    │   ├── websocket/       # WebSocket 服务端
    │   ├── system/          # 系统监控
    │   ├── apps/            # 应用管理
    │   ├── logs/            # 日志收集
    │   ├── packages/        # 包管理
    │   └── config/          # 配置管理
    ├── scripts/             # 部署脚本
    └── configs/             # 配置文件模板
```

## 关键配置文件

- `client/tauri.conf.json`: Tauri 应用配置
- `client/vite.config.js`: 前端构建配置
- `client/src-tauri/Cargo.toml`: 客户端 Rust 依赖
- `agent/Cargo.toml`: 代理端 Rust 依赖
- `client/package.json`: 前端依赖和脚本

## 开发说明

### 当前实现状态

- **已实现**:
  - 项目结构分离
  - 基础客户端 Tauri + Vue 模板
  - 代理端基础架构和模块划分
  - 共享类型定义
- **进行中**: 各个模块的具体实现
- **计划中**: WebSocket 通信协议、安全实现、完整功能

### 添加新的 Tauri 命令

1. 在 `client/src-tauri/src/lib.rs` 中定义命令
2. 添加到 `client/src-tauri/src/main.rs` 的 `tauri::Builder` 中
3. 在前端使用 `@tauri-apps/api` 调用

### 代理端模块开发

1. 在对应的 `agent/src/模块名/` 目录下实现功能
2. 在 `agent/src/lib.rs` 中导出模块
3. 在 `agent/src/main.rs` 中集成模块

### 共享类型定义

- 在 `shared/types/mod.rs` 中定义客户端和代理端共享的数据结构
- 使用 `serde` 进行序列化和反序列化
- 客户端和代理端都可以引用这些类型

### 数据库集成

- 客户端使用 SQLite，存储在 `client/migrations/`
- 代理端根据需要可选择本地存储方案

### 安全考虑

- 包验证使用 ED25519 签名
- 所有网络通信使用 TLS 1.3
- 基于角色的访问控制

## 通信协议

系统使用 WebSocket 连接进行客户端-代理通信：

- **客户端**: Tauri 后端作为 WebSocket 客户端连接代理
- **代理端**: 运行 WebSocket 服务端接受客户端连接
- **消息格式**: JSON 或 Protobuf 序列化
- **功能**: 实时系统指标流、日志跟踪、命令执行、状态更新

详细的系统设计、架构图和实现路线图请参考 README.md。
