package main

import (
	"flag"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"golang.org/x/sync/errgroup"
)

var apiPort = flag.String("ap", "9999", "api port")
var htmlPort = flag.String("hp", "9991", "html port")
var basePath = flag.String("f", "", "wechat bak folder")

func init() {
	flag.Parse()
	if basePath == nil || *basePath == "" {
		panic("please specify basePath")
	}
}

var (
	g errgroup.Group
)

func htmlRouter() http.Handler {
	e := gin.New()
	e.Use(gin.Recovery())
	e.StaticFS("/", http.Dir("./static"))
	return e
}

func apiRouter() http.Handler {
	e := gin.New()
	e.Use(gin.Recovery())
	e.GET("/", func(c *gin.Context) {
		c.JSON(
			http.StatusOK,
			gin.H{
				"code":    http.StatusOK,
				"message": "Welcome server 02",
			},
		)
	})

	return e
}

func main() {
	htmlRouter := &http.Server{
		Addr:         fmt.Sprintf(":%s", *htmlPort),
		Handler:      htmlRouter(),
		ReadTimeout:  5 * time.Second,
		WriteTimeout: 10 * time.Second,
	}

	apiRouter := &http.Server{
		Addr:         fmt.Sprintf(":%s", *apiPort),
		Handler:      apiRouter(),
		ReadTimeout:  5 * time.Second,
		WriteTimeout: 10 * time.Second,
	}

	g.Go(func() error {
		return htmlRouter.ListenAndServe()
	})

	g.Go(func() error {
		return apiRouter.ListenAndServe()
	})

	if err := g.Wait(); err != nil {
		log.Fatal(err)
	}
}
