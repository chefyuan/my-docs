use std::any::Any;
use std::thread;

use actix_web::App;
use actix_web::{post, web, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use actix_rt::System;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use serde_json::Value;


//获取某个机器的温湿度
#[derive(Deserialize)]
struct OneTemAndHumData {
    SN: String,
    RequestId: i32,
    RightNow: bool,
}

//获取所有机器的温湿度  
#[derive(Deserialize)]
struct AllTemAndHumData {
    RequestId: i32,
    RightNow: bool,
   
}

//获取某个机器的校准值
#[derive(Deserialize)]
struct GetOffsetNum {
    RequestId: i32,
    RightNow: bool,
    SN: String
   
}

// 校准温度和湿度
#[derive(Deserialize)]
struct OffsetTempAndHum {
    RequestId: i32,
    RightNow: bool,
    SN:String,
    Offset: i32,
   
}

//返回结构体
#[derive(Deserialize)]
struct  BodyData {
    Name:String,
    Temperature:f32,
    Humidity:f32,
    Timestamp:i32,
    SN:String,     
}

#[derive(Deserialize)]
struct ReturnData {
    RequestId:i32,
    code:i32,
    body:BodyData,
}

//用来保存 socket
struct SocketData {
    conn_socket: TcpStream,
    temperature: f32,
    humidity: f32,
}


impl SocketData {
    fn new(conn_socket: TcpStream, temperature: f32, humidity: f32) -> Self {
        SocketData {
            conn_socket,
            temperature,
            humidity,
        }
    }
}

enum RequestIdList {
    GetAllSensorData = 1,
    GetOneSensorData = 2,
    GetOneOffsetTemp = 3,
    GetOneOffsetHum = 4,
    CalibrationTemp = 5,
    CalibrationHum = 6 
}

//处理客户端连接
fn handle_client(mut conn_socket: TcpStream, conn_sockets_clone: &Arc<Mutex<HashMap<String, SocketData>>>) {

    //获取到 SN 号码
    let mut buffer = [0; 1024];
    conn_socket.read(&mut buffer).unwrap();
   
    let mut socket_number = String::from_utf8_lossy(&buffer).trim_end_matches(char::from(0)).to_owned();
    let mut connections_guard = conn_sockets_clone.lock().unwrap();
    let socket_data = SocketData::new (conn_socket, 0.0, 0.0);

    connections_guard.insert(socket_number, socket_data);
    
}

// 操作传感器
async fn operating_sensors(post_data: web::Json<Value>, conn_sockets_clone_web_server: web::Data<Arc<Mutex<HashMap<String, SocketData>>>>) -> HttpResponse {
  
    let json_data = post_data.into_inner();
    let mut connections_guard = conn_sockets_clone_web_server.lock().unwrap();

    if let Some(request_id) = json_data.get("RequestId"){

        if let Some(request_id_i64) = request_id.as_i64() {
    
            match request_id_i64 {
                1 => {
                    // 处理 GetAllSensorData 的逻辑
                }
                2 => {
                    // 处理 GetOneSensorData 的逻辑
                }
                3 => {

                }
                _ => {
                    // 处理未知的请求ID情况
                }
            }

        }

        HttpResponse::Ok().body("Hello world11111111!")

    } else {
        HttpResponse::Ok().body("Hello world11111111!")
    }

}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //定义 socket map
    let conn_sockets: Arc<Mutex<HashMap<String, SocketData>>> = Arc::new(Mutex::new(HashMap::new()));

    let conn_sockets_server = Arc::clone(&conn_sockets); 

    let http_server = HttpServer::new(move|| {
        let conn_sockets_clone_web_server = Arc::clone(&conn_sockets_server);

        App::new()
            .service(web::scope("/api")
                .data(conn_sockets_clone_web_server)
                .route("/OperatingSensors", web::post().to(operating_sensors))
               
            )
    }).bind("127.0.0.1:8000")?;

    // 启动 HTTP 服务器
    let http_server_handle = http_server.run(); 

    // 创建 TCP 监听器并处理传入的连接
    let tcp_listener = TcpListener::bind("127.0.0.1:9000")?;

    for conn_socket in tcp_listener.incoming() {

        let conn_socket = conn_socket?;
        let conn_sockets_clone = Arc::clone(&conn_sockets);

        thread::spawn(move || {
            //接收到链接之后，处理其内容
            handle_client(conn_socket, &conn_sockets_clone);
        });
    }

    // 等待 HTTP 服务器运行完毕并关闭
    http_server_handle.await?;

    Ok(())
}


