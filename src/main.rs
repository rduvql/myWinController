use std::{time::Duration, error::Error};
use winapi::um::winuser::{INPUT_u, INPUT, INPUT_KEYBOARD, KEYBDINPUT, SendInput, VK_MEDIA_NEXT_TRACK, VK_MEDIA_PREV_TRACK, VK_MEDIA_PLAY_PAUSE, VK_VOLUME_DOWN, VK_VOLUME_UP, VK_CONTROL};
use rumqttc::{MqttOptions, QoS, AsyncClient};

fn press_key(key: u16, flag: u32) /*-> Result<(), Error>*/ {
    
    let mut input_u: INPUT_u = unsafe { std::mem::zeroed() };
    
    unsafe {
        *input_u.ki_mut() = KEYBDINPUT {
            wVk: key,
            dwExtraInfo: 0,
            wScan: 0,
            time: 0,
            dwFlags: flag
        }
    }
    
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: input_u
    };
    
    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    };
    // Ok(())
}


async fn subscribe() {
    
    let mut mqtt_options = MqttOptions::new("myWinController", "192.168.0.11", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));
    
    let (mut client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    client.subscribe("msi/media/+", QoS::AtMostOnce).await.unwrap();
    
    loop {
        match eventloop.poll().await {
            Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish))) => {
                
                println!("message: {:?}", publish);
                // let s = str::from_utf8(&publish.payload).unwrap();
                
                match publish.topic.as_str() {

                    "msi/media/next" => {
                        press_key(VK_CONTROL as u16, 2);
                        press_key(VK_MEDIA_NEXT_TRACK as u16, 0);
                    },
                    "msi/media/prev" => {
                        press_key(VK_CONTROL as u16, 2);
                        press_key(VK_MEDIA_PREV_TRACK as u16, 0);
                    },
                    "msi/media/play_pause" => {
                        press_key(VK_CONTROL as u16, 2);
                        press_key(VK_MEDIA_PLAY_PAUSE as u16, 0);
                    }
                    "msi/media/vol_down" => {
                        press_key(VK_CONTROL as u16, 2);
                        press_key(VK_VOLUME_DOWN as u16, 0);
                    }
                    "msi/media/vol_up" => {
                        press_key(VK_CONTROL as u16, 2);
                        press_key(VK_VOLUME_UP as u16, 0);
                    }
                    _ => {}
                }
            },
            Ok(_) => {},
            Err(e) => {
                println!("err, {}", e);
            }
        }
        
        
        // println!("Received = {:?}", notification);
    }
}

#[tokio::main]
// async fn main() ->Result<(), Box<dyn Error>> {
async fn main() {
    
    subscribe().await;
    
    // Ok(())
}