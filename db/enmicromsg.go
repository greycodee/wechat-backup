package db

import (
	"crypto/md5"
	"database/sql"
	"fmt"
	"log"
	"strings"

	_ "github.com/mattn/go-sqlite3"
)

var MediaPathPrefix = "/media/"

type EnMicroMsg struct {
	db     *sql.DB
	myInfo UserInfo
}

func OpenEnMicroMsg(dbPath string) *EnMicroMsg {
	em := &EnMicroMsg{}
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		log.Fatalf("open db error: %v", err)
	}
	em.db = db
	em.myInfo = em.GetMyInfo()
	return em
}

func (em *EnMicroMsg) Close() {
	em.db.Close()
}

func (em EnMicroMsg) ChatList(pageIndex int, pageSize int) *ChatList {
	result := &ChatList{}
	result.Total = 10
	result.Rows = make([]ChatListRow, 0)
	queryRowsSql := fmt.Sprintf("select count(*) as msgCount,msg.talker,ifnull(rc.nickname,'') as nickname,ifnull(rc.conRemark,'') as conRemark,ifnull(imf.reserved1,'') as reserved1,ifnull(imf.reserved2,'') as reserved2,msg.createtime from message msg left join rcontact rc on msg.talker=rc.username  left join img_flag imf on msg.talker=imf.username group by msg.talker order by msg.createTime desc limit %d,%d", pageIndex*pageSize, pageSize)
	rows, err := em.db.Query(queryRowsSql)
	if err != nil {
		fmt.Println(err)
	}
	defer rows.Close()
	for rows.Next() {
		var r ChatListRow
		err = rows.Scan(&r.MsgCount, &r.Talker, &r.NickName, &r.ConRemark, &r.Reserved1, &r.Reserved2, &r.CreateTime)
		// 判断是否是群聊
		if len(strings.Split(r.Talker, "@")) == 2 && strings.Split(r.Talker, "@")[1] == "chatroom" {
			if r.NickName == "" {
				queryRoomSql := fmt.Sprintf("select displayname as nickname from chatroom where chatroomname='%s'", r.Talker)
				room, _ := em.db.Query(queryRoomSql)
				defer room.Close()
				for room.Next() {
					room.Scan(&r.NickName)
				}
			}
		}
		if err != nil {
			log.Fatal(err)
		}
		r.LocalAvatar = em.getLocalAvatar(r.Talker)
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
	queryRowsSql := fmt.Sprintf("SELECT msgId,msgSvrId,type,isSend,createTime,talker,content,ifnull(imgPath,'') as imgPath FROM message WHERE talker='%s' order by createtime desc limit %d,%d", talker, pageIndex*pageSize, pageSize)
	rows, err := em.db.Query(queryRowsSql)
	if err != nil {
		fmt.Println(err)
	}
	defer rows.Close()
	for rows.Next() {
		var r ChatDetailListRow
		err = rows.Scan(&r.MsgId, &r.MsgSvrId, &r.Type, &r.IsSend, &r.CreateTime, &r.Talker, &r.Content, &r.ImgPath)
		if err != nil {
			log.Fatal(err)
		}
		em.getMediaPath(&r)
		result.Rows = append(result.Rows, r)
	}
	return result
}

func (em EnMicroMsg) GetUserInfo(username string) UserInfo {
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
	r.LocalAvatar = em.getLocalAvatar(r.UserName)
	return r
}

func (em EnMicroMsg) GetMyInfo() UserInfo {
	r := UserInfo{}
	querySql := "select rc.username,rc.alias,rc.conRemark,rc.nickname,ifnull(imf.reserved1,'') as reserved1,ifnull(imf.reserved2,'') as reserved2 from rcontact rc left join img_flag imf on rc.username=imf.username where rc.username=(select value from userinfo WHERE id = 2)"
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
	r.LocalAvatar = em.getLocalAvatar(r.UserName)
	return r
}

func (em EnMicroMsg) getLocalAvatar(username string) string {
	md5Str := fmt.Sprintf("%x", md5.Sum([]byte(username)))
	filePath := fmt.Sprintf("%savatar/%s/%s/user_%s.png", MediaPathPrefix, md5Str[0:2], md5Str[2:4], md5Str)
	return filePath
}

func (em EnMicroMsg) formatImagePath(path string) string {
	imgFileName := strings.Split(path, "://")[1]
	return fmt.Sprintf("%simage2/%s/%s/%s", MediaPathPrefix, imgFileName[3:5], imgFileName[5:7], imgFileName)
}
func (em EnMicroMsg) formatImageBCKPath(chat ChatDetailListRow) string {
	var imgFileName string
	if chat.IsSend == 0 {
		// 接收
		imgFileName = fmt.Sprintf("%s_%s_%s_backup", chat.Talker, em.myInfo.UserName, chat.MsgSvrId)
	} else {
		// 发送
		imgFileName = fmt.Sprintf("%s_%s_%s_backup", em.myInfo.UserName, chat.Talker, chat.MsgSvrId)
	}
	return fmt.Sprintf("%simage2/%s/%s/%s", MediaPathPrefix, imgFileName[0:2], imgFileName[2:4], imgFileName)
}
func (em EnMicroMsg) formatVoicePath(path string) string {
	p := md5.Sum([]byte(path))
	md5Str := fmt.Sprintf("%x", p)
	// 这边原本后缀为amr格式，由于网页不能播放amr格式，所以要用转换工具转换格式，转换后为mp3格式，所以后缀为mp3
	return fmt.Sprintf("%svoice2/%s/%s/msg_%s.mp3", MediaPathPrefix, md5Str[0:2], md5Str[2:4], path)
}
func (em EnMicroMsg) formatVideoPath(path string) string {
	return fmt.Sprintf("%svideo/%s.mp4", MediaPathPrefix, path)
}

func (em EnMicroMsg) getMediaPath(chat *ChatDetailListRow) {
	switch chat.Type {
	case 3:
		// 图片
		chat.MediaPath = em.formatImagePath(chat.ImgPath)
		chat.MediaBCKPath = em.formatImageBCKPath(*chat)
		break
	case 34:
		// 语音
		chat.MediaPath = em.formatVoicePath(chat.ImgPath)
		break
	case 43:
		// 视频
		chat.MediaPath = em.formatVideoPath(chat.ImgPath)
		break
	default:
		break
	}
}
