use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductCategory {
    Bakery,
    DairyEggs,
    FruitVegetables,
    MeatFish,

    StapleFoods,
    CannedJarred,
    OilsSaucesSpices,
    BreakfastSpreads,

    SnacksSweets,
    Beverages,
    DrugstoreBeauty,
    HouseholdCleaning,
    PetSupplies,

    Electronics,
    StationeryOffice,
    HomeLiving,
    ClothingTextiles,
    DiyGarden,
    BooksMedia,

    #[default]
    Uncategorized,
}

impl fmt::Display for ProductCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ProductCategory::Bakery => "Brot & Backwaren",
            ProductCategory::DairyEggs => "Milchprodukte & Eier",
            ProductCategory::FruitVegetables => "Obst & Gemüse",
            ProductCategory::MeatFish => "Fleisch, Fisch & Ersatz",
            ProductCategory::StapleFoods => "Grundnahrungsmittel",
            ProductCategory::CannedJarred => "Konserven & Gläser",
            ProductCategory::OilsSaucesSpices => "Öle, Saucen & Gewürze",
            ProductCategory::BreakfastSpreads => "Frühstück & Aufstriche",
            ProductCategory::SnacksSweets => "Süßwaren & Knabberzeug",
            ProductCategory::Beverages => "Getränke",
            ProductCategory::DrugstoreBeauty => "Drogerie & Kosmetik",
            ProductCategory::HouseholdCleaning => "Haushalt & Reinigung",
            ProductCategory::PetSupplies => "Haustierbedarf",
            ProductCategory::Electronics => "Elektronik & Zubehör",
            ProductCategory::StationeryOffice => "Schreibwaren & Büro",
            ProductCategory::HomeLiving => "Wohnen & Küche",
            ProductCategory::ClothingTextiles => "Kleidung & Textilien",
            ProductCategory::DiyGarden => "Baumarkt & Garten",
            ProductCategory::BooksMedia => "Medien & Freizeit",
            ProductCategory::Uncategorized => "Unkategorisiert",
        };
        write!(f, "{}", s)
    }
}
