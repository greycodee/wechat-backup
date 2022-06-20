package db

type ChatList struct {
	Total int           `json:"total"`
	Rows  []ChatListRow `json:"rows"`
}

type ChatListRow struct {
	Talker      string `json:"talker"`
	NickName    string `json:"nickname"`
	ConRemark   string `json:"conRemark"`
	Reserved1   string `json:"reserved1"`
	Reserved2   string `json:"reserved2"`
	LocalAvatar string `json:"localAvatar"`
	MsgCount    int    `json:"msgCount"`
	CreateTime  int64  `json:"createTime"`
	ChatType    int    `json:"chatType"`
}

type ChatDetailList struct {
	Total int                 `json:"total"`
	Rows  []ChatDetailListRow `json:"rows"`
}
type ChatDetailListRow struct {
	MsgId        string `json:"msgId"`
	MsgSvrId     string `json:"msgSvrId"`
	Type         int    `json:"type"`
	IsSend       int    `json:"isSend"`
	CreateTime   int64  `json:"createTime"`
	Talker       string `json:"talker"`
	Content      string `json:"content"`
	ImgPath      string `json:"imgPath"`
	MediaPath    string `json:"mediaPath"`
	MediaBCKPath string `json:"mediaBCKPath"`
}

type UserInfo struct {
	UserName    string `json:"userName"`
	Alias       string `json:"alias"`
	ConRemark   string `json:"conRemark"`
	NickName    string `json:"nickName"`
	Reserved1   string `json:"reserved1"`
	Reserved2   string `json:"reserved2"`
	LocalAvatar string `json:"localAvatar"`
}
