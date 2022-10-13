use z3::{ast::*, *};

use crate::solvers;

pub fn dog_cat_mouse() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let dog = Int::new_const(&ctx, "dog");
    let cat = Int::new_const(&ctx, "cat");
    let mouse = Int::new_const(&ctx, "mouse");

    let one = Int::from_i64(&ctx, 1);
    let twenty_five = Int::from_i64(&ctx, 25);
    let hundred = Int::from_i64(&ctx, 100);
    let fifteen_hundred = Int::from_i64(&ctx, 1500);
    let ten_k = Int::from_i64(&ctx, 10000);

    let solver = Solver::new(&ctx);
    solver.assert(&dog.gt(&one)); // at leadt one dog
    solver.assert(&cat.gt(&one)); // one cat
    solver.assert(&mouse.gt(&one)); // one mouse

    // dogs + cats + mice == 100
    solver.assert(&Int::add(&ctx, &[&dog, &cat, &mouse])._eq(&hundred));
    // dog 1500 cents, cat 100 cents, mouse 25 cents
    // 1500 * dog + 100 * cat + 25 * mouse == 10000
    solver.assert(
        &Int::add(
            &ctx,
            &[
                &Int::mul(&ctx, &[&fifteen_hundred, &dog]),
                &Int::mul(&ctx, &[&hundred, &cat]),
                &Int::mul(&ctx, &[&twenty_five, &mouse]),
            ],
        )
        ._eq(&ten_k),
    );

    match solver.check() {
        SatResult::Sat => println!("{:?}", solver.get_model().unwrap()),
        SatResult::Unsat => println!("No solution"),
        SatResult::Unknown => println!("Unknown Error"),
    }
}

