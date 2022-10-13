use z3::{ast::*, *};

pub fn run1() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = BV::new_const(&ctx, "x", 16);
    let bv_two = BV::from_i64(&ctx, 2, 16);

    println!("{:?}", x.bvadd(&bv_two));

    let neg_one = BV::from_i64(&ctx, -1, 16);
    // -1 is equal to 65535 for 16-bit integers
    println!("{:?}", neg_one.as_i64().unwrap());

    let a = BV::from_i64(&ctx, -1, 16);
    let b = BV::from_i64(&ctx, 65535, 16);

    println!("{:?}", a._eq(&b).simplify());

    let a = BV::from_i64(&ctx, -1, 32);
    let b = BV::from_i64(&ctx, 65535, 32);

    println!("{:?}", a._eq(&b).simplify());
}

pub fn run2() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = BV::new_const(&ctx, "x", 32);
    let y = BV::new_const(&ctx, "y", 32);
    let two = BV::from_i64(&ctx, 2, 32);
    let zero = BV::from_i64(&ctx, 0, 32);

    let solver = Solver::new(&ctx);
    solver.push();
    // x + y = 2
    solver.assert(&BV::bvadd(&x, &y)._eq(&two));
    // x > 0 (signed)
    solver.assert(&x.bvsgt(&zero));
    // y > 0 (signed)
    solver.assert(&y.bvsgt(&zero));
    println!("Result 1: {:?}", solver.check());
    println!(
        "Solution 1: x = {:?}, y = {:?}",
        solver
            .get_model()
            .unwrap()
            .eval(&x, true)
            .unwrap()
            .as_i64() // converting to int for readability
            .unwrap(),
        solver
            .get_model()
            .unwrap()
            .eval(&y, true)
            .unwrap()
            .as_i64()
            .unwrap()
    );
    solver.pop(1);

    solver.push();

    // x & y == ~y  (& is bv and), (~ is bv not)
    solver.assert(&x.bvand(&y)._eq(&y.bvnot()));
    println!("Result 2: {:?}", solver.check());
    println!(
        "Solution 2: x = {:?}, y = {:?}",
        solver
            .get_model()
            .unwrap()
            .eval(&x, true)
            .unwrap()
            .as_i64()
            .unwrap(),
        solver
            .get_model()
            .unwrap()
            .eval(&y, true)
            .unwrap()
            .as_i64()
            .unwrap()
    );
    solver.pop(1);

    solver.push();
    // unsigned version of <
    solver.assert(&x.bvult(&zero));
    println!("Result 3: {:?}", solver.check());
}

pub fn run3() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = BV::new_const(&ctx, "x", 32);
    let two = BV::from_i64(&ctx, 2, 32);
    let three = BV::from_i64(&ctx, 3, 32);
    let twenty_four = BV::from_i64(&ctx, 24, 32);

    let solver = Solver::new(&ctx);
    solver.push();
    solver.assert(&x.bvashr(&two)._eq(&three));
    println!("Result 1: {:?}", solver.check());
    println!(
        "Solution 1: x = {:?}",
        solver
            .get_model()
            .unwrap()
            .eval(&x, true)
            .unwrap()
            .as_i64()
            .unwrap()
    );
    solver.pop(1);

    solver.push();
    solver.assert(&x.bvshl(&two)._eq(&three));
    println!("Result 2: {:?}", solver.check());
    solver.pop(1);

    solver.push();
    solver.assert(&x.bvshl(&two)._eq(&twenty_four));
    println!("Result 3: {:?}", solver.check());
    println!(
        "Solution 3: x = {:?}",
        solver
            .get_model()
            .unwrap()
            .eval(&x, true)
            .unwrap()
            .as_i64()
            .unwrap()
    );
}
