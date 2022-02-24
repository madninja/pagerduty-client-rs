use ::serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CatalogCountry {
    pub code: String,
    pub name: String,
    pub flag_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Budget {
    pub id: String,
    pub singular: String,
    pub plural: String,
    pub format: BudgetFormat,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BudgetFormat {
    pub singular: String,
    pub plural: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub custom_user_properties: Vec<String>,
    pub give_amounts: Vec<u32>,
    pub catalog_countries: Vec<CatalogCountry>,
    pub valid_billing_information: bool,
    pub show_bonus_amount: bool,
    pub company_hashtags: Vec<String>,
    pub suggested_hashtags: Vec<String>,
    pub trending_hashtags: Vec<String>,
    pub example_reason: String,
    pub hashtagging_mode: String,
    pub budget: Budget,
    pub brand_color: String,
}
