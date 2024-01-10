# SimpleWebAPIsService
用Rust搭建简单快速的Web服务：
服务器：AWS EC2
操作系统：Amazon Linux
项目架构：Rocket+Serde+Diesel+Sqlite+Docker+DevOps
免费域名申请网址：https://www.cloudns.net/
测试网址：https://rocketapp.cloudns.ch/

测试效果：
1）增：
// CREATE
$ curl localhost:8000/rustaceans \
  -H "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==" \
  -H "Content-type: application/json" \
  -X POST -d '{"name" : "Charles", "email" : "CharlesEngX@gmail.com"}' 

2) 查：
// GET ALL
$ curl localhost:8000/rustaceans \
  -H "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==" \
  -H "Content-type: application/json"

3) 改：
// UPDATE
$ curl localhost:8000/rustaceans/1 \
  -H "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==" \
  -H "Content-type: application/json" \
  -X PUT -d '{"id" : 1, "name" : "Ch", "email" : "john.doe@gmail.com"}'


Amazon Linux操作篇：
1）需要配置Rust、Diesel、Sqlite、Nginx、CertBot等
2）配置rocket-app.service:
$ sudo vim /etc/systemd/system/rocket-app.service
$ sudo systemctl start  rocket-app.service
$ sudo systemctl enable rocket-app.service
$ sudo systemctl status rocket-app.service



Tips:
1）Dockerfile需要改成符合你的操作系统的命令，当然你的操作系统环境也需要配置Rust、Diesel、Sqlite等
2）由于Amazon Linux与其他的操作系统下载nginx会有所不同，所以在/devops/nginx.conf修改了服务器部分的代码

