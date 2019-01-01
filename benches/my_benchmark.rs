#[macro_use]
extern crate criterion;
extern crate haddock;

use criterion::Criterion;

fn bench_generate_moves(c: &mut Criterion) {
    let fen = "r7/pp4k1/2p1b1p1/8/7r/1P1Q4/P5PP/5R1K w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = haddock::parser::parse_fen(&fen_parts).unwrap();

    c.bench_function("generate moves", move |b| b.iter(|| haddock::generator::generate_legal_moves(&position)));
}

fn bench_apply_move(c: &mut Criterion) {
    let fen = "r7/pp4k1/2p1b1p1/8/7r/1P1Q4/P5PP/5R1K w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = haddock::parser::parse_fen(&fen_parts).unwrap();

    let move_ = haddock::move_::Move_::from_str("d3g6").unwrap();

    //applying the move includes cloning the position
    c.bench_function("apply move", move |b| b.iter(|| {
        let mut p = position.clone();
        p.apply_move(move_)
    }));
}

fn bench_evaluate_position(c: &mut Criterion) {
    let fen = "r7/pp4k1/2p1b1p1/8/7r/1P1Q4/P5PP/5R1K w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = haddock::parser::parse_fen(&fen_parts).unwrap();

    c.bench_function("evaluate position", move |b| b.iter(|| haddock::evaluation::evaluate(&position)));
}


criterion_group!(
    benches, 
    bench_generate_moves,
    bench_apply_move,
    bench_evaluate_position);
criterion_main!(benches);