package db

import (
	"database/sql"
	"fmt"
	"log"
	"strings"
)

type WxFileIndex struct {
	DBPath string
	db     *sql.DB
}

func OpenWxFileIndex(dbPath string) *WxFileIndex {
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		log.Fatal(err)
	}
	return &WxFileIndex{dbPath, db}
}

func (wf *WxFileIndex) Close() {
	wf.db.Close()
}

func (wf WxFileIndex) GetImgPath(msgId string) string {
	var path string
	querySql := fmt.Sprintf("select path from WxFileIndex2 WHERE msgId=%s and msgSubType=20", msgId)
	err := wf.db.QueryRow(querySql).Scan(&path)
	if err != nil {
		log.Fatal(err)
	}
	return MediaPathPrefix + strings.Join(strings.SplitAfter(path, "/")[1:], "")
}

func (wf WxFileIndex) GetVideoPath(msgId string) string {
	var path string
	querySql := fmt.Sprintf("select path from WxFileIndex2 WHERE msgId=%s and msgSubType=1", msgId)
	err := wf.db.QueryRow(querySql).Scan(&path)
	if err != nil {
		log.Fatal(err)
	}
	return MediaPathPrefix + strings.Join(strings.SplitAfter(path, "/")[1:], "")
}

func (wf WxFileIndex) GetVoicePath(msgId string) string {
	var path string
	querySql := fmt.Sprintf("select path from WxFileIndex2 WHERE msgId=%s", msgId)
	err := wf.db.QueryRow(querySql).Scan(&path)
	if err != nil {
		log.Fatal(err)
	}
	return MediaPathPrefix + strings.Join(strings.SplitAfter(path, "/")[1:], "")
}
