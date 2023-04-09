pub struct Allergies {
    allergies: Vec<Allergen>,
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
        let mut map: Vec<(Allergen, u32)> = vec![];

        map.push((Allergen::Cats, 128));
        map.push((Allergen::Pollen, 64));
        map.push((Allergen::Chocolate, 32));
        map.push((Allergen::Tomatoes, 16));
        map.push((Allergen::Strawberries, 8));
        map.push((Allergen::Shellfish, 4));
        map.push((Allergen::Peanuts, 2));
        map.push((Allergen::Eggs, 1));

        let mut compatible_allergies = vec![];

        let mut reduceble_score = if score <= 255 { score } else { score - 256 };
        for (key, value) in map {
            if value <= reduceble_score {
                compatible_allergies.push(key.to_owned());
                reduceble_score -= value;
            }
        }

        let created_allergies = Allergies {
            allergies: compatible_allergies,
        };

        created_allergies
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // unimplemented!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
        for aller in &self.allergies {
            if aller == allergen {
                return true;
            }
        }

        false
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        //unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
        println!("{:?}", self.allergies);
        self.allergies.clone()
    }
}
