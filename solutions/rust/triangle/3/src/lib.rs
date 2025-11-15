#[derive(PartialEq)]
pub enum TriangleType {
    Equilateral,
    Scalene,
    Isosceles,
}

pub struct Triangle<T> {
    sides: [T; 3],
    ttype: TriangleType,
}

impl<T> Triangle<T>
where
    T: Copy + PartialEq + PartialOrd + std::ops::Add<Output = T> + Default,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        use TriangleType::*;
        let zero = T::default();
        match sides {
            [a, b, c] if a == zero || b == zero || c == zero => None,
            [a, b, c] if a + b < c || b + c < a => None,
            [a, b, c] if a == b && b == c => Some(Triangle {
                sides,
                ttype: Equilateral,
            }),
            [a, b, c] if a == b || b == c || a == c => Some(Triangle {
                sides,
                ttype: Isosceles,
            }),
            [a, b, c] if a != b && b != c => Some(Triangle {
                sides,
                ttype: Scalene,
            }),
            _ => None,
        }
    }
}

impl<'a, T: 'a> Triangle<T> {
    pub fn sides(&'a self) -> &'a [T; 3] {
        &self.sides
    }

    pub fn is_equilateral(&self) -> bool {
        self.ttype == TriangleType::Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.ttype == TriangleType::Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.ttype == TriangleType::Isosceles
    }
}
