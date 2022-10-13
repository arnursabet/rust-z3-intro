use z3::{ast::*, *};

pub fn run1() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    let two = Int::from_i64(&ctx, 2);
    let ten = Int::from_i64(&ctx, 10);
    let eleven = Int::from_i64(&ctx, 11);

    let solver = Solver::new(&ctx);
    solver.assert(&x.gt(&ten)); // x > 10

    // y = x + 2
    solver.assert(&y._eq(&Int::add(&ctx, &[&x, &two])));
    println!("{:?}", solver);

    println!("Solving constraints in the solver ...");
    println!("Result 1: {:?}", solver.check());

    println!("Create a new scope...");
    solver.push();
    solver.assert(&y.lt(&eleven));
    println!("Result 2: {:?}", solver.check());

    println!("Restoring state...");
    solver.pop(1);
    println!("Solving restored set of constraints...");
    println!("Result 3: {:?}", solver.check());
}

pub fn run2() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Real::new_const(&ctx, "x");
    let two = Real::from_real(&ctx, 2, 1);
    let three = Real::from_real(&ctx, 3, 1);

    let solver = Solver::new(&ctx);
    solver.assert(&two.power(&x)._eq(&three));

    println!("Result: {:?}", solver.check());
}

pub fn run3() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Real::new_const(&ctx, "x");
    let y = Real::new_const(&ctx, "y");
    let z = Real::new_const(&ctx, "z");

    let one = Real::from_real(&ctx, 1, 1);
    let three = Real::from_real(&ctx, 3, 1);
    let ten = Real::from_real(&ctx, 10, 1);

    let solver = Solver::new(&ctx);

    solver.assert(&x.gt(&one)); // x > 1
    solver.assert(&y.gt(&one)); // y > 1

    // x + y > 3
    solver.assert(&Real::add(&ctx, &[&x, &y]).gt(&three));
    // z - x < 10
    solver.assert(&Real::sub(&ctx, &[&z, &x]).lt(&ten));

    println!("Result: {:?}", solver.check());

    let model = solver.get_model().unwrap();

    println!("Evaluating model...");
    println!(
        "value of x = {:?}",
        model.eval(&x, true).unwrap().as_real().unwrap()
    );
    println!(
        "value of y = {:?}",
        model.eval(&y, true).unwrap().as_real().unwrap()
    );
}
