package db

import (
	"database/sql"
	"fmt"
	"log"

	_ "github.com/mattn/go-sqlite3"
)

type EnMicroMsg struct {
	DBPath string
	db     *sql.DB
}

func OpenEnMicroMsg(dbPath string) *EnMicroMsg {
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		log.Fatalf("open db error: %v", err)
	}
	return &EnMicroMsg{dbPath, db}
}

func (em *EnMicroMsg) Close() {
	em.db.Close()
}

func (em EnMicroMsg) ChatList(pageIndex int, pageSize int) *ChatList {
	result := &ChatList{}
	result.Total = 10
	result.Rows = make([]ChatListRow, 0)
	// sql := "select count(*),msg.talker,rc.nickname,rc.conRemark,imf.reserved1,imf.reserved2 from message msg left join rcontact rc on msg.talker=rc.username  left join img_flag imf on msg.talker=imf.username group by msg.talker"
	queryRowsSql := fmt.Sprintf("select count(*) as msgCount,msg.talker,ifnull(rc.nickname,'') as nickname,ifnull(rc.conRemark,'') as conRemark,ifnull(imf.reserved1,'') as reserved1,ifnull(imf.reserved2,'') as reserved2,msg.createtime from message msg left join rcontact rc on msg.talker=rc.username  left join img_flag imf on msg.talker=imf.username group by msg.talker order by msg.createTime desc limit %d,%d", pageIndex*pageSize, pageSize)
	rows, err := em.db.Query(queryRowsSql)
	if err != nil {
		fmt.Println(err)
	}
	defer rows.Close()
	for rows.Next() {
		var r ChatListRow
		err = rows.Scan(&r.MsgCount, &r.Talker, &r.NickName, &r.ConRemark, &r.Reserved1, &r.Reserved2, &r.CreateTime)
		if err != nil {
			log.Fatal(err)
		}
		result.Rows = append(result.Rows, r)
	}
	err = rows.Err()
	if err != nil {
		log.Fatal(err)
	}

	queryTotalSql := "select count(*) as total FROM (select msg.talker from message msg left join rcontact rc on msg.talker=rc.username  left join img_flag imf on msg.talker=imf.username group by msg.talker) as d"
	totalRows, err := em.db.Query(queryTotalSql)
	if err != nil {
		fmt.Println(err)
	}
	defer totalRows.Close()
	for totalRows.Next() {
		var total int
		err = totalRows.Scan(&total)
		if err != nil {
			log.Fatal(err)
		}
		result.Total = total
	}
	err = totalRows.Err()
	if err != nil {
		log.Fatal(err)
	}

	return result
}

func (em EnMicroMsg) ChatDetailList(talker string, pageIndex int, pageSize int) *ChatDetailList {
	result := &ChatDetailList{}
	result.Total = 10
	result.Rows = make([]ChatDetailListRow, 0)
	queryRowsSql := fmt.Sprintf("SELECT msgId,msgSvrId,type,isSend,createTime,talker,content FROM message WHERE talker='%s' order by createtime desc limit %d,%d", talker, pageIndex*pageSize, pageSize)
	rows, err := em.db.Query(queryRowsSql)
	if err != nil {
		fmt.Println(err)
	}
	defer rows.Close()
	for rows.Next() {
		var r ChatDetailListRow
		err = rows.Scan(&r.MsgId, &r.MsgSvrId, &r.Type, &r.IsSend, &r.CreateTime, &r.Talker, &r.Content)
		if err != nil {
			log.Fatal(err)
		}
		result.Rows = append(result.Rows, r)
	}
	return result
}

func (em EnMicroMsg) UserInfo(username string) UserInfo {
	r := UserInfo{}
	querySql := fmt.Sprintf("select rc.username,rc.alias,rc.conRemark,rc.nickname,ifnull(imf.reserved1,'') as reserved1,ifnull(imf.reserved2,'') as reserved2 from rcontact rc LEFT JOIN img_flag imf on rc.username=imf.username where rc.username='%s';", username)
	rows, err := em.db.Query(querySql)
	if err != nil {
		fmt.Println(err)
	}
	defer rows.Close()
	for rows.Next() {
		err = rows.Scan(&r.UserName, &r.Alias, &r.ConRemark, &r.NickName, &r.Reserved1, &r.Reserved2)
		if err != nil {
			log.Fatal(err)
		}
	}
	return r
}
