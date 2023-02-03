package db

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