pub fn sudoku() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let o = Int::from_i64(&ctx, 0);
    // 9x9 matrix of integer variables (sudoku board)
    let mut x_mat: Vec<Vec<Int>> = vec![vec![o; 9]; 9];

    for i in 0..9 {
        for j in 0..9 {
            x_mat[i][j] = Int::new_const(&ctx, format!("x{}{}", i + 1, j + 1));
        }
    }

    let one = Int::from_i64(&ctx, 1);
    let nine = Int::from_i64(&ctx, 9);

    // each cell has a value between 1 and 9
    let cells_c = x_mat
        .iter()
        .flatten()
        .map(|cell| Bool::and(&ctx, &[&one.le(cell), &cell.le(&nine)]))
        .collect::<Vec<_>>();

    // each row contains a digit at most once
    let rows_c = x_mat
        .iter()
        .map(|row| {
            Int::distinct(
                &ctx,
                row.iter().map(|cell| cell).collect::<Vec<_>>().as_slice(),
            )
        })
        .collect::<Vec<_>>();

    // convert from matrix of rows to matrix of columns
    let x_mat_colwise: Vec<Vec<_>> = (0..x_mat[0].len())
        .map(|i| x_mat.iter().map(|c| &c[i]).collect())
        .collect();

    // each column contains a digit at most once
    let cols_c = x_mat_colwise
        .iter()
        .map(|col| Int::distinct(&ctx, col.as_slice()))
        .collect::<Vec<_>>();

    // each 3x3 square contains a digit at most once
    let mut sq_c: Vec<Bool> = vec![];
    for i0 in 0..3 {
        for j0 in 0..3 {
            let mut sq: Vec<&Int> = vec![];
            for i in 0..3 {
                for j in 0..3 {
                    sq.push(&x_mat[3 * i0 + i][3 * j0 + j]);
                }
            }
            sq_c.push(Int::distinct(&ctx, sq.as_slice()));
        }
    }

    // sudoku instance, '0' means empty cell
    let instance: [[u64; 9]; 9] = [
        [0, 0, 0, 0, 9, 4, 0, 3, 0],
        [0, 0, 0, 5, 1, 0, 0, 0, 7],
        [0, 8, 9, 0, 0, 0, 0, 4, 0],
        [0, 0, 0, 0, 0, 0, 2, 0, 8],
        [0, 6, 0, 2, 0, 1, 0, 5, 0],
        [1, 0, 2, 0, 0, 0, 0, 0, 0],
        [0, 7, 0, 0, 0, 0, 5, 2, 0],
        [9, 0, 0, 0, 6, 5, 0, 0, 0],
        [0, 4, 0, 9, 7, 0, 0, 0, 0],
    ];
    println!("{:?}", pretty_print(instance.iter()));

    let zero = Int::from_i64(&ctx, 0);
    // Constraint based on the instance
    let instance_c: Vec<Vec<Bool>> = (0..9)
        .map(|i| {
            (0..9)
                .map(|j| {
                    Bool::ite(
                        &Int::from_u64(&ctx, instance[i][j])._eq(&zero),
                        &Bool::from_bool(&ctx, true),
                        &x_mat[i][j]._eq(&Int::from_u64(&ctx, instance[i][j])),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let solver = Solver::new(&ctx);

    let cells_rows_cols_sq_c = [cells_c, rows_c, cols_c, sq_c].concat();

    // assert cell, row, column, square constraints
    let _ = cells_rows_cols_sq_c
        .iter()
        .map(|c| {
            solver.assert(c);
        })
        .collect::<Vec<_>>();

    // assert instance constraints
    let _ = instance_c
        .iter()
        .flatten()
        .map(|c| {
            solver.assert(c);
        })
        .collect::<Vec<_>>();

    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let result: Vec<Vec<u64>> = (0..9)
                .map(|i| {
                    (0..9)
                        .map(|j| model.eval(&x_mat[i][j], true).unwrap().as_u64().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect();
            // nicely print the board
            pretty_print(result.iter());
        }
        _ => println!("No solution."),
    }
}

pub fn pretty_print<T: AsRef<[u64]> + std::fmt::Debug>(matrix: impl Iterator<Item = T>) {
    matrix.enumerate().for_each(|(i, row)| {
        row.as_ref().iter().enumerate().for_each(|(j, item)| {
            print!(" {:?} ", item);
            if (j + 1) % 3 == 0 {
                print!("|");
            }

            if (j + 1) == 9 {
                print!("\n");
                if (i + 1) % 3 == 0 {
                    print!("______________________________\n");
                }
            }
        })
    });
}

pub fn eight_queens() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    // each queen must be in a different row
    // represent a queen by an integer: column position
    let queen: [Int; 8] = (0..8)
        .map(|i| Int::new_const(&ctx, format!("Q_{}", i + 1)))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    // each queen is in a column 1-8
    let one = Int::from_u64(&ctx, 1);
    let eight = Int::from_u64(&ctx, 8);
    let val_c: Vec<Bool> = (0..8)
        .map(|i| Bool::and(&ctx, &[&queen[i].ge(&one), &queen[i].le(&eight)]))
        .collect::<Vec<_>>();

    // at most one queen per column
    let col_c = Int::distinct(&ctx, queen.iter().map(|q| q).collect::<Vec<_>>().as_slice());

    let mut diag_c: Vec<Bool> = vec![];

    for i in 0..8 {
        for j in 0..i {
            let i_int = Int::from_u64(&ctx, i as u64);
            let j_int = Int::from_u64(&ctx, j as u64);

            //If i == j, then True, else queen[i] - queen[j] != i - j AND queen[i] - queen[j] != j - i
            diag_c.push(Bool::ite(
                &i_int._eq(&j_int),
                &Bool::from_bool(&ctx, true),
                &Bool::and(
                    &ctx,
                    &[
                        &Int::sub(&ctx, &[&queen[i], &queen[j]])
                            ._eq(&Int::sub(&ctx, &[&i_int, &j_int]))
                            .not(),
                        &Int::sub(&ctx, &[&queen[i], &queen[j]])
                            ._eq(&Int::sub(&ctx, &[&j_int, &i_int]))
                            .not(),
                    ],
                ),
            ));
        }
    }

    let constraints = [val_c, diag_c].concat();

    let solver = Solver::new(&ctx);
    solver.assert(&col_c);
    let _ = constraints
        .iter()
        .map(|c| {
            solver.assert(c);
        })
        .collect::<Vec<_>>();

    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            println!("{:?}", model);
        }
        _ => println!("No solution"),
    }
}
