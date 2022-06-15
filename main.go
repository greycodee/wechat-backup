package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/greycodee/wcdb-parse/db"
)

var enmicromsg *db.EnMicroMsg

func main() {
	enmicromsg = db.OpenEnMicroMsg("/Users/zheng/Documents/wcdb/enmicromsg_plaintext.db")

	http.Handle("/api/", route())
	err := http.ListenAndServe(":8080", nil)
	if err != nil {
		log.Fatal(err)
	}
}

func route() http.Handler {
	return &API{}
}

type API struct {
}

func (api *API) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	path := r.URL.Path
	apiMap[path](w, r)
}

var apiMap = map[string]func(w http.ResponseWriter, r *http.Request){
	"/api/chat/list": func(w http.ResponseWriter, r *http.Request) {
		// TODO 聊天列表
		fmt.Println("/api/chat/list")
		w.Write([]byte(enmicromsg.ChatList()))
	},
	"/api/chat/detail": func(w http.ResponseWriter, r *http.Request) {
		// TODO 聊天记录
		fmt.Println("/api/chat/detail")
		w.Write([]byte("/api/chat/detail"))
	},
	"/api/user/info": func(w http.ResponseWriter, r *http.Request) {
		// TODO 用户信息
		fmt.Println("/api/user/info")
		w.Write([]byte("/api/user/info"))
	},
	"/api/media/img": func(w http.ResponseWriter, r *http.Request) {
		// TODO 图片
		fmt.Println("/api/media/img")
		w.Write([]byte("/api/media/img"))
	},
	"/api/media/video": func(w http.ResponseWriter, r *http.Request) {
		// TODO 视频
		fmt.Println("/api/media/video")
		w.Write([]byte("/api/media/video"))
	},
	"/api/media/voice": func(w http.ResponseWriter, r *http.Request) {
		// TODO 语音
		fmt.Println("/api/media/voice")
		w.Write([]byte("/api/media/voice"))
	},
}
