use crate::systeminfo::SystemInfo;
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
 *
 * Return format is "x.xx[SIPrefix][Unit]".
 * Be careful about x.xx is rounded because of formatting {:.2}
 */
#[allow(non_snake_case)]
fn convert_to_SI(n: &u64, unit: &str) -> String {
    let SI = vec!["", "K", "M", "G", "T", "P"];
    let mut index = 0;
    while n / 1000_u64.pow(index) >= 1000 {
        index += 1;
    }

    format!{
        "{value:.2}{prefix}{value_unit}",
        value = *n as f64 / (1000.0_f64).powf(index.into()),
        prefix = SI[index as usize],
        value_unit = unit
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_kilo_byte() {
        assert_eq!(convert_to_SI(&1000_u64, "B"), "1.00KB");
    }

    #[test]
    fn one_point_twenty_three_kilo_byte() {
        assert_eq!(convert_to_SI(&1230_u64, "B"), "1.23KB");
    }

    #[test]
    fn kilo_byte_rounded() {
        assert_eq!(convert_to_SI(&1235_u64, "B"), "1.24KB");
    }

    #[test]
    fn normal_lets_note_used() {
        assert_eq!(convert_to_SI(&6016638976_u64, "B"), "6.02GB");
    }

    #[test]
    fn normal_lets_note_free() {
        assert_eq!(convert_to_SI(&2242838528_u64, "B"), "2.24GB");
    }

    #[test]
    fn terabyte() {
        assert_eq!(convert_to_SI(&1099511627776_u64, "B"), "1.10TB");
    }
}
