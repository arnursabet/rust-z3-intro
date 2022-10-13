use z3::{ast::*, *};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let a = Bool::new_const(&ctx, "a");
    let b = Bool::new_const(&ctx, "b");
    let c = Bool::new_const(&ctx, "c");
    let d = Bool::new_const(&ctx, "d");
    let e = Bool::new_const(&ctx, "e");
    let f = Bool::new_const(&ctx, "f");
    let g = Bool::new_const(&ctx, "g");
    let z = Bool::new_const(&ctx, "z");

    println!("Check 1:");

    install_check(
        &ctx,
        vec![
            &depends_on(&ctx, &a, vec![&b, &c, &z]),
            &depends_on(&ctx, &b, vec![&d]),
            &depends_on(
                &ctx,
                &c,
                vec![&Bool::or(&ctx, &[&c, &d]), &Bool::or(&ctx, &[&f, &g])],
            ),
            &conflict(&ctx, vec![&d, &e]),
            &conflict(&ctx, vec![&d, &g]),
            &a,
            &z,
        ],
    );

    println!("Check 2:");

    install_check(
        &ctx,
        vec![
            &depends_on(&ctx, &a, vec![&b, &c, &z]),
            &depends_on(&ctx, &b, vec![&d]),
            &depends_on(
                &ctx,
                &c,
                vec![&Bool::or(&ctx, &[&c, &d]), &Bool::or(&ctx, &[&f, &g])],
            ),
            &conflict(&ctx, vec![&d, &e]),
            &conflict(&ctx, vec![&d, &g]),
            &a,
            &z,
            &g,
        ],
    );
}
pub fn depends_on<'ctx>(
    ctx: &'ctx Context,
    pack: &Bool<'ctx>,
    deps: Vec<&Bool<'ctx>>,
) -> Bool<'ctx> {
    // create a vector of pack =>(implies) dep
    let deps = deps
        .iter()
        .map(|dep| Bool::and(ctx, &[&pack.implies(dep)]))
        .collect::<Vec<_>>();
    // return the vector in an 'and' clause
    // i.e. a => b AND a => c AND a => z
    // If user installs a, then user must install b, c, and z
    Bool::and(ctx, deps.iter().map(|d| d).collect::<Vec<_>>().as_slice())
}

pub fn conflict<'ctx>(ctx: &'ctx Context, packs: Vec<&Bool<'ctx>>) -> Bool<'ctx> {
    // create a vector of Not(d), Not(e)
    let packs = packs.iter().map(|p| p.not()).collect::<Vec<_>>();
    // return the vector in an 'or' clause
    // i.e. Not(d) OR Not(e)
    Bool::or(ctx, packs.iter().map(|p| p).collect::<Vec<_>>().as_slice())
}

pub fn install_check(ctx: &Context, problem: Vec<&Bool>) {
    let solver = Solver::new(ctx);

    problem.iter().for_each(|c| {
        solver.assert(c);
    });

    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            println!("{:?}", model);
        }
        _ => println!("Invalid installation profile"),
    }
}
