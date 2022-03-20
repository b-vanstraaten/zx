use itertools::iproduct;
use log::debug;
use nalgebra::matrix;
use std::fmt;

use crate::index_swapping::*;
use crate::types::*;
use crate::{DensityMatrix, Program};

use crate::state::{Measure, MeasureAll, SingleQubitGate, SingleQubitKraus, TwoQubitGate};

#[derive(Debug, Clone)]
pub enum Operation {
    Measure(usize),
    MeasureAll,

    X(usize),
    Y(usize),
    Z(usize),
    H(usize),
    ArbitarySingle(usize, Matrix2x2),
    S(usize),

    RX(usize, Angle),
    RY(usize, Angle),
    RZ(usize, Angle),
    R(usize, Angle, Angle, Angle),

    CNOT(usize, usize),
    SISWAP(usize, usize),
    ArbitaryTwo(usize, usize, Matrix4x4),
    ISWAP(usize, usize),
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operation::Measure(qubit) => write!(f, "M"),
            Operation::MeasureAll => write!(f, ""),

            Operation::X(qubit) => write!(f, "X_{}", qubit),
            Operation::Y(qubit) => write!(f, "Y_{}", qubit),
            Operation::Z(qubit) => write!(f, "Z_{}", qubit),
            Operation::H(qubit) => write!(f, "H_{}", qubit),
            Operation::S(qubit) => write!(f, "S_{}", qubit),
            Operation::RX(qubit, angle) => write!(f, "RX_{}({})", qubit, angle),
            Operation::RY(qubit, angle) => write!(f, "RY_{}({})", qubit, angle),
            Operation::RZ(qubit, angle) => write!(f, "RZ_{}({})", qubit, angle),

            Operation::ArbitarySingle(qubit, _) => write!(f, "U_{}", qubit),
            Operation::R(qubit,phi, theta, omega) => write!(f, "R_{}({}, {}, {})", qubit, phi, theta, omega),
            Operation::CNOT(control, target) => write!(f, "CNOT {} -> {}", control, target),
            Operation::SISWAP(_, _) => write!(f, ""),
            Operation::ArbitaryTwo(_, _, _) => write!(f, ""),
            Operation::ISWAP(_, _) => write!(f, ""),
        }
    }
}

pub fn implement_gate<T: Measure + MeasureAll + SingleQubitGate + TwoQubitGate>(state: &mut T, gate: &Operation) {
    debug!("{:?}", gate);
    match gate {
        Operation::Measure(qubit) => state.measure(qubit),
        Operation::MeasureAll => state.measure_all(),

        Operation::X(qubit) => state.single_qubit_gate(qubit, &SIGMA_X),
        Operation::Y(qubit) => state.single_qubit_gate(qubit, &SIGMA_Y),
        Operation::Z(qubit) => state.single_qubit_gate(qubit, &SIGMA_Z),
        Operation::S(qubit) => state.single_qubit_gate(qubit, &S),
        Operation::H(qubit) => state.single_qubit_gate(qubit, &H),

        Operation::RX(qubit, angle) => {
            let u = IDENTITY * C::new((angle / 2.).cos(), 0.)
                - SIGMA_X * C::new(0., (angle / 2.).sin());
            state.single_qubit_gate(qubit, &u)
        }

        Operation::RY(qubit, angle) => {
            let u = IDENTITY * C::new((angle / 2.).cos(), 0.)
                - SIGMA_Y * C::new(0., (angle / 2.).sin());
            state.single_qubit_gate(qubit, &u)
        }

        Operation::RZ(qubit, angle) => {
            let u = IDENTITY * C::new((angle / 2.).cos(), 0.)
                - SIGMA_Z * C::new(0., (angle / 2.).sin());
            state.single_qubit_gate(qubit, &u)
        }
        Operation::R(qubit, phi, theta, omega) => {
            let (c_theta, s_theta) = ((theta / 2.).cos(), (theta / 2.).sin());
            let (c_plus, s_plus) = (((phi + omega) / 2.).cos(), ((phi + omega) / 2.).sin());
            let (c_minus, s_minus) = (((phi - omega) / 2.).cos(), ((phi - omega) / 2.).sin());

            let u: Matrix2x2 = matrix![
                C::new(c_plus, -s_plus) * c_theta, -C::new(c_minus, s_minus) * s_theta;
                C::new(c_minus, -s_minus) * s_theta,  C::new(c_plus, s_plus) * c_theta
            ];
            state.single_qubit_gate(qubit, &u)
        }
        Operation::ArbitarySingle(qubit, u) => state.single_qubit_gate(qubit, u),

        Operation::CNOT(control, target) => state.two_qubit_gate(target, control, &CNOT),
        Operation::ISWAP(control, target) => state.two_qubit_gate(target, control, &ISWAP),
        Operation::SISWAP(control, target) => state.two_qubit_gate(target, control, &SISWAP),
        Operation::ArbitaryTwo(control, target, u) => state.two_qubit_gate(control, target, u),
    }
}

pub fn which_qubits(gate: &Operation) -> Vec<&usize> {
    match gate {
        Operation::Measure(qubit) => vec![qubit],
        Operation::MeasureAll => vec![],

        Operation::X(qubit) => vec![qubit],
        Operation::Y(qubit) => vec![qubit],
        Operation::Z(qubit) => vec![qubit],
        Operation::S(qubit) => vec![qubit],
        Operation::H(qubit) => vec![qubit],

        Operation::RX(qubit, _) => vec![qubit],
        Operation::RY(qubit, _) => vec![qubit],
        Operation::RZ(qubit, _) => vec![qubit],
        Operation::R(qubit, _, _, _) => vec![qubit],
        Operation::ArbitarySingle(qubit, _) => vec![qubit],

        Operation::CNOT(control, target) => vec![control, target],
        Operation::ISWAP(control, target) => vec![control, target],
        Operation::SISWAP(control, target) => vec![control, target],
        Operation::ArbitaryTwo(control, target, _) => vec![control, target],
    }
}
