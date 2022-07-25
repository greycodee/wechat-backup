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
		log.Printf("未查询到 WxFileIndex.db 文件,%s", err)
	}
	// 查询表名
	var tableName string
	querySql := "SELECT name _id FROM sqlite_master WHERE type ='table' limit 1"
	err = db.QueryRow(querySql).Scan(&tableName)
	if err != nil {
		log.Printf("未查询到图片索引表名,%s", err)
		// log.Fatal(err)
	} else {
		log.Printf("文件索引表名: %s", tableName)
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
		log.Printf("未查询到图片,%s", err)
		return ""
	} else {
		return MediaPathPrefix + strings.Join(strings.SplitAfter(path, "/")[1:], "")
	}

}

func (wf WxFileIndex) GetVideoPath(msgId string) string {
	var path string
	querySql := fmt.Sprintf("select path from %s WHERE msgId=%s and msgSubType=1", wf.tableName, msgId)
	err := wf.db.QueryRow(querySql).Scan(&path)
	if err != nil {
		log.Printf("未查询到视频,%s", err)
		return ""
	} else {
		return MediaPathPrefix + strings.Join(strings.SplitAfter(path, "/")[1:], "")
	}
}

func (wf WxFileIndex) GetVoicePath(msgId string) string {
	var path string
	querySql := fmt.Sprintf("select path from %s WHERE msgId=%s", wf.tableName, msgId)
	err := wf.db.QueryRow(querySql).Scan(&path)
	if err != nil {
		log.Printf("未查询到语音,%s", err)
		return ""
	} else {
		return MediaPathPrefix + strings.Join(strings.SplitAfter(path, "/")[1:], "")
	}
}

func (wf WxFileIndex) GetFilePath(msgId string) (path string, size int64) {
	querySql := fmt.Sprintf("select path,size from %s WHERE msgId=%s", wf.tableName, msgId)
	err := wf.db.QueryRow(querySql).Scan(&path, &size)
	if err != nil {
		log.Printf("未查询到文件,%s", err)
		return "", 0
	} else {
		return MediaPathPrefix + path, size
	}
}
