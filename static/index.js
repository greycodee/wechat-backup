$(function(){
    let pageIndex = 1;
    let pageSize  = 10;

    $(".chat").niceScroll(
    );
    
    // 滑动监听
    $(".chat").getNiceScroll(0).scrollend(function(e) {
      // TODO
      console.log(e);
      if(e.current.y<300 && e.current.y>0){
        pageIndex++;
        addChatList(paramsObj['talker'],pageIndex,pageSize,true);
      }
    });
    
    let search = window.location.search;
    let params = search.split('?')[1];
    let paramsArray = params.split('&');
    let paramsObj = {};
    if (paramsArray.length > 0) {
      jQuery.each( paramsArray, function( i, item ) {
        let param = item.split('=');
        paramsObj[param[0]] = param[1];
      });
    }else{
      let param = params.split('=');
      paramsObj[param[0]] = param[1];
    }

    let otherInfo = '';
    $.ajax({
      url: 'http://127.0.0.1:8080/api/user/info?username='+paramsObj['talker'],
      type: 'GET',
      jsonp: true,
      async: false,
      dataType: 'json',
      success: function(data){
        otherInfo=data;
      }});

    let myInfo = '';
    $.ajax({
      url: 'http://127.0.0.1:8080/api/user/info?username=wxid_nn17m6suhg8122',
      type: 'GET',
      jsonp: true,
      async: false,
      dataType: 'json',
      success: function(data){
        myInfo=data;
      }});

    // $.ajax({
    //   url: 'http://127.0.0.1:8080/api/chat/detail?talker='+paramsObj['talker']+'&pageIndex=1&pageSize=20',
    //   type: 'GET',
    //   jsonp: true,
    //   async: false,
    //   dataType: 'json',
    //   success: function(data){
    //     let htmldiv2 = ``;
    //     jQuery.each( data.rows, function( i, item ) {
    //       let position = item.isSend==0? 'left': 'right';
    //       let userInfo = item.isSend==0? otherInfo: myInfo;

    //       let n1 = userInfo.conRemark==""? userInfo.nickName: userInfo.conRemark;
    //       let n2 = n1==""?userInfo.alias:n1;
    //       let n3 = n2==""?userInfo.userName:n2;
    //       let div = `<div class="answer ${position}">
    //               <div class="avatar">
    //                 <img src="${userInfo.reserved2}" alt="${userInfo.nickName}">
        
    //               </div>
    //               <div class="name">${n3}</div>
    //               <div class="text">
    //                 ${getText(item)}
    //               </div>
    //               <div class="time">${timestampToTime(item.createTime)}</div>
    //             </div>`;
    //       $(".chat-body").prepend(div);
    //       }
    //     );
        
    //   }});

    addChatList(paramsObj['talker'],pageIndex,pageSize,false);

    //  = $(".chat").scrollHeight // 滚动高度至最底部
    // $('.chat').getNiceScroll(0).doScrollTop($('.chat-body').height());
    $('.chat').getNiceScroll(0).doScrollTop($('.chat-body').height(), -1); // -1 is the animation duration


    function timestampToTime(timestamp) {
      var date = new Date(timestamp);
      return date.toLocaleString()
    }

    function getText(chat){
      switch (chat.type) {
        case 1:
          return chat.content;
        case 3:
          return '[图片]';
        case 34:
          return '[语音]';
        case 43:
          return '[视频]';
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

    // 添加聊天记录
    function addChatList(talker,pageIndex,pageSize,async){
      $.ajax({
        url: 'http://127.0.0.1:8080/api/chat/detail?talker='+talker+'&pageIndex='+pageIndex+'&pageSize='+pageSize+'',
        type: 'GET',
        jsonp: true,
        async: async,
        dataType: 'json',
        success: function(data){
          jQuery.each( data.rows, function( i, item ) {
            let position = item.isSend==0? 'left': 'right';
            let userInfo = item.isSend==0? otherInfo: myInfo;
  
            let n1 = userInfo.conRemark==""? userInfo.nickName: userInfo.conRemark;
            let n2 = n1==""?userInfo.alias:n1;
            let n3 = n2==""?userInfo.userName:n2;
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
          );
          
        }});
    }
    



}) 