use crate::{
    Auth, RMessage, rev_message_clean, send, sendas};

use serde::{Serialize, Deserialize};

use crate::fs_str;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageConf {

    pub enabled: bool
}


// main message engine 
pub async fn message_process(details: Auth, message_in: RMessage) {

    let conf = fs_str("config/message.json").expect("failed to read config/message.json\n{e}");

    let message: MessageConf = serde_json::from_str(&conf)
            .expect("Failed to deser message.json");


    if message.enabled == false {
        return
    };

    let content = message_in.content.clone();
    // validity test
    if content == None {
        return
    }else if message_in.author == details.bot_id {
        return
    };
    let message = rev_message_clean(message_in).await;

    let content_vec =  content.as_ref().expect("failed to split vec").split(' ').collect::<Vec<&str>>();

    let mut content_min1 = String::new();

    for x in 0..content_vec.len() -1 {
        content_min1 += &format!("{} ", content_vec[x + 1])
    };


    if content_vec[0] != "?bre" {return};

    if content_vec[1] == "ver" {
        send(details.clone(), message.clone(),"**Version**\nReywen: `2`\nRevX: `2`\nBetter Revolt Emotes: `1.0`".to_string()).await;
    } else{ 

    let returner = match &content_vec[1] as &str {

        "sad" | "sadness" => "https://autumn.revolt.chat/attachments/NIoYkEvZZbp3vU17CiPqdhkBNndsxfk7de1iiJim-G/sadness.jpg",
        _ => "placeholder",
    };
     send(details, message, format!("[]({returner})")).await
    };
    


    // "ver" | "version" => "**Version**\nReywen: `2`\nRevX: `2`\nBetter Revolt Emotes: `1.0`",

    

}


