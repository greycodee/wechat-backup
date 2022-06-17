$(function () {
  let pageIndex = 1;
  let pageSize = 5;
  let noneData = false;
  let host = window.location.host;

  $(".chat").niceScroll();
  $(".chat-list").niceScroll();

  addUserChatList(1, 50);

  $("#more").click(function () {
    if (!noneData) {
      let h = $('.chat-body').height()
      console.log(h)
      pageIndex++;
      addChatList(paramsObj['talker'], pageIndex, pageSize, true);

      let h2 = $('.chat-body').height()
      console.log(h2)
    } else {
      console.log("没有更多数据了");
    }
  })

  //  = $(".chat").scrollHeight // 滚动高度至最底部
  // $('.chat').getNiceScroll(0).doScrollTop($('.chat-body').height());
  $('.chat').getNiceScroll(0).doScrollTop($('.chat-body').height(), -1); // -1 is the animation duration


  function timestampToTime(timestamp) {
    var date = new Date(timestamp);
    return date.toLocaleString()
  }

  function getText(chat) {
    let mediaPath = 'media/' + getMediaPath(chat.msgId, chat.type)
    switch (chat.type) {
      case 1:
        return chat.content;
      case 3:
        return '<img src="' + mediaPath + '" alt="图片" width="100" height="200" >';
      case 34:
        // 语音由于将amr转换成了mp3，所以这里的路径是mp3
        mediaPath = mediaPath.split('.')[0] + '.mp3';
        return `<audio controls>
                    <source src="`+ mediaPath + `" type="audio/mpeg">
                    您的浏览器不支持 audio 元素。
                  </audio>`;
      case 43:
        return `<video controls width="250"> -->
                    <source src="`+ mediaPath + `">
                    Sorry, your browser doesn't support embedded videos.
                  </video>`;
      case 47:
        return '[大表情]';
      case 49:
        return '[文件]';
      case 436207665:
        return '[微信红包]';
      case 419430449:
        return '[微信转账]';
      default:
        return '[未知消息]';
    }
  }

  // 添加个人用户聊天记录
  function addChatList(talker, pageIndex, pageSize, async, otherInfo, myInfo) {
    $.ajax({
      url: ' http://' + host + '/api/chat/detail?talker=' + talker + '&pageIndex=' + pageIndex + '&pageSize=' + pageSize + '',
      type: 'GET',
      jsonp: true,
      async: async,
      dataType: 'json',
      success: function (data) {
        if (data.rows.length == 0) {
          noneData = true;
        } else {
          jQuery.each(data.rows, function (i, item) {
            addChatBody(item, myInfo, otherInfo)
            $('.chat').getNiceScroll(0).doScrollTop(837);
          }
          );
        }
      }
    });
  }

  function addChatBody(item, myInfo, otherInfo) {
    let position = item.isSend == 0 ? 'left' : 'right';
    let userInfo = item.isSend == 0 ? otherInfo : myInfo;

    let n1 = userInfo.conRemark == "" ? userInfo.nickName : userInfo.conRemark;
    let n2 = n1 == "" ? userInfo.alias : n1;
    let n3 = n2 == "" ? userInfo.userName : n2;
    let div = `<div class="answer ${position}">
              <div class="avatar">
                <img src="${userInfo.reserved2}" alt="${userInfo.nickName}">
    
              </div>
              <div class="name">${n3}</div>
              <div class="text">
                ${getText(item)}
              </div>
              <div class="time">${timestampToTime(item.createTime)}</div>
            </div>`;
    $(".chat-body").prepend(div);
  }

  // 聊天列表初始化
  function addUserChatList(pageIndex, pageSize) {
    $.ajax({
      url: ' http://' + host + '/api/chat/list?pageIndex=' + pageIndex + '&pageSize=' + pageSize + '',
      type: 'GET',
      jsonp: true,
      async: false,
      dataType: 'json',
      success: function (data) {
        jQuery.each(data.rows, function (i, item) {
          let li = `<li class="list-group-item d-flex justify-content-between align-items-start" id="${item.talker}">
                      <div class="avatar">
                          <img src="${item.reserved1}" alt="头像">
                      </div>
                      <div class="ms-2 me-auto chat-list-item">
                          <div class="fw-bold chat-title">${item.nickname}</div>
                          <div class="fw-bold chat-talker">talker: ${item.talker}</div>
                      </div>
                    <span class="badge bg-primary rounded-pill">${item.msgCount}</span>
                  </li>`;
          $(".chat-user-list").append(li);
        })
      }
    });
  }

  $("ul li").click(function () {
    let talker = $(this).attr('id');
    console.log(talker)
    // 更新聊天框
    // var talker = $(this).attr('class');
    $("li").removeClass("active");
    $(this).addClass("active");
    $(".chat-body").html('');
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

  function getMediaPath(msgId, type) {
    let url = ''
    let imgPath = ''
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
        imgPath = data;
      }
    });
    return imgPath;
  }
}) 