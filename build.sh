#!/bin/bash

CGO_ENABLED=0 GOOS=darwin GOARCH=amd64 go build -o dist/wechat-backup-mac/wechat-backup .
CGO_ENABLED=0 GOOS=windows GOARCH=amd64 go build -o dist/wechat-backup-win/wechat-backup.exe .
CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o dist/wechat-backup-linux/wechat-backup .