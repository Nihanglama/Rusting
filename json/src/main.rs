use serde::{Deserialize,Serialize};


#[derive(Serialize,Deserialize)]
struct Paragraph{
    name:String
}

#[derive(Serialize,Deserialize)]
struct Article{
    article:String,
    author:String,
    paragraph: Vec<Paragraph>

}

fn main(){

    let json = r#"{
        "article":"working with json",
        "author":"Nihang",
        "paragraph":[
            {
                "name":"start"

        },
        {
            "name":"end"
        }
        ]


    }"#;

    let parsed :Article=read_json_typed(json);
    println!("\n\n The name of the first paragrah is {}",parsed.paragraph[0].name);

}

fn read_json_typed(raw_json:&str)-> Article {
    let parsed :Article= serde_json::from_str(raw_json).unwrap();
    return parsed;
}