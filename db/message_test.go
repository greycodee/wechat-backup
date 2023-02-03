package db

import (
	"testing"

	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

func TestQuery(t *testing.T) {
	db, err := gorm.Open(sqlite.Open("/home/zheng/coding/wechatbak/dest/79b23ef49a3016d8c52a787fc4ab59e4/EnMicroMsg_plain.db"), &gorm.Config{})
	if err != nil {
		t.Fatal(err)
	}
	// db.AutoMigrate(&Message{})

	var message Message
	db.Take(&message)
	t.Log(message)

}

func TestOut(t *testing.T) {
	t.Log("hello")
	// t.Error("sad")
}
