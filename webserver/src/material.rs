/* 

//defalut implementations for build a web sevice
//file created only for develop process

//query parameters 
//http://localhost:3000/person?name=danilo&age=21

#[get("/person")]
async fn param(info: web::Query<Info>) -> impl Responder{
  let msg = format!("name :{}, age : {}",info.name,info.age);
  HttpResponse::Ok().body(msg)
    
}

#[derive(Deserialize)]
  struct Info{
  name: String,
  age: i32 
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
  
*/

