## 数据库解析
- 通过消息id聚合消息
- 查询聊天微信昵称
- 查询聊天微信头像


## 快速解密微信DB
把要解密的微信 DB 所在文件夹挂在到容器的 `/wcdb` 上面。
```shell
$ docker run --rm -v /Users/zheng/coding/study:/wcdb  greycodee/wcdb-sqlcipher:1.0 -f DB名字 -k 解密密钥

2022/06/22 05:31:17 开始解密...
2022/06/22 05:31:28 解密成功: ok
2022/06/22 05:31:28 明文数据库文件名: EnMicroMsg_plain.db
```
