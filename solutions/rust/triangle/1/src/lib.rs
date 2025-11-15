pub struct Triangle {
    sides: [u64; 3],
    equilateral: bool,
    scalene: bool,
    isosceles: bool,
}

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        sides.sort();

        match sides {
            [0, _, _] => None,
            [a, b, c] if a + b < c || b + c < a => None,
            [a, b, c] if a == b && b == c => Some(Triangle {
                sides,
                equilateral: true,
                scalene: false,
                isosceles: false,
            }),
            [a, b, c] if a == b || b == c => Some(Triangle {
                sides,
                equilateral: false,
                scalene: false,
                isosceles: true,
            }),
            [a, b, c] if a != b && b != c => Some(Triangle {
                sides,
                equilateral: false,
                scalene: true,
                isosceles: false,
            }),
            _ => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.isosceles
    }
}
