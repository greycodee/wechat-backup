package api

import (
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/greycodee/wechat-backup/db"
)

const (
	ListApi     = "/api/chat/list"
	DetailApi   = "/api/chat/detail"
	UserInfoApi = "/api/user/info"
	MyInfoApi   = "/api/user/myinfo"
	ImgApi      = "/api/media/img"
	VideoApi    = "/api/media/video"
	VoiceApi    = "/api/media/voice"
)

type Api struct {
	wcdb *db.WCDB
}

func New(dbPath string) *Api {
	a := &Api{}
	a.wcdb = db.InitWCDB(dbPath)
	return a
}

func (a Api) listHandler(c *gin.Context) {
	pageIndex, _ := strconv.Atoi(c.DefaultQuery("pageIndex", "1"))
	pageSize, _ := strconv.Atoi(c.DefaultQuery("pageSize", "10"))
	name := c.Query("name")
	all, _ := strconv.ParseBool(c.DefaultQuery("all", "false"))

	result := a.wcdb.ChatList(pageIndex-1, pageSize, all, name)
	// 聊天列表
	c.JSON(200, result)
}

func (a Api) detailHandler(c *gin.Context) {
	pageIndex, _ := strconv.Atoi(c.DefaultQuery("pageIndex", "1"))
	pageSize, _ := strconv.Atoi(c.DefaultQuery("pageSize", "10"))
	talker := c.Query("talker")
	c.JSON(200, a.wcdb.ChatDetailList(talker, pageIndex-1, pageSize))
}

func (a Api) Router() http.Handler {
	g := gin.New()
	g.Use(gin.Recovery())

	g.GET(ListApi, a.listHandler)
	g.GET(DetailApi, a.detailHandler)

	return g
}
