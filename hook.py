import frida 
import sys   
 
jscode = """
    Java.perform(function(){ 
        var utils = Java.use("com.tencent.wcdb.database.SQLiteDatabase"); // 类的加载路径
         
        utils.openDatabase.overload('java.lang.String', '[B', 'com.tencent.wcdb.database.SQLiteCipherSpec', 'com.tencent.wcdb.database.SQLiteDatabase$CursorFactory', 'int', 'com.tencent.wcdb.DatabaseErrorHandler', 'int').implementation = function(a,b,c,d,e,f,g){  
            console.log("Hook start......");
            var JavaString = Java.use("java.lang.String");
            var database = this.openDatabase(a,b,c,d,e,f,g);
            send(a);
            console.log(JavaString.$new(b));
            send("Hook ending......");
            return database;
        };
         
    });
"""
 
 
def on_message(message,data): 
    if message["type"] == "send":
        print("[*] {0}".format(message["payload"]))
    else:
        print(message)
     
process = frida.get_remote_device()
pid = process.spawn(['com.tencent.mm']) 
session = process.attach(pid)  
script = session.create_script(jscode) 
script.on('message',on_message) 
script.load()
process.resume(pid)
sys.stdin.read()