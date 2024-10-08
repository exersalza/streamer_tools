use std::borrow::Cow;
use std::net::SocketAddr;
use std::ops::{BitXor, ControlFlow, Not};

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    headers,
    response::IntoResponse, TypedHeader,
};
//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::Path;
use std::sync::Mutex;
use std::time;
use axum::extract::ws::CloseFrame;
//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};
use lazy_static::lazy_static;
use frontend::components::timer::{Time, Timer};
use shared::get_random_id;
use crate::SQL;


lazy_static! {
    static ref SUB_COUNTER: Mutex<u16> = Mutex::new(0);
    static ref SUB_PAUSED: Mutex<bool> = Mutex::new(false);
}

pub async fn ws_handler(Path(t): Path<String>,
                        ws: WebSocketUpgrade,
                        user_agent: Option<TypedHeader<headers::UserAgent>>,
                        ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse { // fuck around find out
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    log::info!("Useragent {}, at {}", user_agent, addr);

    ws.on_upgrade(move |socket| handle_socket(socket, addr, t))
}

// don't touch my clogs

// thanks to the guy that wrote the example in axum/examples
async fn handle_socket(mut socket: WebSocket, who: SocketAddr, _type: String) {
    // send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        log::info!("Pinged {who}...");
    } else {
        log::info!("Could not send ping {who}!");
        return;
    }

    let _id = get_random_id();


    // we need something to defer what type the timer is, if it's an subathon timer we also have
    // to create a new thread that handles twitch and stuff
    let (mut tx, mut rx) = socket.split();

    // this thread kills everything somehow???? also when we get a close, of course
    tokio::spawn(async move {
        loop {
            if let Some(Ok(msg)) = rx.next().await {
                match &msg {
                    Message::Text(t) => {
                        if t == "flip_pause" {
                            flip_paused().await;
                        }
                    },
                    Message::Close(c) => {
                        if let Some(cf) = c {
                            log::info!(
                                ">>> {} sent close with code {} and reason `{}`",
                                who, cf.code, cf.reason
                            );
                        } else {
                            log::info!(">>> {who} somehow sent close message without CloseFrame");
                        }
                        break;
                    },
                    _ => ()
                };
            }
        }
    });

    // Spawn a task that will push several messages to the client (does not matter what client does)
    tokio::spawn(async move {
        let mut last: u64 = 0;
        loop {
            // short timeout so the thread can actually process whats going on
            tokio::time::sleep(time::Duration::from_millis(69)).await;

            let sys_time = time::SystemTime::now()
                                        .duration_since(time::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs();

            if get_paused().await {
                continue;
            }

            // hopefully prevent added time that comes from processing
            if last == sys_time {
                continue;
            }

            last = sys_time;

            set_thread_id(_id).await;

            // In case of any websocket error, we exit.
            let mut text: String = String::from("");

            // check if we have the subathon timer and trait it different.

            if _type == "sub" /* && sub == _id */ { // maybe being renamed later in progress
                text = format!("{}", dec_time(6969, _id));
            }

            // send a tick to every timer every seconds, functionality is handled inside the timer itself
            if tx.send(Message::Text(text)).await.is_err() {
                log::info!("{who} broke connection");
                break;
            }
        }

        log::info!("Sending close to {who}...");
        if let Err(e) = tx
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from("Goodbye"),
            })))
            .await
        {
            set_thread_id(0).await;
            log::error!("Could not send Close due to {e}, probably it is ok? {_id}");
        }
    });
}


// most complex function i wrote in the whole project
async fn set_thread_id(id: u16) {
    let mut sub = SUB_COUNTER.lock().unwrap();

    // set new thread id if there is none, else reset
    if *sub == 0 && id != 0 && *sub != id {
        *sub = id;
        return;
    }

    if id == 0 {
        *sub = 0;
    }
}

fn dec_time(id: i32, thread_id: u16) -> i32 {
    let sql = SQL.lock().expect("Can't lock");
    let sub = SUB_COUNTER.lock().unwrap().clone();
    let mut time = sql.get_time(id).unwrap().timer.to_seconds();

    if time <= 0 {
        return time;
    }

    time -= 1;
    if sub != thread_id {
        return time;
    }

    let timer = Timer {
        timer: Time::from(time),
        id,
        browser: false,
    };
    sql.create_timer(&timer);

    time
}

async fn get_paused() -> bool {
    let rret = SUB_PAUSED.lock().expect("cAN#T LOCK");
    *rret
}

async fn flip_paused() {
    let mut flipper = SUB_PAUSED.lock().expect("Can't lock");
    *flipper = flipper.not();
}