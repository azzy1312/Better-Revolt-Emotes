use crate::{
    Auth, RMessage, rev_message_clean, send, sendas};

use serde::{Serialize, Deserialize};

use crate::fs_str;

use crate::lib::message::Masquerade;

use crate::rev_user;

use crate::lib::message::RMessagePayload;

use crate::rev_send;

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
    let message = rev_message_clean(message_in.clone()).await;

    let content_vec =  content.as_ref().expect("failed to split vec").split(' ').collect::<Vec<&str>>();

    let mut content_min1 = String::new();

    for x in 0..content_vec.len() -1 {
        content_min1 += &format!("{} ", content_vec[x + 1])
    };


    // validity check
    if content_vec[0] != "?bre" {
        return

    }else if content_vec.len() < 2 {
        return
    };

    // head match

    if content_vec[1] != "e" {

        let payload = match &content_vec[1] as &str {
            "ver" => "balls",
            _     => "invalid command"
        };

        send(details, message_in.clone(), payload.to_string()).await;

    } else {

        emoji_engine(details, message_in, content_vec).await;
    };


}

async fn emoji_engine(details: Auth, input_message: RMessage, content_vec:Vec<&str>){

    let masq: Masquerade;

    if input_message.masquerade == None {

        let user = rev_user(details.clone(), input_message.clone().author).await;
        match user {

            Ok(_) => {},
            Err(e) => {println!("user not found"); return}
        };

        
    let user2 = user.unwrap().clone();
    let profile_image = user2.avatar.unwrap().id;


    let pfp = format!("https://autumn.revolt.chat/avatars/{profile_image}");

    let username = user2.username;

    masq = Masquerade {

        name: Some(username),
        avatar: Some(pfp),
        colour: None
    };

    } else {

        masq = Masquerade {

            name: input_message.masquerade.as_ref().unwrap().name.clone(),
            avatar: input_message.masquerade.as_ref().unwrap().avatar.clone(),
            colour: None
        };
    };

    let payload = RMessagePayload {

        content: input_message.content.clone(),
        attachments: None,
        replies: None,
        masquerade: Some(masq)
    };


    println!("{:?}", payload);
    rev_send(details,input_message,payload).await

    
}
   
