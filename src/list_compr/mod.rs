use z3::{ast::*, *};

pub fn run1() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    // Create a list of 5 integer variables x
    let x_list = (1..6)
        .map(|i: i32| Int::new_const(&ctx, format!("x{}", i)))
        .collect::<Vec<_>>();
    // Create a list of 5 integer variables y
    let y_list = (1..6)
        .map(|i: i32| Int::new_const(&ctx, format!("y{}", i)))
        .collect::<Vec<_>>();

    println!("List X: {:?}", x_list);
    println!("List Y: {:?}", y_list);

    // Create a list of x + y element-wise
    let x_plus_y = (0..5)
        .map(|i: usize| Int::add(&ctx, &[&x_list[i], &y_list[i]]))
        .collect::<Vec<_>>();

    println!("List X + Y: {:?}", x_plus_y);

    // Create a list of x > y element-wise
    let x_gt_y = (0..5)
        .map(|i: usize| x_list[i].gt(&y_list[i]))
        .collect::<Vec<_>>();

    // Turning the elements of x > y into references for the next step
    let x_gt_y = (0..5).map(|i: usize| &x_gt_y[i]).collect::<Vec<_>>();

    println!("List X > Y: {:?}", x_gt_y);

    // Creating a conjunction (AND) of x > y element-wise
    let and = Bool::and(&ctx, x_gt_y.as_slice());

    println!("Conjunction of X > Y: {:?}", and);

    let zero = Int::from_i64(&ctx, 0);
    // Extra: creating a matrix (2D vector) of 9 variables x
    let mut x_matrix: Vec<Vec<Int>> = vec![vec![zero; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            x_matrix[i][j] = Int::new_const(&ctx, format!("x{}{}", i + 1, j + 1));
        }
    }

    println!("Matrix X: {:?}", x_matrix);
}
