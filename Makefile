export GO111MODULE=on

all: wechat-backup

LDFLAGS = -s -w
ifdef STATIC
        LDFLAGS += -linkmode external -extldflags '-static'
        CC = /usr/bin/musl-gcc
        export CC
endif

wechat-backup:
        go build -ldflags="$(LDFLAGS)"  -o wechat-backup .
       
wechat-backup.linux:
        CGO_ENABLED=1 GOOS=linux GOARCH=amd64 CC=x86_64-linux-musl-gcc CGO_LDFLAGS="-static" go build -ldflags="$(LDFLAGS)"  -o wechat-backup .
# Please execute the `apt install gcc-arm-linux-gnueabihf` command before using it. 
wechat-backup.linux-arm:
        CGO_ENABLED=1 GOOS=linux GOARCH=arm CC=arm-linux-gnueabihf-gcc CGO_LDFLAGS="-static" go build -ldflags="$(LDFLAGS)"  -o wechat-backup .
# Please execute the `apt install gcc-aarch64-linux-gnu` command before using it. 
wechat-backup.linux-arm64:
        CGO_ENABLED=1 GOOS=linux GOARCH=arm CC=arm-linux-gnueabihf-gcc CGO_LDFLAGS="-static" go build -ldflags="$(LDFLAGS)"  -o wechat-backup .
# Please execute the `apt install gcc-mingw-w64-x86-64` command before using it. 
wechat-backup.windows:
        CGO_ENABLED=1 GOOS=windows GOARCH=amd64 CC=x86_64-w64-mingw32-gcc go build -ldflags="$(LDFLAGS)" -buildmode exe -o wechat-backup.exe .
