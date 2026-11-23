use fp_oop::solid::{
    BankTransferProcessor, CreditCardProcessor, CryptoProcessor, Human, InMemoryUserRepository,
    PaymentProcessor, Rectangle, RegisterData, Robot, Square, UserService, Workable,
    print_area,
};

fn main() {
    let processors: Vec<&dyn PaymentProcessor> = vec![
        &CreditCardProcessor,
        &CryptoProcessor,
        &BankTransferProcessor,
    ];
    for processor in processors {
        processor.process(123.0).expect("payment failed");
    }

    print_area(&Rectangle {
        width: 5.0,
        height: 3.0,
    });
    print_area(&Square { side: 10.0 });

    println!("{}", Robot.work());
    println!("{}", Human.work());

    let mut service = UserService::new(InMemoryUserRepository::new());
    let user = service.register(RegisterData {
        id: 10,
        name: "user".into(),
    });
    println!("Зарегистрирован: {user:?}");
    println!("Найден: {:?}", service.find(10));
}
