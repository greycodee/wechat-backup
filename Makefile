export GO111MODULE=on
binary_name=wechat-backup

all:  ${binary_name}.linux ${binary_name}.windows ${binary_name}.linux-arm64
LDFLAGS = -s -w
ifdef STATIC
        LDFLAGS += -linkmode external -extldflags '-static'
        CC = /usr/bin/musl-gcc
        export CC
endif

$(binary_name):
	go build -ldflags="$(LDFLAGS)"  -o $(binary_name) .
       
$(binary_name).linux:
	CGO_ENABLED=1 GOOS=linux GOARCH=amd64 CGO_LDFLAGS="-static" go build -ldflags="$(LDFLAGS)"  -o dist/linux/$(binary_name) .
# Please execute the `apt install gcc-aarch64-linux-gnu` command before using it.
$(binary_name).linux-arm64:
	CGO_ENABLED=1 GOOS=linux GOARCH=arm64 CC=aarch64-linux-gnu-gcc CGO_LDFLAGS="-static" go build -ldflags="$(LDFLAGS)"  -o dist/linux-arm64/$(binary_name) .
# Please execute the `apt install gcc-mingw-w64-x86-64` command before using it. 
$(binary_name).windows:
	CGO_ENABLED=1 GOOS=windows GOARCH=amd64 CC=x86_64-w64-mingw32-gcc go build -ldflags="$(LDFLAGS)" -buildmode exe -o dist/windows/$(binary_name).exe .
