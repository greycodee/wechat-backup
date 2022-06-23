## 数据库解析
- 通过消息id聚合消息
- 查询聊天微信昵称
- 查询聊天微信头像


## 快速解密微信DB
把要解密的微信 DB 所在文件夹挂在到容器的 `/wcdb` 上面。
```shell
$ docker run --rm -v /Users/zheng/Documents:/wcdb  greycodee/wcdb-sqlcipher -f DB名字 -k 解密密钥

2022/06/22 05:31:17 开始解密...
2022/06/22 05:31:28 解密成功: ok
2022/06/22 05:31:28 明文数据库文件名: EnMicroMsg_plain.db
```

## 快速转换微信语音 amr 文件
把要转换的语音文件夹挂载到容器的 `/media` 目录上，然后执行下面的命令，就会自动将文件夹里的语音转换成 `mp3` 格式了。
```shell
$ docker run --rm -v /Users/zheng/Documents/voice2:/media  greycodee/silkv3-decoder

/media/msg_491351061422dbfa9bb0a84104.amr
-e [OK] Convert /media/msg_491351061422dbfa9bb0a84104.amr To /media/msg_491351061422dbfa9bb0a84104.mp3 Finish.
```
