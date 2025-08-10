use actix_web::{HttpServer,HttpResponse,App,web,get,post,Responder};
use serde::Deserialize;
#[get("/")]
async fn index()->impl Responder{
    "Welcome to the rust backend program "
}
#[derive(Deserialize)]
struct Info {
    name :String,
    work:String,age:u128,
}

#[post("/greet")]
async fn greet(req:web::Json<Info>)->impl Responder{
    let name = &req.name;
    let age = &req.age;
    let work = &req.work;
    let response = format!("Your name is {} and age is {} and as for the work {}",name,age,work);
    HttpResponse::Ok().body(response)
   
}
#[actix_web::main]
async fn main ()-> std::io::Result<()>{
    HttpServer::new(||{
        App::new().service(index).service(greet)
    })
    .bind(("127.0.0.1",8080))?
    .run().await

    
}