#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::HashSet,
    fmt::{Display, Result},
    hash::Hash,
    ops::{Add, Mul, Sub},
};

use crate::topo_sort;

#[derive(Debug, Clone)]
pub struct Scalar {
    pub value: f64,
    pub grad: f64,
    pub _operation: Option<String>,
    pub _children: Option<HashSet<Scalar>>,
}

impl Scalar {
    pub fn from(value: f64, op: Option<String>, _children: Option<HashSet<Scalar>>) -> Scalar {
        Scalar {
            value,
            grad: 0.0,
            _operation: op,
            _children,
        }
    }

    pub fn backward(&mut self) {
        let mut sorted_nodes: Vec<Scalar> = Vec::new();
        let mut visited: HashSet<Scalar> = HashSet::new();
        topo_sort::topological_sort(self, &mut visited, &mut sorted_nodes);

        self.grad = 1.0;
        for node in sorted_nodes.iter().rev() {
            // node._backward()
        }
    }
}

impl Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "scalar=[{}]", self.value)
    }
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Hash for Scalar {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {}
}

// Scalar + Scalar -> Scalar
impl Add for Scalar {
    type Output = Scalar;
    fn add(self, other: Scalar) -> Scalar {
        let mut out = Scalar::from(
            self.value + other.value,
            Some(String::from("+")),
            Some(HashSet::from([self, other])),
        );

        // self.grad += out.grad;
        // other.grad += out.grad;

        out
    }
}

impl Eq for Scalar {
    fn assert_receiver_is_total_eq(&self) {}
}

// Scalar + f64 -> Scalar
impl Add<f64> for Scalar {
    type Output = Scalar;
    fn add(self, t: f64) -> Scalar {
        let other = Scalar::from(t, Some(String::from("")), None);
        Scalar::from(
            self.value + other.value,
            Some(String::from("+")),
            Some(HashSet::from([self, other])),
        )
    }
}

// Scalar - Scalar -> Scalar
impl Sub for Scalar {
    type Output = Scalar;
    fn sub(self, other: Self) -> Scalar {
        Scalar::from(
            self.value - other.value,
            Some(String::from("-")),
            Some(HashSet::from([self, other])),
        )
    }
}

// Scalar - f64 -> Scalar
impl Sub<f64> for Scalar {
    type Output = Scalar;
    fn sub(self, t: f64) -> Scalar {
        let other = Scalar::from(t, Some(String::from("")), None);
        Scalar::from(
            self.value - t,
            Some(String::from("-")),
            Some(HashSet::from([self, other])),
        )
    }
}

impl Mul for Scalar {
    type Output = Scalar;
    fn mul(self, other: Self) -> Scalar {
        Scalar::from(
            self.value * other.value,
            Some(String::from("*")),
            Some(HashSet::from([self, other])),
        )
    }
}

// Scalar * f64 -> Scalar
impl Mul<f64> for Scalar {
    type Output = Scalar;
    fn mul(self, t: f64) -> Scalar {
        let other = Scalar::from(t, Some(String::from("")), None);
        Scalar::from(
            self.value * t,
            Some(String::from("*")),
            Some(HashSet::from([self, other])),
        )
    }
}
