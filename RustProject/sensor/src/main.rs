use actix_web::App;
use actix_web::{post, web, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

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
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .route("/GetOneSensorData", web::post().to(get_one_sensor_data_fun))
                .route("/GetAllSensorData", web::post().to(get_all_sensor_data_fun))
            )
    })
    .bind("127.0.0.1:8000")
    .expect("fail to bindin")
    .run()
    .await
}

