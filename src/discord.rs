use crate::systeminfo::SystemInfo;
use num;
use reqwest::blocking::Client as reqwestClient;
use serde::Serialize;

//Insert webhook url before build
const WHURL: &str = "";

#[derive(Serialize)]
struct PostTest {
    username: String,
    content: String
}

#[derive(Serialize)]
struct Post {
    username: String,
    embeds  : Vec<Embed>
}

#[derive(Serialize)]
struct Embed {
    title : String,  //title of embed
    fields: Vec<Field>
}

#[derive(Serialize)]
struct Field {
    name: String,
    value: String,
    inline: bool
}


/*
 * Implemented only for u64, it is great to define macro_rules 
 * if you want to use this function for other integer types.
 */
fn convert_to_SI(n: &u64) -> String {
    
}

fn format_info(info: &SystemInfo){

}

pub fn post_webhook(info: SystemInfo) -> Result<(), String> {
    let client = reqwestClient::new();

    let payload = Post {
        username: "tester".to_string(),
        embeds: vec![Embed {
            title: "Test".to_string(),
            fields: vec![
                Field {name: "f1".to_string(), value: "v1".to_string(), inline: true},
                Field {name: "f2".to_string(), value: "v2".to_string(), inline: true}
            ]
        }]
    };

    if let Ok(res) = client.post(WHURL).json(&payload).send() {
        if res.status().as_u16() == 204 {
            Ok(())
        }
        else {
            Err(format!{"Resp: {}", res.text().unwrap()})
        }
    }
    else {
        Err("Could not send the post request".to_string())
    }
}
