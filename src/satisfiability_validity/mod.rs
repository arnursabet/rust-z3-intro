use z3::{ast::*, *};

pub fn run1() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let p = Bool::new_const(&ctx, "p");
    let q = Bool::new_const(&ctx, "q");

    // DeMorgan's Law: p AND q == Not(Not(p) OR Not(q))
    let demorgan = Bool::and(&ctx, &[&p, &q])._eq(&Bool::or(&ctx, &[&p.not(), &q.not()]).not());

    println!("{}", demorgan.to_string());

    println!("Proving demorgan...");
    prove(&ctx, &demorgan);
}

pub fn prove(ctx: &Context, f: &Bool) {
    let solver = Solver::new(ctx);
    solver.assert(&f.not());

    match solver.check() {
        SatResult::Unsat => println!("proved"),
        SatResult::Sat => println!("failed to prove"),
        _ => println!("Unknown"),
    }
}
