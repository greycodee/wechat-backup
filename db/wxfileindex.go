package db

import (
	"database/sql"
	"fmt"
	"log"
	"strings"
)

type WxFileIndex struct {
	DBPath    string
	db        *sql.DB
	tableName string
}

func OpenWxFileIndex(dbPath string) *WxFileIndex {
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		log.Fatal(err)
	}
	// 查询表名
	var tableName string
	querySql := "SELECT name _id FROM sqlite_master WHERE type ='table' limit 1"
	err = db.QueryRow(querySql).Scan(&tableName)
	if err != nil {
		log.Fatal(err)
	}
	return &WxFileIndex{dbPath, db, tableName}
}

func (wf *WxFileIndex) Close() {
	wf.db.Close()
}

func (wf WxFileIndex) GetImgPath(msgId string) string {
	var path string
	querySql := fmt.Sprintf("select path from %s WHERE msgId=%s and msgSubType=20", wf.tableName, msgId)
	err := wf.db.QueryRow(querySql).Scan(&path)
	if err != nil {
		log.Fatal(err)
	}
	return MediaPathPrefix + strings.Join(strings.SplitAfter(path, "/")[1:], "")
}

func (wf WxFileIndex) GetVideoPath(msgId string) string {
	var path string
	querySql := fmt.Sprintf("select path from %s WHERE msgId=%s and msgSubType=1", wf.tableName, msgId)
	err := wf.db.QueryRow(querySql).Scan(&path)
	if err != nil {
		log.Fatal(err)
	}
	return MediaPathPrefix + strings.Join(strings.SplitAfter(path, "/")[1:], "")
}

func (wf WxFileIndex) GetVoicePath(msgId string) string {
	var path string
	querySql := fmt.Sprintf("select path from %s WHERE msgId=%s", wf.tableName, msgId)
	err := wf.db.QueryRow(querySql).Scan(&path)
	if err != nil {
		log.Fatal(err)
	}
	return MediaPathPrefix + strings.Join(strings.SplitAfter(path, "/")[1:], "")
}
