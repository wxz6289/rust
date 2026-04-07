Tokio

## Tokio 相关技术栈

- HTTP/服务端框架: axum, warp, actix-web, hyper
- RPC/微服务: tonic (gRPC), tower
- WebSocket/实时通信: tokio-tungstenite, warp/axum 的 WS 支持
- 数据库/缓存客户端: sqlx, tokio-postgres, redis (redis-rs async)
- 消息队列/流处理: lapin (RabbitMQ), rdkafka (Kafka)
- 基础设施: tokio + tracing + tower + serde + config + anyhow/thiserror
- CLI/工具链: clap + tokio + reqwest
- 定时/任务: tokio-cron-scheduler, tokio-util