#[macro_use]
extern crate batch;
extern crate env_logger;
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tokio;

use batch::{exchange, queue, Perform, WorkerBuilder};
use futures::Future;

#[derive(Serialize, Deserialize, Task)]
#[task_name = "batch::SayHello"]
#[task_routing_key = "hello-world"]
struct SayHello {
    to: String,
}

impl Perform for SayHello {
    type Context = ();

    fn perform(&self, _ctx: Self::Context) {
        println!("Hello {}", self.to);
    }
}

fn main() {
    env_logger::init();
    println!("Starting RabbitMQ worker example");
    let exchanges = vec![exchange("batch.example")];
    let queues = vec![queue("hello-world").bind("batch.example", "hello-world")];
    let worker = WorkerBuilder::new(())
        .connection_url("amqp://localhost/%2f")
        .exchanges(exchanges)
        .queues(queues)
        .task::<SayHello>()
        .build()
        .unwrap();
    tokio::run(
        worker
            .run()
            .map_err(|e| eprintln!("An error occured in the Worker: {}", e)),
    );
}
