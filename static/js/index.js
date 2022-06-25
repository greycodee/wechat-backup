$(function () {
  let pageIndex = 1;
  let pageSize = 5;
  let noneData = false;
  let host = window.location.host;
  let timestamp = new Date().getTime();

  $(".chat").niceScroll();
  $(".chat-list").niceScroll();

  addUserChatList(0,0,true);

  // 滚动高度至最底部
  $('.chat').getNiceScroll(0).doScrollTop($('.chat-body').height(), -1); // -1 is the animation duration


  function timestampToTime(timestamp) {
    var date = new Date(timestamp);
    return date.toLocaleString()
  }

  function getText(chat) {
    switch (chat.type) {
      case 1:
        return chat.content;
      case 3:
        return '<a href="' + chat.mediaSourcePath + '"><img src="' + chat.mediaPath + '" alt="图片" width="100" height="200" ></a>';
      case 34:
        // 语音由于将amr转换成了mp3，所以这里的路径是mp3
        // mediaPath = mediaPath.split('.')[0] + '.mp3';
        return `<audio controls>
                    <source src="`+ chat.mediaPath + `" type="audio/mpeg">
                    您的浏览器不支持 audio 元素。
                  </audio>`;
      case 43:
        return `<video controls width="250"> -->
                    <source src="`+ chat.mediaPath + `">
                    Sorry, your browser doesn't support embedded videos.
                  </video>`;
      case 47:
        return '[大表情]';
      case 49:
        return '[卡片信息]';
      case 436207665:
        return '[微信红包]';
      case 419430449:
        return '[微信转账]';
      case 1090519089:
        return `<a href="`+chat.fileInfo.filePath+`">
                  <div style="width: 210px; height: 60px;">
                      <div class="filedesc">
                          <div class="title">`+chat.fileInfo.fileName+`</div>
                          <div class="desc">`+chat.fileInfo.fileSize+`</div>
                      </div>
                      <div class="fileico"">
                          <embed src="./img/`+chat.fileInfo.fileExt+`.svg" type="image/svg+xml" width="50" height="50"/>
                      </div>
                  </div>
                </a>`;
      case 10000:
        // 撤回消息
        return '[撤回]'+chat.content;
      default:
        return '[未知消息]';
    }
  }

  // 添加个人用户聊天记录
  function addChatList(talker, pageIndex, pageSize, async) {
    $.ajax({
      url: ' http://' + host + '/api/chat/detail?talker=' + talker + '&pageIndex=' + pageIndex + '&pageSize=' + pageSize + '',
      type: 'GET',
      jsonp: true,
      async: async,
      dataType: 'json',
      success: function (data) {
        if (data.rows.length == 0) {
          noneData = true;
          alert('没有更多数据了');
        } else {
          let ta = talker.split('@')
          let chatRoomFlag = false
          if (ta.length == 2 && ta[1] === 'chatroom') {
            // 群聊 
            chatRoomFlag = true
          }
          jQuery.each(data.rows, function (i, item) {
            if (chatRoomFlag) {
              item.talker = item.content.split(':', 1)[0];
              item.content = item.content.slice(item.talker.length + 1);
            }
            addChatBody(item)
            // $('.chat').getNiceScroll(0).doScrollTop(837);
          }
          );
        }
      }
    });
  }

  function addChatBody(item) {
    let position = item.isSend == 0 ? 'left' : 'right';
    let userInfo = item.isSend == 0 ? getUserInfoLocalStrage(item.talker, false) : getUserInfoLocalStrage('', true);

    let n1 = typeof (userInfo.conRemark) == 'undefined' || userInfo.conRemark == "" ? userInfo.nickName : userInfo.conRemark;
    let n2 = n1 == "" ? userInfo.alias : n1;
    let n3 = n2 == "" ? userInfo.userName : n2;
    let img1 = typeof (userInfo.reserved2) == 'undefined' || userInfo.reserved2 == "" ? userInfo.reserved1 : userInfo.reserved2;
    let img2 = img1 == "" ? userInfo.localAvatar : img1;
    let div = `<div class="answer ${position}">
              <div class="avatar">
                <img src="${img2}" alt="${n3}">
              </div>
              <div class="name">${n3}</div>
              <div class="text">
                ${getText(item)}
              </div>
              <div class="time">${timestampToTime(item.createTime)}</div>
            </div>`;
    $(".divide").after(div);
  }

  // 聊天列表初始化
  function addUserChatList(pageIndex, pageSize, all) {
    let url = '';
    if (all) {
      url = ' http://' + host + '/api/chat/list?all=true&pageIndex=' + pageIndex + '&pageSize=' + pageSize + '';
    }else{
      url = ' http://' + host + '/api/chat/list?pageIndex=' + pageIndex + '&pageSize=' + pageSize + '';
    }
    $.ajax({
      url: url,
      type: 'GET',
      jsonp: true,
      async: false,
      dataType: 'json',
      success: function (data) {
        jQuery.each(data.rows, function (i, item) {
          let name = typeof (item.conRemark) == 'undefined' || item.conRemark == "" ? item.nickname : item.conRemark;
          let img1 = typeof (item.reserved2) == 'undefined' || item.reserved2 == "" ? item.reserved1 : item.reserved2;
          let img2 = img1 == "" ? item.localAvatar : img1;
          let li = `<li class="list-group-item d-flex justify-content-between align-items-start" id="${item.talker}">
                      <div class="avatar">
                          <img src="${img2}" alt="头像">
                      </div>
                      <div class="ms-2 me-auto chat-list-item">
                          <div class="fw-bold chat-title">${name}</div>
                          <div class="fw-bold chat-talker">talker: ${item.talker}</div>
                      </div>
                    <span class="badge rounded-pill">${item.msgCount}</span>
                  </li>`;
          $(".chat-user-list").append(li);
        })
      }
    });
  }

  $("ul li").click(function () {
    let talker = $(this).attr('id');
    noneData = false;
    // 更新聊天框
    $("li").removeClass("active");
    $(this).addClass("active");
    $(".chat-body").html('');
    let more = `<div class="divide" ><i class="fa fa-arrow-circle-o-right"></i></div>`;
    $(".chat-body").prepend(more);
    $('.divide').click(function () {
      moreData(talker);
    })
    pageIndex = 1
    addChatList(talker, pageIndex, pageSize, true, getUserInfo(talker), getUserInfo(talker));
  })

  // 获取用户信息
  function getUserInfo(username) {
    let info = {};
    $.ajax({
      url: ' http://' + host + '/api/user/info?username=' + username,
      type: 'GET',
      jsonp: true,
      async: false,
      dataType: 'json',
      success: function (data) {
        info = data;
      }
    });
    return info;
  }

  function getUserInfoLocalStrage(talker, isMyself) {
    if (isMyself) {
      let myInfo = {};
      let k = "myinfo" + timestamp;
      let local = localStorage.getItem(k)
      if (local && local != '') {
        myInfo = JSON.parse(local);
      } else {
        $.ajax({
          url: ' http://' + host + '/api/user/myinfo',
          type: 'GET',
          jsonp: true,
          async: false,
          dataType: 'json',
          success: function (data) {
            myInfo = data;
            localStorage.setItem(k, JSON.stringify(myInfo));
          }
        });
      }
      return myInfo;
    } else {
      let local = localStorage.getItem(talker)
      let userInfo = {}
      if (local && local != '') {
        userInfo = JSON.parse(local);
      } else {
        userInfo = getUserInfo(talker);
        localStorage.setItem(talker, JSON.stringify(userInfo));
      }
      return userInfo;

    }
  }

  function moreData(talker) {
    if (!noneData) {
      pageIndex++;
      addChatList(talker, pageIndex, pageSize, true);
    } else {
      alert('没有更多数据了');
    }
  }

  function getMediaPath(msgId, type) {
    let url = ''
    let mediaPath = ''
    switch (type) {
      case 3:
        url = 'http://' + host + '/api/media/img?msgId=' + msgId;
        break;
      case 34:
        url = 'http://' + host + '/api/media/voice?msgId=' + msgId;
        break;
      case 43:
        url = 'http://' + host + '/api/media/video?msgId=' + msgId;
        break;
    }
    $.ajax({
      url: url,
      type: 'GET',
      async: false,
      success: function (data) {
        mediaPath = data;
      }
    });
    return mediaPath;
  }
}) 