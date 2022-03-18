use nalgebra::{dmatrix, dvector};
use rand::{thread_rng, Rng};
use test_log::test;

use zx::*;

// pretty assertions for human readability

/// tests the x gate on a single qubit
#[test]
fn x0_pi() {
    let number_of_qubits: usize = 1;
    let angle = PI;

    let mut program = Program::new(number_of_qubits);
    program.rx(0, angle);
    program.run();

    let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());
    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
            C::new(c * c, 0.), C::new(0.,c * s);
            C::new(0., -c * s), C::new(s * s, 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state)
}

/// tests the x gate on a single qubit
#[test]
fn x0_pi_half() {
    let number_of_qubits: usize = 1;
    let angle = PI / 2.;

    let mut program = Program::new(number_of_qubits);
    program.rx(0, angle);
    program.run();

    let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
            C::new(c * c, 0.), C::new(0.,c * s);
            C::new(0., -c * s), C::new(s * s, 0.);
        ],
    );

    assert_approximately_equal(required_state, program.state)
}

/// tests the x gate on a single qubit
#[test]
fn y0_pi() {
    let number_of_qubits: usize = 1;
    let angle = PI;

    let mut program = Program::new(number_of_qubits);
    program.ry(0, angle);
    program.run();

    let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
            C::new(c * c, 0.), C::new(c * s, 0.);
            C::new(c * s, 0.), C::new(s * s, 0.);
        ],
    );

    assert_approximately_equal(required_state, program.state)
}

/// tests the y gate on a single qubit
#[test]
fn y0_pi_half() {
    let number_of_qubits: usize = 1;
    let angle = PI / 2.;

    let mut program = Program::new(number_of_qubits);
    program.ry(0, angle);
    program.run();

    let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
            C::new(c * c, 0.), C::new(c * s, 0.);
            C::new(c * s, 0.), C::new(s * s, 0.);
        ],
    );
    assert_approximately_equal(required_state, program.state)
}

/// tests the z gate on a single qubit
#[test]
fn z0_pi() {
    let number_of_qubits: usize = 1;
    let mut program = Program::new(number_of_qubits);
    program.rz(0, PI);
    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        DensityMatrix::from_diagonal(&dvector!(C::new(1., 0.), C::new(0., 0.))),
    );

    assert_approximately_equal(required_state, program.state)
}

/// tests the x gate on a single qubit
#[test]
fn z0_pi_half() {
    let number_of_qubits: usize = 1;
    let mut program = Program::new(number_of_qubits);
    program.rz(0, PI / 2.);
    program.run();

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        DensityMatrix::from_diagonal(&dvector!(C::new(1., 0.), C::new(0., 0.))),
    );

    assert_approximately_equal(required_state, program.state)
}

#[test]
fn xy_commutation() {
    let number_of_qubits: usize = 1;
    let angle = PI;

    let mut program = Program::new(number_of_qubits);
    program.rx(0, angle);
    program.ry(0, angle);
    program.run();

    let mut other_program = Program::new(number_of_qubits);
    other_program.rz(0, angle);
    other_program.run();

    assert_approximately_equal(program.state, other_program.state)
}

#[test]
fn xz_commutation() {
    let number_of_qubits: usize = 1;
    let angle = PI;

    let mut program = Program::new(number_of_qubits);
    program.rx(0, angle);
    program.rz(0, angle);
    program.run();

    let mut other_program = Program::new(number_of_qubits);
    other_program.ry(0, 3. * angle);
    other_program.run();

    assert_approximately_equal(program.state, other_program.state)
}

/// tests the x gate on a single qubit
#[test]
fn m0() {
    let number_of_qubits: usize = 1;
    let angle = PI / 3.;

    let mut program = Program::new(number_of_qubits);
    program.rx(0, angle);
    program.measure(0);
    program.run();

    let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

    let required_state = State::new_from_density_matrix(
        number_of_qubits,
        dmatrix![
        C::new(c * c, 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(s * s, 0.);],
    );

    assert_approximately_equal(required_state, program.state)
}

#[test]
fn h0() {
    let number_of_qubits: usize = 1;

    let mut program = Program::new(number_of_qubits);
    program.h(0);
    program.run();

    let required_density_matrix = dmatrix![
        C::new(1. / 2., 0.), C::new(1. / 2., 0.);
        C::new(1. / 2., 0.), C::new(1. / 2., 0.);
    ];

    let required_state = State::new_from_density_matrix(number_of_qubits, required_density_matrix);

    assert_approximately_equal(required_state, program.state)
}

#[test]
fn r0() {
    let number_of_qubits: usize = 1;
    let mut range = thread_rng();
    for _ in 1..10 {
        let phi = 2. * PI * range.gen::<Angle>();
        let theta = 2. * PI * range.gen::<Angle>();
        let omega = 2. * PI * range.gen::<Angle>();

        let mut program = Program::new(number_of_qubits);
        program.r(0, phi, theta, omega);
        program.run();

        let mut other_program = Program::new(number_of_qubits);
        other_program.rz(0, phi);
        other_program.ry(0, theta);
        other_program.rz(0, omega);
        other_program.run();

        assert_approximately_equal(program.state, other_program.state)
    }
}

#[test]
fn s() {
    let number_of_qubits: usize = 1;

    let mut program = Program::new(number_of_qubits);
    program.s(0);
    program.s(0);
    program.run();

    let mut other_program = Program::new(number_of_qubits);
    other_program.z(0);
    other_program.run();

    assert_approximately_equal(program.state, other_program.state)
}
