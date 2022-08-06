use criterion::{black_box, criterion_group, criterion_main, Criterion};

use api_actix_web::*;

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("benches");

    g.bench_function("make_user", |b| b.iter(|| black_box(make_user(15))));
    g.bench_function("get_users", |b| b.iter(|| black_box(get_users())));
    g.bench_function("get_resp", |b| b.iter(|| black_box(get_resp())));
    g.bench_function("fill_json_string", |b| {
        let idx = 25;
        let user = api_actix_web::models::User::new(
            25,
            25,
            format!("Firstname{idx}"),
            format!("LastName{idx}"),
        );
        b.iter(|| {
            let mut fill = String::with_capacity(128);
            black_box(user.fill_json_string(&mut fill))
        })
    });
    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
