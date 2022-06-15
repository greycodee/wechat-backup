package main

import (
	"embed"
	"encoding/json"
	"fmt"
	"io/fs"
	"log"
	"net/http"
	"strconv"

	"github.com/greycodee/wcdb-parse/db"
)

var enmicromsg *db.EnMicroMsg
var wxfileindex *db.WxFileIndex

//go:embed static
var htmlFile embed.FS

func main() {
	enmicromsg = db.OpenEnMicroMsg("/mnt/d/MicroMsg/enmicromsg_plaintext.db")
	wxfileindex = db.OpenWxFileIndex("/mnt/d/MicroMsg/wxfileindex_plaintext.db")

	fsys, _ := fs.Sub(htmlFile, "static")
	staticHandle := http.FileServer(http.FS(fsys))

	http.Handle("/", staticHandle)
	http.Handle("/api/", route())

	log.Println("server start")
	err := http.ListenAndServe(":8080", nil)
	if err != nil {
		log.Fatalf("ListenAndServe: %v", err)
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
		// 聊天列表
		params := r.URL.Query()
		pageIndex, _ := strconv.Atoi(params["pageIndex"][0])
		pageSize, _ := strconv.Atoi(params["pageSize"][0])
		result, err := json.Marshal(enmicromsg.ChatList(pageIndex-1, pageSize))
		if err != nil {
			log.Fatalf("json marshal error: %v", err)
		}
		w.Write(result)
	},
	"/api/chat/detail": func(w http.ResponseWriter, r *http.Request) {
		//聊天记录
		params := r.URL.Query()
		talker := params["talker"][0]
		pageIndex, _ := strconv.Atoi(params["pageIndex"][0])
		pageSize, _ := strconv.Atoi(params["pageSize"][0])

		result, err := json.Marshal(enmicromsg.ChatDetailList(talker, pageIndex-1, pageSize))
		if err != nil {
			log.Fatalf("json marshal error: %v", err)
		}
		w.Write(result)
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
