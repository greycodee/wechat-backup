$(function(){
    let pageIndex = 1;
    let pageSize  = 10;
    let noneData = false;
    let host = window.location.host;

    $(".chat").niceScroll(
    );
    
    // 滑动监听
    $(".chat").getNiceScroll(0).scrollend(function(e) {
      // TODO
      console.log(e);
      if(e.current.y<300 && e.current.y>0){
        if(!noneData){
          pageIndex++;
          addChatList(paramsObj['talker'],pageIndex,pageSize,true);
        }else{
          console.log("没有更多数据了");
        }
       
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
      url: ' http://'+host+'/api/user/info?username='+paramsObj['talker'],
      type: 'GET',
      jsonp: true,
      async: false,
      dataType: 'json',
      success: function(data){
        otherInfo=data;
      }});

    let myInfo = '';
    $.ajax({
      url: ' http://'+host+'/api/user/info?username=wxid_nn17m6suhg8122',
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
      let mediaPath = 'media/'+getMediaPath(chat.msgId,chat.type)
      switch (chat.type) {
        case 1:
          return chat.content;
        case 3:
          return '<img src="'+mediaPath+'" alt="图片" width="100" height="200" >';
        case 34:
          // TODO 语音要转码 amr格式
          // return '<audio controls>'+
          // '<source src="'+mediaPath+'" >您的浏览器不支持 audio 元素。</audio>';
          return `<audio controls>
                    <source src="`+mediaPath+`" type="audio/mpeg">
                    您的浏览器不支持 audio 元素。
                  </audio>`;
        case 43:
          var samples = AMR.decode(mediaPath);
          return `<video controls width="250"> -->
                    <source src="`+samples+`">
          
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

    // 添加聊天记录
    function addChatList(talker,pageIndex,pageSize,async){
      $.ajax({
        url: ' http://'+host+'/api/chat/detail?talker='+talker+'&pageIndex='+pageIndex+'&pageSize='+pageSize+'',
        type: 'GET',
        jsonp: true,
        async: async,
        dataType: 'json',
        success: function(data){
          if  (data.rows.length==0) {
            noneData = true;
          }else{
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
          }
        }});

        
    }

    function getMediaPath(msgId,type){
      let url = ''
      let imgPath = ''
      switch (type) {
        case 3:
          url = 'http://'+host+'/api/media/img?msgId='+msgId;
        break;
        case 34:
          url = 'http://'+host+'/api/media/voice?msgId='+msgId;
          // TODO 语音要转码 amr格式
          // return '<audio controls>'+
          // '<source src="'+mediaPath+'" >您的浏览器不支持 audio 元素。</audio>';
          break;
        case 43:
          url = 'http://'+host+'/api/media/video?msgId='+msgId;
          break;
      }
      $.ajax({
        url: url,
        type: 'GET',
        async: false,
        success: function(data){
          imgPath = data;
        }
      });
      return imgPath;
    }
    




    // amr
  function readBlob(blob, callback) {
    var reader = new FileReader();
    reader.onload = function(e) {
        var data = new Uint8Array(e.target.result);
        callback(data);
    };
    reader.readAsArrayBuffer(blob);
  }
  function playAmrBlob(blob, callback) {
    readBlob(blob, function(data) {
        playAmrArray(data);
    });
  }  
  function playAmrArray(array) {
    var samples = AMR.decode(array);
    if (!samples) {
        alert('Failed to decode!');
        return;
    }
    playPcm(samples);
  }

  function playPcm(samples) {
      var ctx = getAudioContext();
      var src = ctx.createBufferSource();
      var buffer = ctx.createBuffer(1, samples.length, 8000);
      if (buffer.copyToChannel) {
          buffer.copyToChannel(samples, 0, 0)
      } else {
          var channelBuffer = buffer.getChannelData(0);
          channelBuffer.set(samples);
      }

      src.buffer = buffer;
      src.connect(ctx.destination);
      src.start();
  }


}) 