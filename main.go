package main

import (
	"database/sql"
	"fmt"
	"log"

	_ "github.com/mattn/go-sqlite3"
)

func main() {
	db, err := sql.Open("sqlite3", "/mnt/c/Users/zheng/Desktop/wxfileindex_plaintext.db")
	if err != nil {
		fmt.Println(err)
	}
	defer db.Close()

	rows, err := db.Query("select username from WxFileIndex2 where msgId=9127")
	if err != nil {
		log.Fatal(err)
	}
	defer rows.Close()
	for rows.Next() {
		var username string
		err = rows.Scan(&username)
		if err != nil {
			log.Fatal(err)
		}
		fmt.Println(username)
	}
	err = rows.Err()
	if err != nil {
		log.Fatal(err)
	}

	// db.Query("SELECT * FROM wxfileindex")
}
