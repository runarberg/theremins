use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ws::{listen, Handler, Handshake, Sender, Result, Message, CloseCode, Error};

struct Server {
    out: Sender,
    room: String,
    sines: Rc<RefCell<HashMap<usize, (String, String)>>>,
    connections: Rc<RefCell<HashMap<usize, (String, Sender)>>>,
}

impl Server {
    fn list_connections(&mut self) {
        let json_seq = self.connections.borrow().values()
            .map(|&(ref room, _)| format!("\"{}\"", room))
            .collect::<Vec<_>>()
            .join(",");

        for t in self.connections.borrow().values() {
            if t.0 == "/list" {
                let _ = t.1.send(Message::text(format!("[{}]", json_seq)));
            }
        }
    }

    fn cleanup(&mut self)  {
        let token = self.out.token();
        (*self.sines.borrow_mut())
            .remove(&token.as_usize());
        (*self.connections.borrow_mut())
            .remove(&token.as_usize());

        self.list_connections();
    }
}

impl Handler for Server {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        if let Ok(resource) = handshake.request.resource() {
            let room = resource.to_string();
            let out = self.out.clone();
            self.room = resource.to_string();
            (*self.connections.borrow_mut())
                .insert(
                    self.out.token().as_usize(),
                    (room, out)
                );
        }

        self.list_connections();
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Ok(text) = msg.into_text() {
            (*self.sines.borrow_mut())
                .insert(self.out.token().as_usize(), (self.room.clone(), text));
            let sines = self.sines.borrow();
            let json_seq = sines.values()
                .filter(|&&(ref room, _)| *room == self.room)
                .map(|&(_, ref sine)| sine.as_str())
                .collect::<Vec<_>>()
                .join(",");

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

pub fn serve(ws_host: &str) {
    use std::process;

    let sines = Rc::new(RefCell::new(HashMap::new()));
    let connections = Rc::new(RefCell::new(HashMap::new()));

    println!("Serving web sockets on {}", &ws_host);

    let _guard = if let Err(error) = listen(ws_host, |out| {
        Server {
            out: out,
            room: "".to_string(),
            sines: sines.clone(),
            connections: connections.clone(),
        }
    }) {
        println!("Error in web socets server: {}", error);
        process::exit(1);
    };
}
