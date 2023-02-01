package main

import (
	"embed"
	"flag"
	"fmt"
	"io/fs"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/greycodee/wechat-backup/api"
)

var apiPort = flag.String("p", "9999", "api port")
var basePath = flag.String("f", "", "wechat bak folder")

//go:embed static
var staticFile embed.FS

//go:embed index.html
var indexHtml []byte

func init() {
	flag.Parse()
	if basePath == nil || *basePath == "" {
		panic("please specify basePath")
	}
}

func main() {

	fsys, _ := fs.Sub(staticFile, "static")

	apiRouter := api.New(*basePath)

	apiRouter.Engine.StaticFS("/static", http.FS(fsys))
	apiRouter.Engine.GET("/", func(ctx *gin.Context) {
		ctx.Header("Content-Type", "text/html")
		ctx.String(http.StatusOK, string(indexHtml))
	})

	apiRouter.Engine.Static("/media/", *basePath)

	apiRouter.Engine.NoRoute(func(ctx *gin.Context) {
		ctx.Redirect(http.StatusFound, "/")
	})

	httpServer := &http.Server{
		Addr:         fmt.Sprintf(":%s", *apiPort),
		Handler:      apiRouter.Router(),
		ReadTimeout:  5 * time.Second,
		WriteTimeout: 10 * time.Second,
	}

	httpServer.ListenAndServe()
}
