use criterion::{black_box, criterion_group, criterion_main, Criterion};

use api_actix_web::get_users;

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("benches");

    g.bench_function("get_users", |b| b.iter(|| black_box(get_users())));
    g.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
