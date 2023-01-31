package api

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/greycodee/wechat-backup/db"
)

var wcdb *db.WCDB

const (
	ListApi     = "/api/chat/list"
	DetailApi   = "/api/chat/detail"
	UserInfoApi = "/api/user/info"
	MyInfoApi   = "/api/user/myinfo"
	ImgApi      = "/api/media/img"
	VideoApi    = "/api/media/video"
	VoiceApi    = "/api/media/voice"
)

func ListHandler(c *gin.Context) {
	pageIndex, _ := strconv.Atoi(c.DefaultQuery("pageIndex", "1"))
	pageSize, _ := strconv.Atoi(c.DefaultQuery("pageSize", "10"))
	name := c.Query("name")
	all, _ := strconv.ParseBool(c.Query("all"))

	// 聊天列表
	c.JSON(200, wcdb.ChatList(pageIndex-1, pageSize, all, name))
}

func ApiRouter() http.Handler {
	g := gin.New()
	g.Use(gin.Recovery())
	g.GET(ListApi, ListHandler)
	return g
}
