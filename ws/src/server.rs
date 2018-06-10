use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ws::{listen, Handler, Handshake, Sender, Result, Message, CloseCode, Error};
use ws::util::Token;

struct Server {
    out: Sender,
    room: String,
    sines: Rc<RefCell<HashMap<Token, (String, String)>>>,
    connections: Rc<RefCell<HashMap<Token, (String, Sender)>>>,
}

impl Server {
    fn list_connections(&mut self) {
        let json_seq = self.connections.borrow().values()
            .map(|&(ref room, _)| format!("\"{}\"", room))
            .collect::<Vec<_>>()
            .join(",");

        for &(ref url, ref out) in self.connections.borrow().values() {
            if url == "/list" {
                let _ = out.send(Message::text(format!("[{}]", json_seq)));
            }
        }
    }

    fn cleanup(&mut self)  {
        let token = self.out.token();
        (*self.sines.borrow_mut())
            .remove(&token);
        (*self.connections.borrow_mut())
            .remove(&token);

        self.list_connections();
    }
}

impl Handler for Server {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        let room = handshake.request.resource();
        let token = self.out.token();

        self.room = room.to_string();
        (*self.connections.borrow_mut()).insert(
            token,
            (room.to_string(), self.out.clone())
        );

        self.list_connections();
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Ok(text) = msg.into_text() {
            (*self.sines.borrow_mut())
                .insert(self.out.token(), (self.room.clone(), text));
            let sines = self.sines.borrow();
            let json_seq = sines.values()
                .filter(|&&(ref room, _)| *room == self.room)
                .fold("".to_string(), |join, &(_, ref sine)| {
                    if join.len() == 0 {
                        format!("{}", sine)
                    } else {
                        format!("{},{}", join, sine)
                    }
                });

            for t in self.connections.borrow().values().into_iter() {
                if t.0 == self.room {
                    let _ = t.1.send(Message::text(format!("[{}]", json_seq)));
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        self.cleanup();
    }

    fn on_error(&mut self, _: Error) {
        self.cleanup();
    }
}

pub fn serve(port: u16) {
    use std::process;

    let ws_host = format!("0.0.0.0:{}", port);
    let sines = Rc::new(RefCell::new(HashMap::new()));
    let connections = Rc::new(RefCell::new(HashMap::new()));

    println!("Serving web sockets on {}", &ws_host);
    let listener = listen(ws_host, |out| {
        Server {
            out: out,
            room: "".to_string(),
            sines: sines.clone(),
            connections: connections.clone(),
        }
    });

    match listener {
        Ok(_) => (),
        Err(error) => {
            println!("Error in web sokcets server: {}", error);
            process::exit(1);
        }
    };
}
