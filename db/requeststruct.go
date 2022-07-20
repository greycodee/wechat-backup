package db

type ChatListRequestBody struct {
	All       bool `json:"all"`
	PageIndex int  `json:"pageIndex"`
	PageSize  int  `json:"pageSize"`
}
