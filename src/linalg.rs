use std::ops;

pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn add(&self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn print(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

impl ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn add(&self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn print(&self) {
        println!("x: {}, y: {}, z: {}", self.x, self.y, self.z);
    }
}

impl ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

pub struct Matrix4D {
    m: [[f64; 4]; 4],
}

impl Matrix4D {
    pub fn new(m: [[f64; 4]; 4]) -> Self {
        Matrix4D { m }
    }
}

pub fn multiply_matrix_vector(v: &Vector3D, mat: &Matrix4D) -> Vector3D {
    let mut out = Vector3D {
        x: v.x * mat.m[0][0] + v.y * mat.m[1][0] + v.z * mat.m[2][0] + mat.m[3][0],
        y: v.x * mat.m[0][1] + v.y * mat.m[1][1] + v.z * mat.m[2][1] + mat.m[3][1],
        z: v.x * mat.m[0][2] + v.y * mat.m[1][2] + v.z * mat.m[2][2] + mat.m[3][2],
    };

    let w = v.x * mat.m[0][3] + v.y * mat.m[1][3] + v.z * mat.m[2][3] + mat.m[3][3];

    if w != 0.0 {
        out.x /= w;
        out.y /= w;
        out.z /= w;
    }

    out
}

impl ops::Mul<Vector3D> for Matrix4D {
    type Output = Vector3D;
    fn mul(self, rhs: Vector3D) -> Self::Output {
        multiply_matrix_vector(&rhs, &self)
    }
}

impl ops::Mul<Matrix4D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: Matrix4D) -> Self::Output {
        multiply_matrix_vector(&self, &rhs)
    }
}
