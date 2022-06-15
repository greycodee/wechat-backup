package db

import (
	"database/sql"
	"log"
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
