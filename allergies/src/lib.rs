use std::collections::HashMap;

pub struct Allergies {
    allergies: Vec<Allergen>,
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        // unimplemented!("Given the '{score}' score, construct a new Allergies struct.");

        let mut map: HashMap<Allergen, u32> = HashMap::new();

        map.insert(Allergen::Eggs, 1);
        map.insert(Allergen::Peanuts, 2);
        map.insert(Allergen::Shellfish, 4);
        map.insert(Allergen::Strawberries, 8);
        map.insert(Allergen::Tomatoes, 16);
        map.insert(Allergen::Chocolate, 32);
        map.insert(Allergen::Pollen, 64);
        map.insert(Allergen::Cats, 128);

        let mut created_allergies = Allergies {
            allergies: vec![],
            score,
        };

        let mut score_checked = score;
        let mut posible_allergies: HashMap<Allergen, u32> = map
            .iter()
            .filter(|(allergy, score)| score <= score_checked)
            .collect();

        while score_checked > 0 {
            posible_allergies
        }

        /* for (allergy, value) in map.iter() {
                   if value <= &created_allergies.score {
                       created_allergies.allergies.push(allergy.clone());
                   }

                   if value == &created_allergies.score {
                       created_allergies.allergies.clear();
                       created_allergies.allergies.push(allergy.clone());
                   }
               }
        */
        created_allergies
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // unimplemented!("Determine if the patient is allergic to the '{allergen:?}' allergen.");

        self.allergies
            .iter()
            .any(|current_allergen| current_allergen.eq(allergen))
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        //unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
        self.allergies.clone()
    }
}
