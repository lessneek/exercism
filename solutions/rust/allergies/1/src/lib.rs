pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

use Allergen::*;

impl Allergen {
    fn iter_all() -> std::slice::Iter<'static, Allergen> {
        static ALL_ALLERGENS: [Allergen; 8] = [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ];
        ALL_ALLERGENS.iter()
    }

    fn is_in(&self, score: u32) -> bool {
        score & *self as u32 > 0
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            allergens: Allergen::iter_all()
                .filter(|&&a| a.is_in(score))
                .map(|a| a.clone())
                .collect(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
