use riker::actors::*;
use riker_default::DefaultModel;

#[derive(Clone, Debug)]
pub enum Protocol {
    Go,
    Ping,
    Pong
}

impl Into<ActorMsg<Protocol>> for Protocol {
    fn into(self) -> ActorMsg<Protocol> {
        ActorMsg::User(self)
    }
}

struct PingActor;

impl PingActor {
    pub fn actor() -> BoxActor<Protocol> {
        Box::new(PingActor)
    }
    pub fn props() -> BoxActorProd<Protocol> {
        Props::new(Box::new(PingActor::actor))
    }
}

impl Actor for PingActor {
    type Msg = Protocol;

    fn receive(&mut self,
               ctx: &Context<Protocol>,
               msg: Protocol,
               sender: Option<ActorRef<Protocol>>) {
        match msg {
            Protocol::Go => {
                let pong = ctx.actor_of(PongActor::props(), "pong").unwrap();
                pong.tell(Protocol::Ping, Some(ctx.myself()));
            },
            Protocol::Pong => {
                println!("Pong");
                ctx.stop(&ctx.myself());
            },
            _ => { panic!("Wrong message type!"); }
        }
    }
}

struct PongActor;

impl PongActor {
    pub fn actor() -> BoxActor<Protocol> {
        Box::new(PongActor)
    }
    pub fn props() -> BoxActorProd<Protocol> {
        Props::new(Box::new(PongActor::actor))
    }
}

impl Actor for PongActor {
    type Msg = Protocol;

    fn receive(&mut self,
               ctx: &Context<Protocol>,
               msg: Protocol,
               sender: Option<ActorRef<Protocol>>) {
        match msg {
            Protocol::Ping => {
                println!("Ping");
                sender.unwrap().tell(Protocol::Pong, Some(ctx.myself()));
            },
            _ => { panic!("Wrong message type!"); }
        }
    }
}

pub fn main() {
    let sys = ActorSystem::new(&DefaultModel::<Protocol>::new()).unwrap();
    let ping = sys.actor_of(PingActor::props(), "test-actor").unwrap();

    ping.tell(Protocol::Go, None);

    std::thread::sleep(std::time::Duration::from_secs(2));

}

