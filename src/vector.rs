use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub, Neg},
};

pub trait VectorElement:
    Copy + Debug + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Neg<Output = Self>
{
}
impl<T> VectorElement for T where
    T: Copy + Debug + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = Self>
{
}

#[derive(Copy, Clone, Debug)]
pub struct Vector<T, const N: usize>
where
    T: VectorElement,
{
    pub data: [T; N],
}

impl<T, const N: usize> Vector<T, N>
where
    T: VectorElement,
{
    pub fn new(data: [T; N]) -> Self {
        Vector { data }
    }

    pub fn add(&self, other: &Self) -> Self {
        Vector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a + *b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Vector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a - *b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        Vector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a * *b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn div(&self, other: &Self) -> Self {
        Vector {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| *a / *b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

pub fn dot<T, const N: usize>(u: &Vector<T, N>, v: &Vector<T, N>) -> T
where
    T: VectorElement,
{
    u.data
        .iter()
        .zip(v.data.iter())
        .map(|(a, b)| *a * *b)
        .reduce(|acc, x| acc + x)
        .unwrap()
}

macro_rules! impl_vector_methods {
    ($($N:expr => { $($field:ident : $index:expr),+ }),+) => {
        $(
            impl<T> Vector<T, $N>
            where
                T: VectorElement,
            {
                $(
                    pub fn $field(&self) -> T {
                        self.data[$index]
                    }
                )+
            }
        )+
    };
}

impl_vector_methods! {
    2 => {x: 0, y: 1},
    3 => {x: 0, y: 1, z: 2},
    4 => {x: 0, y: 1, z: 2, w: 3}
}

pub fn cross<T>(u: &Vector<T, 3>, v: &Vector<T, 3>) -> Vector<T, 3>
where
    T: VectorElement,
{
    Vector::<T, 3>::new([
        u.y() * v.z() - u.z() * v.y(), 
        -(u.x() * v.z() - u.z() * v.x()),
        u.x() * v.y() - u.y() * v.x()
    ])
}
