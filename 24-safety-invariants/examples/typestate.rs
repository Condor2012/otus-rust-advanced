use safety_invariants::contracts::{Builder, Param1, Param2};

fn main() {
    let param1 = Param1::new(1).expect("1 не равно нулю");
    let param2 = Param2::new(2).expect("2 не равно нулю");

    let configured = Builder::new().with_param1(param1).with_param2(param2);

    println!("Сумма параметров: {}", configured.sum());
}
