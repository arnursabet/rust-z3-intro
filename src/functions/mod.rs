use z3::{ast::*, *};

pub fn run1() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    // integer sort (type)
    let int_sort = Sort::int(&ctx);
    // function declaration f: Z -> Z
    let f = FuncDecl::new(&ctx, "f", &[&int_sort], &int_sort);
    // f(x)
    let f_x = f.apply(&[&x]);
    // f(f(x))
    let f_f_x = f.apply(&[&f_x]).as_int().unwrap();
    // f(f(x)) == x
    let f_f_x_eq_x = f_f_x._eq(&x);
    // f(x) == y
    let f_x_eq_y = f_x.as_int().unwrap()._eq(&y);
    // x != y
    let x_not_eq_y = x._eq(&y).not();

    let solver = Solver::new(&ctx);
    solver.assert(&f_f_x_eq_x);
    solver.assert(&f_x_eq_y);
    solver.assert(&x_not_eq_y);

    println!("Result: {:?}", solver.check());

    println!("Solution:\n{:?}", solver.get_model().unwrap());
}

pub fn run2() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    // integer sort (type)
    let int_sort = Sort::int(&ctx);
    // function declaration f: Z -> Z
    let f = FuncDecl::new(&ctx, "f", &[&int_sort], &int_sort);
    // f(x)
    let f_x = f.apply(&[&x]);
    // f(f(x))
    let f_f_x = f.apply(&[&f_x]).as_int().unwrap();
    // f(f(x)) == x
    let f_f_x_eq_x = f_f_x._eq(&x);
    // f(x) == y
    let f_x_eq_y = f_x.as_int().unwrap()._eq(&y);
    // x != y
    let x_not_eq_y = x._eq(&y).not();

    let solver = Solver::new(&ctx);
    solver.assert(&f_f_x_eq_x);
    solver.assert(&f_x_eq_y);
    solver.assert(&x_not_eq_y);

    println!("Result: {:?}", solver.check());

    let model = solver.get_model().unwrap();

    println!(
        "Solution:\nf(f(x)) = {:?}\nf(x) = {:?}",
        model.eval(&f_f_x, true).unwrap(),
        model.eval(&f_x, true).unwrap()
    );
}
