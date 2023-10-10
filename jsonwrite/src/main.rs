use serde::{Deserialize,Serialize};



#[derive(Deserialize,Serialize)]
struct Myinfo{
    name:String,
    age:i32
}
fn main(){

    let info:Myinfo=Myinfo { 
        name:String::from("nihang"),
        age:24
    };

    let json=serde_json::to_string(&info).unwrap();

    println!("{}",json)

}