use std::thread;

use actix_web::App;
use actix_web::{post, web, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use actix_rt::System;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

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




// 获取某个机器的温湿度
async fn get_one_sensor_data_fun(post_data: web::Json<OneTemAndHumData>) -> HttpResponse {
  
    println!("{} {}", post_data.SN, post_data.RequestId);
    HttpResponse::Ok().body("Hello world11111111!")
}

//获取所有机器的温湿度
async fn get_all_sensor_data_fun(post_data: web::Json<AllTemAndHumData>) -> HttpResponse {
    println!("{}", post_data.RequestId);
    HttpResponse::Ok().body("Hello world11111111!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let http_server = HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .route("/GetOneSensorData", web::post().to(get_one_sensor_data_fun))
                .route("/GetAllSensorData", web::post().to(get_all_sensor_data_fun))
            )
    }).bind("127.0.0.1:8000")?;

    // 启动 HTTP 服务器
    let http_server_handle = http_server.run();
    
    // 创建 TCP 监听器并处理传入的连接
    let tcp_listener = TcpListener::bind("127.0.0.1:9000")?;
    for stream in tcp_listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            // 处理客户端连接的代码
        });
    }

    // 等待 HTTP 服务器运行完毕并关闭
    http_server_handle.await?;

    Ok(())
}


