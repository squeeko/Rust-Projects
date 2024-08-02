use linregress::{FormulaRegressionBuilder, RegressionDataBuilder}; // https://docs.rs/linregress/latest/linregress/
use std::collections::HashMap;
fn main() {
    let mut data1 = HashMap::new();
    data1.insert("Y", vec![1., 2., 3., 4.]);
    data1.insert("X", vec![10., 20., 30., 40.]);
    let data1 = RegressionDataBuilder::new().build_from(data1).unwrap();

    let y = vec![30., 60., 90., 120.];
    let x = vec![50., 100., 200., 400.];
    let data2 = vec![("X", x), ("Y", y)];
    let data2 = RegressionDataBuilder::new().build_from(data2).unwrap();

    let regression1 = FormulaRegressionBuilder::new()
        .data(&data1)
        .formula("Y ~ X")
        .fit()
        .unwrap();

    dbg!(&regression1);

    let regression2 = FormulaRegressionBuilder::new()
        .data(&data2)
        .formula("Y ~ X")
        .fit()
        .unwrap();

    dbg!(&regression2);
}
/*
 Output:

 [src/main.rs:20:5] &regression1 = RegressionModel {
    regressor_names: [
        "X",
    ],
    model: LowLevelRegressionModel {
        parameters: [
            -2.886579864025407e-15,
            0.10000000000000012,
        ],
        se: [
            2.3785734810242756e-15,
            8.685322334937218e-17,
        ],
        ssr: 7.543482406175925e-30,
        rsquared: 1.0,
        rsquared_adj: 1.0,
        pvalues: [
            0.3487778115186867,
            4.9303806576313245e-30,
        ],
        residuals: [
            1.7763568394002505e-15,
            6.661338147750939e-16,
            -8.881784197001252e-16,
            -1.7763568394002505e-15,
        ],
        scale: 3.771741203087963e-30,
    },
}
[src/main.rs:28:5] &regression2 = RegressionModel {
    regressor_names: [
        "X",
    ],
    model: LowLevelRegressionModel {
        parameters: [
            30.0,
            0.24,
        ],
        se: [
            11.534447462313462,
            0.05004345937369794,
        ],
        ssr: 360.0,
        rsquared: 0.92,
        rsquared_adj: 0.8800000000000001,
        pvalues: [
            0.1214716393098556,
            0.04083369533745614,
        ],
        residuals: [
            -12.0,
            6.0,
            12.0,
            -6.0,
        ],
        scale: 180.0,
    },
}
  */
