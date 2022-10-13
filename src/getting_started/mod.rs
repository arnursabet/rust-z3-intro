use z3::{ast::*, *};

pub fn run1() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    let solver = Solver::new(&ctx);

    let two = Int::from_i64(&ctx, 2);
    let ten = Int::from_i64(&ctx, 10);
    let seven = Int::from_i64(&ctx, 7);
    let x_plus_y = Int::add(&ctx, &[&x, &y]);

    solver.assert(&x.gt(&two));
    solver.assert(&y.lt(&ten));
    solver.assert(&x_plus_y._eq(&seven));

    assert_eq!(SatResult::Sat, solver.check());

    let model = solver.get_model().unwrap();
    println!("{}", model.to_string());
}

pub fn run2() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    let two = Int::from_i64(&ctx, 2);
    let two_x = Int::mul(&ctx, &[&two, &x]);
    // x + y + 2x + 2
    let x_plus_y_plus_two_x_plus_two = Int::add(&ctx, &[&x, &y, &two_x, &two]);

    println!("{:#?}", x_plus_y_plus_two_x_plus_two);
    println!("{:#?}", x_plus_y_plus_two_x_plus_two.simplify());

    let y_plus_x_plus_two = Int::add(&ctx, &[&y, &x, &two]);
    // x < y + x + 2
    let x_lt_y_plus_x_plus_two = x.lt(&y_plus_x_plus_two);

    println!("{:#?}", x_lt_y_plus_x_plus_two);
    println!("{:#?}", x_lt_y_plus_x_plus_two.simplify());
}

pub fn run3() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    let three = Int::from_i64(&ctx, 3);
    let n = Int::add(&ctx, &[&x, &y]).ge(&three);

    println!("num args: {}", n.num_children());
    println!("children: {:?}", n.children());
    println!("1st child: {:?}", n.nth_child(0).unwrap());
    println!("2nd child: {:?}", n.nth_child(1).unwrap());
    println!("operator: {:?}", n.decl());
    println!("op name: {:?}", n.decl().name());
}

pub fn run4() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Real::new_const(&ctx, "x");
    let y = Real::new_const(&ctx, "y");

    let two = Real::from_real(&ctx, 2, 1);
    let x_2 = x.power(&two);
    let y_2 = y.power(&two);
    let three = Real::from_real(&ctx, 3, 1);
    let x_2_plus_y_2_gt_3 = Real::add(&ctx, &[&x_2, &y_2]).gt(&three);

    let x_3 = x.power(&three);
    let five = Real::from_real(&ctx, 5, 1);
    let x_3_plus_y_lt_5 = Real::add(&ctx, &[&x_3, &y]).lt(&five);

    let mut params = Params::new(&ctx);
    params.set_bool("pp.decimal", true);

    let solver = Solver::new(&ctx);
    solver.set_params(&params);
    solver.assert(&x_2_plus_y_2_gt_3);
    solver.assert(&x_3_plus_y_lt_5);

    assert_eq!(SatResult::Sat, solver.check());

    let model = solver.get_model().unwrap();

    println!("{}", model.to_string());
}

pub fn run5() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Real::new_const(&ctx, "x");
    let four = Real::from_real(&ctx, 4, 1);
    let zero = Real::from_real(&ctx, 0, 1);

    let solver = Solver::new(&ctx);
    solver.assert(&x.gt(&four)); // x > 4
    solver.assert(&x.lt(&zero)); // x < 0

    println!("{:?}", solver.check());
}
