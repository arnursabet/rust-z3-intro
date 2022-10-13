use z3::{ast::*, *};

pub fn run1() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Real::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let a = Real::new_const(&ctx, "a");
    let c = Real::new_const(&ctx, "c");
    let s = Int::new_const(&ctx, "s");

    let one = Int::from_i64(&ctx, 1);
    println!(
        "{:?}",
        Real::add(&ctx, &[&x, &y.to_real(), &one.to_real(), &a, &s.to_real()])
    );
    println!("{:?}", Int::add(&ctx, &[&y, &c.to_int()]));
}

pub fn run2() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Real::new_const(&ctx, "x");
    let y = Real::new_const(&ctx, "y");

    let large_real = Real::from_real_str(&ctx, "10000000000000000000000", "1").unwrap();
    let large_real_2 = Real::from_real_str(&ctx, "20000000000000000", "1").unwrap();

    let solver = Solver::new(&ctx);

    solver.assert(&Real::add(&ctx, &[&x, &large_real])._eq(&y));
    solver.assert(&y.gt(&large_real_2));
    println!("{:?}", solver.check());

    println!("Solution: {:#?}", solver.get_model().unwrap());
}
