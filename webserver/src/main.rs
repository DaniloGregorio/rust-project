use actix_web::{HttpResponse, HttpServer, web,App, Responder,get,post};

#[get("/person")]
async fn hello(info: web::Query<Info>) -> impl Responder{
  let msg = format!("name :{}, age : {}",info.name,info.age);
  HttpResponse::Ok().body(msg)
    
}

#[get("/one")]
async  fn one() -> impl Responder{
  HttpResponse::Ok().body("one")
}

#[post("/two")]
async fn two()-> impl Responder{
  HttpResponse::Ok().body("a")
}
//dynamic route
#[get("/user/{id}")]
async fn hello(path : web::Path<i32>) -> impl Responder{
  let id = path.into_inner();
  let msg = format!("{}",id);
  HttpResponse::Ok().body(msg)
}

#[actix_web::main]
async fn main(){
  HttpServer::new(|| {
    App::new()
    .service(one)
    .service(two)
        .route("/", web::get().to(|| async {
          HttpResponse::Ok().body("hello world".to_string())
        }))
    
  }).bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await 
    .unwrap()

}