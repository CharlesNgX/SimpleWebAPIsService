# SimpleWebAPIsService

使用 Rust 构建的简单且高效的 Web 服务。

## 技术栈
- **框架**: Rocket (用于 Web APIs 快速开发)
- **本地环境**: MacOS M2
- **开发工具**: VSCode
- **远端服务器**: AWS EC2
- **操作系统**: Amazon Linux
- **项目架构**: Rocket + Serde + Diesel + Sqlite + Docker + DevOps
- **免费域名申请网址**: [CloudNS](https://www.cloudns.net/)
- **我的测试网址**: [rocketapp.cloudns.ch](https://rocketapp.cloudns.ch/)

## 测试效果🚀
其中 localhost 可修改成我的测试网址。

### 1) 创建 - CREATE
```
// CREATE
$ curl localhost:8000/rustaceans
-H "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=="
-H "Content-type: application/json"
-X POST -d '{"name" : "Charles", "email" : "Charles.Ng.X@gmail.com"}'
```

### 2) 查询 - GET ALL
```
// GET ALL
$ curl localhost:8000/rustaceans
-H "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=="
-H "Content-type: application/json"
```

### 3) 更新 - UPDATE
```
// UPDATE
$ curl localhost:8000/rustaceans/1
-H "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=="
-H "Content-type: application/json"
-X PUT -d '{"id" : 1, "name" : "Ch", "email" : "Charles.Ng.X@gmail.com"}'
```

### 4) 删除 - DELETE
```
// DELETE
$ curl localhost:8000/rustaceans/1
-H "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=="
-H "Content-type: application/json"
-X DELETE
```

## Amazon Linux 操作篇
1. 需要配置 Rust、Diesel、Sqlite、Nginx、CertBot 等。
2. 配置 `rocket-app.service`:
```
$ sudo vim /etc/systemd/system/rocket-app.service
$ sudo systemctl start rocket-app.service
$ sudo systemctl enable rocket-app.service
$ sudo systemctl status rocket-app.service
```
3. 开启 Rocket-App：
```
$ cd /var/www/rocket-app
$ ROCKET_DATABASES={sqlite_path={url./database.sqlite}} ./rocket-app
```

## Diesel
- Diesel 官方指南：[Getting Started](https://diesel.rs/guides/getting-started)
- 常用命令：
```
$ cargo install diesel_cli --no-default-features --features sqlite
$ diesel setup --database-url=database.sqlite
$ diesel migration generate create_rustaceans
$ diesel migration run --database-url=database.sqlite
$ diesel migration redo --database-url=database.sqlite
```

## Tips💡
1. Dockerfile 需要根据你的操作系统进行相应的修改，当然你的操作系统环境也需要配置 Rust、Diesel、Sqlite 等。
2. Amazon Linux 与其他操作系统下载 Nginx 有所不同，因此在 `/devops/nginx.conf` 中修改了服务器部分的代码。
3. 代码提交时使用 [gitmoji](https://gitmoji.dev/)规范
4. 遵守 GitHub Action 工作流规范

## On More Thing🌹
如果觉得这个项目有帮助，欢迎给个星星✨，感谢您的支持！













