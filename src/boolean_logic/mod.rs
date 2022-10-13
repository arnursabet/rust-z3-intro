use z3::{ast::*, *};

pub fn run1() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let p = Bool::new_const(&ctx, "p");
    let q = Bool::new_const(&ctx, "q");
    let r = Bool::new_const(&ctx, "r");

    let solver = Solver::new(&ctx);

    solver.assert(&p.implies(&q)); // p implies q
    solver.assert(&r.iff(&q.not())); // r <=> not q
    solver.assert(&Bool::or(&ctx, &[&p.not(), &r])); // not p or r

    assert_eq!(SatResult::Sat, solver.check());

    let model = solver.get_model().unwrap();
    println!("{}", model.to_string());
}

pub fn run2() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let p = Bool::new_const(&ctx, "p");
    let x = Real::new_const(&ctx, "x");
    let two = Real::from_real(&ctx, 2, 1);
    let five = Real::from_real(&ctx, 5, 1);
    let ten = Real::from_real(&ctx, 10, 1);

    let solver = Solver::new(&ctx);
    // x < 5 OR x > 10
    solver.assert(&Bool::or(&ctx, &[&x.lt(&five), &x.gt(&ten)]));
    // p OR x^2 = 2
    solver.assert(&Bool::or(&ctx, &[&p, &x.power(&two)._eq(&two)]));
    solver.assert(&p.not()); // not p

    assert_eq!(SatResult::Sat, solver.check());

    let model = solver.get_model().unwrap();
    println!("{}", model.to_string());
}
