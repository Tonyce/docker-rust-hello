// use std::env;

// // fn main() {
// //     let rust_env = env::var("RUST_ENV").unwrap_or("beta".to_owned());
// //     println!("rust_env {} .", rust_env);
// //     println!("Hello, world! {}", rust_env);
// // }

// #[async_std::main]
// async fn main() -> Result<(), std::io::Error> {
//     let rust_env = env::var("RUST_ENV").unwrap_or("beta".to_owned());
//     let mut app = tide::new();
//     app.at("/").get(|_| async { Ok("Hello, world!") });

//     let addr = "0.0.0.0:8080";
//     println!("rust_env: {}, addr: {}", rust_env, addr);
//     app.listen(addr).await?;
//     Ok(())
// }

/////////////////////////////////////////////////////////////
use async_std::task;
use futures::channel::mpsc;
use futures::{Future, StreamExt};
use std::pin::Pin;

struct Context {
    value: i32,
}

type ExecFuture<'a> = Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

// pub(crate) type ExecFn = Box<dyn FnOnce(&mut Context) -> ExecFuture + Send + 'static>;
pub(crate) type ExecFn =
    Box<dyn for<'a> FnOnce(&'a mut Context) -> ExecFuture<'a> + Send + 'static>;

#[async_std::main]
async fn main() {
    let (mut tx, mut rx) = mpsc::unbounded::<ExecFn>();
    task::spawn(async move {
        let mut ctx = Context { value: 0 };
        while let Some(f) = rx.next().await {
            f(&mut ctx).await;
        }
    });
    tx.start_send(Box::new(|ctx: &mut Context| {
        Box::pin(async move {
            ctx.value += 1;
        })
    }))
    .unwrap();
}
