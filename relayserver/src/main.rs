use actix::*;
use actix_web::*;
use actix_ws::*;
use futures_util::StreamExt as _;
use std::sync::Mutex;

struct HostDevice {
    name: String,
    client_addrs: Vec<Addr<ClientDevice>>,
}

struct ClientDevice {
    name: String,
    host_addr: Option<Addr<HostDevice>>,
}

impl Actor for HostDevice {
    type Context = Context<Self>;
}

impl Actor for ClientDevice {
    type Context = Context<Self>;
}

struct AppState {
    hosts: Mutex<Vec<Addr<HostDevice>>>,
    clients: Mutex<Vec<Addr<ClientDevice>>>,
}

async fn test() -> impl Responder {
    println!("test() called");
    "this is a triumph"
}

// register an actor (device) + create context
async fn register(
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    println!("register() called");
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    let meow = text.parse::<String>().unwrap();
                    println!("parsed text: {}", meow.as_str());
                    match meow.as_str() {
                        "host" => {
                            let ctx = Context::<HostDevice>::new();
                            let actor = HostDevice {
                                name: "meow".to_string(),
                                client_addrs: vec![],
                            };
                            let addr = ctx.run(actor);
                            println!("registered {addr:?}");

                            let mut host_list = data.hosts.lock().unwrap();
                            host_list.push(addr);
                        }
                        "client" => {
                            let ctx = Context::<ClientDevice>::new();
                            let actor = ClientDevice {
                                name: "mrrp".to_string(),
                                host_addr: None,
                            };
                            let addr = ctx.run(actor);

                            let mut client_list = data.clients.lock().unwrap();
                            client_list.push(addr);
                        }
                        _ => {
                            println!("register(): parsed string does not match");
                        }
                    }
                }
                _ => {
                    println!("register(): received msg is not text");
                }
            }
        }
    });

    println!("register() -> {res:?}");
    Ok(res)
}

async fn connect(
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    println!("connect() called");
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    let meow = text.parse::<String>().unwrap();
                    println!("{meow}");
                    let mut host_list = data.hosts.lock().unwrap();
                }
                _ => {
                    println!("connect(): received msg is not text");
                }
            }
        }
    });

    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        hosts: Mutex::new(vec![]),
        clients: Mutex::new(vec![]),
    });

    meow();

    HttpServer::new(move || {
        App::new().app_data(data.clone()).service(
            web::scope("/api")
                .route("/test", web::get().to(test))
                .route("/register", web::get().to(register))
                .route("/connect", web::get().to(connect)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn meow() {}