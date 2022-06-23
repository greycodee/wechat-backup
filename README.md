## 效果图
![](./web.png)

## 使用流程
> 详细说明在: https://greycode.top/posts/android-wechat-bak/

1. 手机聊天记录备份到电脑，在有 ROOT 权限的手机上登陆微信，电脑点击备份恢复，把聊天记录恢复到有 ROOT 的手机上。（如果没有有 ROOT 权限的手机，建议可以用安卓模拟器）
2. 收集下面这些数据，然后放在**同一个文件夹下**:
    - image2 文件夹：里面存放着所有的微信聊天图片，位置在：/data/data/com.tencent.mm/MicroMsg/[32位字母]/image2
    - voice2 文件夹：里面存放着所有的微信语音，位置在：/sdcard/Android/data/com.tencent.mm/MicroMsg/[32位字母]/voice2
    - voide 文件夹：里面存放着所有的微信视频，位置在：/sdcard/Android/data/com.tencent.mm/MicroMsg/[32位字母]/voide
    - avatar 文件夹：里面存放着所有的微信头像，位置在：/data/data/com.tencent.mm/MicroMsg/[32位字母]/avatar
    - Download 文件夹: 微信的聊天发送的文件存放在这里，位置在：/sdcard/Android/data/com.tencent.mm/MicroMsg/Download
    - EnMicroMsg.db: 微信的数据库文件，位置在：/data/data/com.tencent.mm/MicroMsg/[32位字母]/EnMicroMsg.db
    - WxFileIndex.db: 微信的文件索引数据库文件，位置在：/data/data/com.tencent.mm/MicroMsg/[32位字母]/WxFileIndex.db
3. 获取解密 DB 的密钥。
4. 进行微信聊天数据 DB 的解密
5. 转换微信语音
6. 运行本程序，打开控制台输出的网址，就可以查看你的聊天记录了。
> 运行时，记得在 main.go 里指定你存放上面这些文件的目录地址

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
