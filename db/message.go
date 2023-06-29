package db

import (
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

type Message struct {
	MsgId             int    `gorm:"column:msgId;primary_key" json:"msgId"`
	MsgSvrId          int    `gorm:"column:msgSvrId" json:"msgSvrId"`
	Type              int    `gorm:"column:type" json:"type"`
	Status            int    `gorm:"column:status" json:"status"`
	IsSend            int    `gorm:"column:isSend" json:"isSend"`
	IsShowTimer       int    `gorm:"column:isShowTimer" json:"isShowTimer"`
	CreateTime        int    `gorm:"column:createTime" json:"createTime"`
	Talker            string `gorm:"column:talker" json:"talker"`
	Content           string `gorm:"column:content" json:"content"`
	ImgPath           string `gorm:"column:imgPath" json:"imgPath"`
	Reserved          string `gorm:"column:reserved" json:"reserved"`
	Lvbuffer          string `gorm:"column:lvbuffer" json:"lvbuffer"`
	TransContent      string `gorm:"column:transContent" json:"transContent"`
	TransBrandWording string `gorm:"column:transBrandWording" json:"transBrandWording"`
	TalkerId          int    `gorm:"column:talkerId" json:"talkerId"`
	BizClientMsgId    string `gorm:"column:bizClientMsgId" json:"bizClientMsgId"`
	BizChatId         int    `gorm:"column:bizChatId" json:"bizChatId"`
	BizChatUserId     string `gorm:"column:bizChatUserId" json:"bizChatUserId"`
	MsgSeq            int    `gorm:"column:msgSeq" json:"msgSeq"`
	Flag              int    `gorm:"column:flag" json:"flag"`
	SolitaireFoldInfo string `gorm:"column:solitaireFoldInfo" json:"solitaireFoldInfo"`
	HistoryId         string `gorm:"column:historyId" json:"historyId"`
}

func (Message) TableName() string {
	return "message"
}

type Rcontact struct {
	Username         string `gorm:"column:username;primary_key" json:"username"`
	Alias            string `gorm:"column:alias" json:"alias"`
	ConRemark        string `gorm:"column:conRemark" json:"conRemark"`
	DomainList       string `gorm:"column:domainList" json:"domainList"`
	Nickname         string `gorm:"column:nickname" json:"nickname"`
	PyInitial        string `gorm:"column:pyInitial" json:"pyInitial"`
	QuanPin          string `gorm:"column:quanPin" json:"quanPin"`
	ShowHead         int    `gorm:"column:showHead;default:0" json:"showHead"`
	Type             int    `gorm:"column:type;default:0" json:"type"`
	UiType           int    `gorm:"column:uiType;default:0" json:"uiType"`
	WeiboFlag        int    `gorm:"column:weiboFlag;default:0" json:"weiboFlag"`
	WeiboNickname    string `gorm:"column:weiboNickname" json:"weiboNickname"`
	ConRemarkPYFull  string `gorm:"column:conRemarkPYFull" json:"conRemarkPYFull"`
	ConRemarkPYShort string `gorm:"column:conRemarkPYShort" json:"conRemarkPYShort"`
	Lvbuff           string `gorm:"column:lvbuff" json:"lvbuff"`
	VerifyFlag       int    `gorm:"column:verifyFlag;default:0" json:"verifyFlag"`
	EncryptUsername  string `gorm:"column:encryptUsername" json:"encryptUsername"`
	ChatroomFlag     int    `gorm:"column:chatroomFlag" json:"chatroomFlag"`
	DeleteFlag       int    `gorm:"column:deleteFlag;default:0" json:"deleteFlag"`
	ContactLabelIds  string `gorm:"column:contactLabelIds" json:"contactLabelIds"`
	DescWordingId    string `gorm:"column:descWordingId" json:"descWordingId"`
	OpenImAppid      string `gorm:"column:openImAppid" json:"openImAppid"`
	SourceExtInfo    string `gorm:"column:sourceExtInfo" json:"sourceExtInfo"`
	Ticket           string `gorm:"column:ticket" json:"ticket"`
	UsernameFlag     int    `gorm:"column:usernameFlag;default:0" json:"usernameFlag"`
}

func (Rcontact) TableName() string {
	return "rcontact"
}

type ChatListResult struct {
	MsgCount   int64  `gorm:"column:msgCount"`
	Alias      string `gorm:"column:alias"`
	Talker     string `gorm:"column:talker"`
	NickName   string `gorm:"column:nickname"`
	ConRemark  string `gorm:"column:conRemark"`
	Reserved1  string `gorm:"column:reserved1"`
	Reserved2  string `gorm:"column:reserved2"`
	CreateTime int64  `gorm:"column:createTime"`
}

func GetChatList() (list []ChatListResult, err error) {
	db, err := gorm.Open(sqlite.Open("/home/zheng/coding/wechatbak/dest/79b23ef49a3016d8c52a787fc4ab59e4/EnMicroMsg_plain.db"), &gorm.Config{})
	if err != nil {
		return nil, err
	}

	db.Table("message msg").
		Select("count(*) as msgCount,rc.alias,msg.talker,rc.nickname,rc.conRemark,imf.reserved1,imf.reserved2,msg.createTime").
		Joins("left join rcontact rc on msg.talker=rc.username").
		Joins("left join img_flag imf on msg.talker=imf.username").
		Group("msg.talker").
		Order("msg.createTime desc").
		Limit(5).Scan(&list)

	return
}
