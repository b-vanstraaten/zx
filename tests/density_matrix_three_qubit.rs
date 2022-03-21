use nalgebra::dmatrix;
use test_log::test;
// pretty assertions for human readability
use zx::*;

#[test]
fn h0() {
    let number_of_qubits: usize = 3;

    let mut program = Program::new(number_of_qubits);
    program.h(0);
    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
                C::new(0.5, 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn h1() {
    let number_of_qubits: usize = 3;

    let mut program = Program::new(number_of_qubits);
    program.h(1);
    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn h2() {
    let number_of_qubits: usize = 3;

    let mut program = Program::new(number_of_qubits);
    program.h(2);
    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn h0_cnot01() {
    let number_of_qubits: usize = 3;

    let mut program = Program::new(number_of_qubits);
    program.h(0);
    program.cnot(0, 1);
    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn h0_cnot02() {
    let number_of_qubits: usize = 3;

    let mut program = Program::new(number_of_qubits);
    program.h(0);
    program.cnot(0, 2);
    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn h2_cnot21() {
    let number_of_qubits: usize = 3;

    let mut program = Program::new(number_of_qubits);
    program.h(2);
    program.cnot(2, 1);
    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn h2_cnot20() {
    let number_of_qubits: usize = 3;

    let mut program = Program::new(number_of_qubits);
    program.h(2);
    program.cnot(2, 0);
    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn ghz_0() {
    let number_of_qubits: usize = 3;

    let mut program = Program::new(number_of_qubits);
    program.h(0);
    program.cnot(0, 1);
    program.cnot(0, 2);
    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn ghz_1() {
    let number_of_qubits: usize = 3;

    let mut program = Program::new(number_of_qubits);
    program.h(1);
    program.cnot(1, 0);
    program.cnot(1, 2);
    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn ghz_2() {
    let number_of_qubits: usize = 3;

    let mut program = Program::new(number_of_qubits);
    program.h(2);
    program.cnot(2, 1);
    program.cnot(2, 0);
    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn hhh_measure() {
    let number_of_qubits: usize = 3;

    let mut program = Program::new(number_of_qubits);
    program.h(0);
    program.h(1);
    program.h(2);

    program.measure(0);
    program.measure(1);
    program.measure(2);

    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
                C::new(0.125, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0.125, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0.125, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.125, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.125, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.125, 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.125, 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.125, 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn deutsch_jozsa_balanced() {
    let number_of_qubits: usize = 3;

    let mut program = Program::new(number_of_qubits);
    program.rx(2, PI);
    program.h(0);
    program.h(1);
    program.h(2);

    program.cnot(0, 2);
    program.cnot(1, 2);

    program.h(0);
    program.h(1);

    program.measure(0);
    program.measure(1);

    program.run();

    program.draw();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(-0.5, 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(-0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state);
}