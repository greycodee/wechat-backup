package db

import (
	"encoding/json"
	"testing"
)

func TestQuery(t *testing.T) {
	res, _ := GetChatList()

	str, _ := json.Marshal(res)
	t.Log(string(str))
}
