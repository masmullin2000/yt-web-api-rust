#[macro_use]
extern crate bencher;

use bencher::Bencher;

const ITERATIONS: usize = 1000;

pub struct User {
    pub(crate) Id: u16,
    pub(crate) Age: u16,
    pub(crate) First_Name: String,
    pub(crate) Last_Name: String,
    pub(crate) Framework: String,
}

pub fn get_users() -> Vec<User> {
    let mut users = Vec::with_capacity(1000);
    for index in 1..1001_u16 {
        users.push(User {
            Id: index,
            Age: 25,
            First_Name: format!("First_Name{}", index),
            Last_Name: format!("Last_Name{}", index),
            Framework: "Rust (actix-web)".to_owned(),
        })
    }
    users
}

fn tokio_blocking(bench: &mut Bencher) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    bench.iter(|| {
        let j = runtime.spawn(async {
            for _ in 0..ITERATIONS {
                tokio::task::spawn_blocking(|| {
                    let _ = get_users();
                })
                .await;
            }
        });
        runtime.block_on(j).unwrap();
    });
}

fn async_std_blocking(bench: &mut Bencher) {
    bench.iter(|| {
        async_std::task::block_on(async {
            for _ in 0..ITERATIONS {
                async_std::task::spawn_blocking(|| {
                    let _ = get_users();
                })
                .await;
            }
        });
    });
}

benchmark_group!(benches, tokio_blocking, async_std_blocking);
benchmark_main!(benches);
