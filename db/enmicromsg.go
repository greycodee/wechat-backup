package db

import (
	"database/sql"
	"fmt"
	"log"
	"strings"

	_ "github.com/mattn/go-sqlite3"
)

type EnMicroMsg struct {
	DBPath string
	db     *sql.DB
}

func OpenEnMicroMsg(dbPath string) *EnMicroMsg {
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		log.Fatal(err)
	}
	return &EnMicroMsg{dbPath, db}
}

func (em *EnMicroMsg) Close() {
	em.db.Close()
}

func (em EnMicroMsg) ChatList() string {
	// sql := "select count(*),msg.talker,rc.nickname,rc.conRemark,imf.reserved1,imf.reserved2 from message msg left join rcontact rc on msg.talker=rc.username  left join img_flag imf on msg.talker=imf.username group by msg.talker"
	sql := "select rc.nickname from message msg left join rcontact rc on msg.talker=rc.username  left join img_flag imf on msg.talker=imf.username group by msg.talker"
	rows, err := em.db.Query(sql)
	if err != nil {
		fmt.Println(err)
	}
	result := strings.Builder{}
	defer rows.Close()
	for rows.Next() {
		var nickname string
		err = rows.Scan(&nickname)
		if err != nil {
			log.Fatal(err)
		}
		fmt.Println(nickname)
		result.Write([]byte(nickname))
		result.Write([]byte("\n"))
	}
	err = rows.Err()
	if err != nil {
		log.Fatal(err)
	}

	return result.String()
}
