use teloxide::prelude::*;
use teloxide::types::MediaKind;
use teloxide::types::MediaText;
use teloxide::types::MediaVoice;
use teloxide::types::MessageKind;
use teloxide::types::User;

use teloxide::net::Download;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

use regex::Regex;

use featurebot::core::command;

async fn send_text(text: &str) -> String {
    let command = text
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut cmd = String::new();
    let mut value: Option<String> = None;

    if command.len() > 1 {
        cmd = command[0].clone();
        value = Some(command[1].clone());
    } else {
        cmd = command[0].clone();
    }

    let commandi = command::Cmds {
        cmd: cmd,
        value: value,
    };

    let result = commandi.run().await;

    //if let Ok() =result
    if result.is_ok() {
        return result.unwrap();
    }

    "Errore di qualche tipo".to_string()
}

async fn speech_to_text(dati: String) -> String {
    /*
    let file_path = format!("https://api.telegram.org/file/bot{}/{}", token, voce);

    // Scarica il file audio
    let mut file = File::open(file_path).await.unwrap();
    let mut reader = BufReader::new(file);

    // Leggi il file audio in un buffer
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).await.unwrap();

    println!("buffer:{:#?}",buffer);
    /*
        // Invia il buffer all'API locale
    // Sostituisci "YOUR_LOCAL_API_URL" con l'URL della tua API locale
    let response = reqwest::Client::new()
        .post("YOUR_LOCAL_API_URL")
        .body(buffer)
        .send()
        .await
        .unwrap();

    // Controlla la risposta dell'API locale
    if response.status().is_success() {
        // API locale ha avuto successo
        ctx.reply("Nota vocale inviata correttamente all'API locale").await.unwrap();
    } else {
        // Errore nell'invio all'API locale
        ctx.reply(format!("Errore nell'invio della nota vocale all'API locale: {}", response.status())).await.unwrap();
    }
    */
    */
    let audio_traslate = format!("Audio con nome {}", dati);
    let response = send_text(audio_traslate.as_ref()).await;
    response
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        //println!("{:#?}",msg.chat.kind);

        let user: User = match msg.from() {
            Some(user) => user.clone(),
            None => {
                log::warn!("User information not available");
                return respond(());
            }
        };

        // Estrai il nome dell'utente
        let user_name = user.mention();

        //println!("{:#?}",user_name);
        //let user_name = msg.from().user_id;

        //message => kind => media_kind
        if user_name == Some("@metalneox".to_string()) {
            let media_kind = msg.kind.clone();
            //println!("->{:#?}",media_kind);
            match media_kind {
                MessageKind::Common(ref msg2) => match &msg2.media_kind {
                    MediaKind::Text(data) => {
                        let result = send_text(data.text.as_ref()).await;
                        bot.send_message(msg.chat.id, result).await?
                    }
                    MediaKind::Voice(data) => {
                        let voice = msg.voice().unwrap();

                        //println!("->{:#?}",voice);
                        let file_id = &voice.file.id;
                        //println!("->{:#?}",&voice.file.path);

                        let file = bot.get_file(file_id).await?;
                        //println!("{:#?}",file.path);

                        //Regex per togliere inizio
                        let regex = Regex::new(r"voice/(.*)").unwrap();
                        let file_name = regex.replace(&file.path, "$1");

                        //Salvo file
                        let path_file = format!("./tmp/{}", file_name);
                        let mut dst = File::create(path_file.clone()).await?;
                        bot.download_file(&file.path, &mut dst).await?;

                        let result = speech_to_text(path_file).await;

                        bot.send_message(msg.chat.id, result).await?
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            };
        } else {
            bot.send_message(msg.chat.id, "Sorry only administrator can use it")
                .await?;
        }

        //bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
