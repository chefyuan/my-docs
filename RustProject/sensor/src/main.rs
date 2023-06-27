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
    a: f32,
    b: f32,
}


impl SocketData {
    fn new(conn_socket: TcpStream, a: f32, b: f32) -> Self {
        SocketData {
            conn_socket,
            a,
            b,
        }
    }
}



//处理客户端连接
fn handle_client(mut conn_socket: TcpStream, conn_sockets_clone: &Arc<Mutex<HashMap<String, SocketData>>>) {

    //获取到 SN 号码
    let mut buffer = [0; 1024];
    conn_socket.read(&mut buffer).unwrap();

    let mut socket_number = String::from_utf8_lossy(&buffer).trim().to_owned();

    println!("Received data: {}", socket_number);
   
    let mut connections_guard = conn_sockets_clone.lock().unwrap();
    let socket_data = SocketData::new (conn_socket, 0.0, 0.0);

    connections_guard.insert(socket_number.to_string(), socket_data);


    // 这里需要接收到客户端的 SN，然后将其设置为 key 存到 map 里
   
    // let mut tmp_socket_data = connections_guard.get(&socket_number).unwrap();
    // let mut stream = tmp_socket_data.conn_socket.try_clone().unwrap();
    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    // println!("Received data: {}", String::from_utf8_lossy(&buffer));

    // let response = "Hello from TCP server!";
    // let response1 = "Hello from TCP server1!";
    // let response2 = "Hello from TCP server2!";
    // let response3 = "Hello from TCP server3!";
    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
    // stream.write(response1.as_bytes()).unwrap();
    // stream.flush().unwrap();
    // stream.write(response2.as_bytes()).unwrap();
    // stream.flush().unwrap();
    // stream.write(response3.as_bytes()).unwrap();
    // stream.flush().unwrap();
}

// 获取某个机器的温湿度
async fn get_one_sensor_data_fun(post_data: web::Json<OneTemAndHumData>, conn_sockets_clone_web_server: web::Data<Arc<Mutex<HashMap<String, SocketData>>>>) -> HttpResponse {
  
    println!("{} {}", post_data.SN, post_data.RequestId);
    
    // println!("conn_sockets_clone_web_server: {:?}", conn_sockets_clone_web_server);
    // 或者使用 dbg! 宏
    // dbg!(&conn_sockets_clone_web_server);

    let sy_number = post_data.SN.clone();

    // print!("{}", sy_number);
    

    let mut connections_guard = conn_sockets_clone_web_server.lock().unwrap();

    println!("sy_number: {:?}", sy_number);
    let mut tmp_socket_data = connections_guard.get(&sy_number).unwrap();


    let mut stream = tmp_socket_data.conn_socket.try_clone().unwrap();

    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();

    // println!("Received data: {}", String::from_utf8_lossy(&buffer));

    let response = "Hello from TCP server!";
    let response1 = "Hello from TCP server1!";
    let response2 = "Hello from TCP server2!";
    let response3 = "Hello from TCP server3!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.write(response1.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.write(response2.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.write(response3.as_bytes()).unwrap();
    stream.flush().unwrap();
    HttpResponse::Ok().body("Hello world11111111!")
}

//获取所有机器的温湿度
async fn get_all_sensor_data_fun(post_data: web::Json<AllTemAndHumData>) -> HttpResponse {
    println!("{}", post_data.RequestId);
    HttpResponse::Ok().body("Hello world11111111!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //定义 socket map
    let conn_sockets: Arc<Mutex<HashMap<String, SocketData>>> = Arc::new(Mutex::new(HashMap::new()));

    let conn_sockets_server = Arc::clone(&conn_sockets); 

    let http_server = HttpServer::new(move|| {
        let conn_sockets_clone_web_server = Arc::clone(&conn_sockets);
        
        App::new()
            .service(web::scope("/api")
                .data(conn_sockets_clone_web_server)
                .route("/GetOneSensorData", web::post().to(get_one_sensor_data_fun))
                .route("/GetAllSensorData", web::post().to(get_all_sensor_data_fun))
            )
    }).bind("127.0.0.1:8000")?;

    // 启动 HTTP 服务器
    let http_server_handle = http_server.run(); 

    // 创建 TCP 监听器并处理传入的连接
    let tcp_listener = TcpListener::bind("127.0.0.1:9000")?;

    for conn_socket in tcp_listener.incoming() {

        let conn_socket = conn_socket?;
        let conn_sockets_clone = Arc::clone(&conn_sockets_server);

        thread::spawn(move || {
            //接收到链接之后，处理其内容
            handle_client(conn_socket, &conn_sockets_clone);
        });
    }

    // 等待 HTTP 服务器运行完毕并关闭
    http_server_handle.await?;

    Ok(())
}


