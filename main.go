package main

import (
	"database/sql"
	"fmt"
	"log"

	_ "github.com/mattn/go-sqlite3"
)

func main() {
	db, err := sql.Open("sqlite3", "/Users/zheng/Documents/wcdb/wxfileindex_plaintext.db")
	if err != nil {
		fmt.Println(err)
	}
	defer db.Close()

	db2, err := sql.Open("sqlite3", "/Users/zheng/Documents/wcdb/enmicromsg_plaintext.db")
	if err != nil {
		fmt.Println(err)
	}
	defer db2.Close()
	rows, err := db2.Query("select content from message where msgId=9127")
	if err != nil {
		log.Fatal(err)
	}
	defer rows.Close()
	for rows.Next() {
		var content string
		err = rows.Scan(&content)
		if err != nil {
			log.Fatal(err)
		}
		fmt.Println(content)
	}
	err = rows.Err()
	if err != nil {
		log.Fatal(err)
	}

	rows2, err := db.Query("select username from WxFileIndex2 where msgId=9127")
	if err != nil {
		log.Fatal(err)
	}
	defer rows2.Close()
	for rows2.Next() {
		var username string
		err = rows2.Scan(&username)
		if err != nil {
			log.Fatal(err)
		}
		fmt.Println(username)
	}
	err = rows2.Err()
	if err != nil {
		log.Fatal(err)
	}

	// db.Query("SELECT * FROM wxfileindex")
}
