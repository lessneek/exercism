#[derive(PartialEq)]
pub enum TriangleType {
    Equilateral,
    Scalene,
    Isosceles,
}

pub struct Triangle {
    pub sides: [u64; 3],
    pub ttype: TriangleType,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        use TriangleType::*;

        match sides {
            [a, b, c] if a == 0 || b == 0 || c == 0 => None,
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
