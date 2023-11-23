use sos_dentes::input_handler::{select_operation_mode, OperationMode};

#[test]
fn client_mode_selection() {
    let input_mock = "1\n".as_bytes();
    let output_mock = Vec::with_capacity(0);
    match select_operation_mode(input_mock, output_mock) {
        Some(OperationMode::Client) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn service_mode_selection() {
    let input_mock = "2\n".as_bytes();
    let output_mock = Vec::with_capacity(0);
    match select_operation_mode(input_mock, output_mock) {
        Some(OperationMode::Service) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn dentist_mode_selection() {
    let input_mock = "3\n".as_bytes();
    let output_mock = Vec::with_capacity(0);
    match select_operation_mode(input_mock, output_mock) {
        Some(OperationMode::Dentist) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn wrong_mode_selection() {
    let input_mocks: Vec<&[u8]> = vec![b"4\n", b"5\n", b"\n", b"error\n", b"wrong\n"];
    for input in input_mocks.into_iter() {
        match select_operation_mode(input, Vec::with_capacity(0)) {
            None => assert!(true),
            _ => assert!(false),
        }
    }
}
