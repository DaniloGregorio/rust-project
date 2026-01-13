use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::{self, Json}};
use serde::{Deserialize , Serialize};


//post route to  send JSON,example:
//{
//  leader : "Danilo",
//  member1 : "kevin"
//{

#[post("/team")]
async fn team(team:Json<Team>) ->impl Responder{
    let msg = format!("leader: {} , member1: {}",team.leader,team.member1);
    HttpResponse::Ok().body(msg)
}
#[derive(Deserialize)]
struct Team{
  leader : String,
  member1 : String,
}
//query parameters 
//http://localhost:3000/person?name=danilo&age=21
#[get("/person")]
async fn param(info: web::Query<Info>) -> impl Responder{
  let msg = format!("name :{}, age : {}",info.name,info.age);
  HttpResponse::Ok().body(msg)
    
}

//simple get
#[get("/one")]
async  fn one() -> impl Responder{
  HttpResponse::Ok().body("one")
}

//simple post
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
    .service(param)
    .service(handlerfunc)
        .route("/", web::get().to(|| async {
          HttpResponse::Ok().body("hello world".to_string())
        }))
    
  }).bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await 
    .unwrap()

}

#[derive(Deserialize)]
  struct Info{
  name: String,
  age: i32 
  }

//route to get a response of a JSON
#[get("/handler")]
async fn handlerfunc()-> impl Responder{
  let persona : Persona = Persona { name: "Danilo".to_string(), age: 21 };
  let persona_json = serde_json::to_string(&persona).unwrap();
  HttpResponse::Ok().json(persona_json)
}

#[derive(Serialize)]
struct Persona{
  name : String,
  age : i32
}
