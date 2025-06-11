# Cloudflare IP 优选工具

![Rust](https://img.shields.io/badge/Rust-1.75+-blue) ![License](https://img.shields.io/badge/License-MIT-green)

一个基于 Rust 的 Cloudflare IP 优选工具，提供 HTTP API 和 DNS 管理功能。

## 功能特性

- ✅ Cloudflare IP 批量测试与优选
- ✅ DNS 记录自动同步到 Namesilo
- ✅ RESTful API 接口
- ✅ 实时任务状态查询
- ✅ 静态资源服务

## 快速开始

### 前置要求

- Rust 1.75+
- CloudflareSpeedTest 工具
- Namesilo API 密钥(可选)

### 安装运行

```bash
# 克隆项目
git clone https://github.com/your-repo/cfselect.git
cd cfselect

# 安装依赖
cargo build --release

# 运行服务
APP__NAMESILO__KEY=xxxxxx cargo run

# docker-compose 运行服务
docker-compose up
```

## 配置说明

编辑`config.toml`文件：

```toml
[listen]
host = "0.0.0.0"
port = 8080

[log]
# 日志配置...

[namesilo]
# API配置...
```

## API 文档

### IP 优选

- `POST /api/ip/select` - 启动 IP 优选
- `GET /api/ip/select` - 获取优选结果
- `GET /api/ip/select/status` - 查询任务状态

### DNS 管理

- `POST /api/dns/sync` - 同步 DNS 记录

### 健康检查

- `GET /api/server/health_check` - 服务健康状态

## 项目结构

```
src/
├── api/          # API处理逻辑
├── client/       # 外部服务客户端
├── configure/    # 配置管理
├── model/        # 数据模型
├── router/       # 路由定义
└── server/       # 服务核心
```

## 许可证

MIT License
