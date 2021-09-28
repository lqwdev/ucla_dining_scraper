use crate::model::item::ItemDetails;
use scraper::{Html, Selector};

pub fn parse(doc: &str) -> ItemDetails {
    parse_item(&Html::parse_document(doc))
}

fn parse_item(doc: &Html) -> ItemDetails {
    ItemDetails {
        description: parse_description(doc),
        ingredients: parse_ingredients(doc),
        allergens: parse_allergens(doc),
    }
}

fn parse_description(doc: &Html) -> Option<String> {
    Some(
        doc.select(&Selector::parse("div").unwrap())
            .filter(|e| e.value().attr("class") == Some("productinfo"))
            .nth(0)?
            .select(&Selector::parse("div").unwrap())
            .filter(|e| e.value().attr("class") == Some("description"))
            .nth(0)?
            .text()
            .nth(0)?
            .trim()
            .into(),
    )
}

fn parse_ingredients(doc: &Html) -> Option<String> {
    Some(
        doc.select(&Selector::parse("div").unwrap())
            .filter(|e| {
                if let Some(cls) = e.value().attr("class") {
                    return cls.contains("ingred_allergen");
                }
                return false;
            })
            .nth(0)?
            .text()
            .nth(2)?
            .trim()
            .into(),
    )
}

fn parse_allergens(doc: &Html) -> Option<String> {
    Some(
        doc.select(&Selector::parse("div").unwrap())
            .filter(|e| {
                if let Some(cls) = e.value().attr("class") {
                    return cls.contains("ingred_allergen");
                }
                return false;
            })
            .nth(0)?
            .text()
            .nth(5)?
            .trim()
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_item() {
        let html = r#"
<div class="recipecontainer">
<h2>Bruin Cheeseburger</h2>
<div class="productinfo">
    <div class="description">
    Blended Patty, American Cheese, Lettuce, Tomato, Pickle, Red Onion, Mayo, House-made Bun
    </div>
    <div class="prodwebcode"><img alt="AWHT" class="webcode-16px" src="./bruin-cheeseburger_files/awht.png">&nbsp;Contains Wheat</div>
    <div class="prodwebcode"><img alt="AGTN" class="webcode-16px" src="./bruin-cheeseburger_files/agtn.png">&nbsp;Contains Gluten</div>
    <div class="prodwebcode"><img alt="ASOY" class="webcode-16px" src="./bruin-cheeseburger_files/asoy.png">&nbsp;Contains Soy</div>
    <div class="prodwebcode"><img alt="AMLK" class="webcode-16px" src="./bruin-cheeseburger_files/amlk.png">&nbsp;Contains Dairy</div>
    <div class="prodwebcode"><img alt="AEGG" class="webcode-16px" src="./bruin-cheeseburger_files/aegg.png">&nbsp;Contains Eggs</div>
    <div class="prodwebcode"><img alt="HC" class="webcode-16px" src="./bruin-cheeseburger_files/hc.png">&nbsp;High Carbon Footprint</div>
</div>
<div class="nfbox">
<h3 class="nftitle">Nutrition Facts</h3>
<p class="nfserv">Serving Size 1&nbsp;each</p>
<p class="nfamt">Amount Per Serving</p>
<p class="nfcal"><span class="nfcaltxt">Calories</span> 459 </p>
<p class="nfdvhdr">% Daily Value*</p>
<p class="nfnutrient"><span class="nfmajornutrient">Total Fat</span> 21.3g <span class="nfdvval"><span class="nfdvvalnum">33</span>%</span></p>
<div class="nfindent">
    <p class="nfnutrient">Saturated Fat 8.8g <span class="nfdvval"><span class="nfdvvalnum">44</span>%</span></p>
    <p class="nfnutrient">Trans Fat 1.1g </p>
</div>
<p class="nfnutrient"><span class="nfmajornutrient">Cholesterol</span> 65.7mg <span class="nfdvval"><span class="nfdvvalnum">22</span>%</span></p>
<p class="nfnutrient"><span class="nfmajornutrient">Sodium</span> 1014.9mg <span class="nfdvval"><span class="nfdvvalnum">42</span>%</span></p>
<p class="nfnutrient"><span class="nfmajornutrient">Total Carbohydrate</span> 43.9g <span class="nfdvval"><span class="nfdvvalnum">34</span>%</span></p>
<div class="nfindent">
    <p class="nfnutrient">Dietary Fiber 1.6g <span class="nfdvval"><span class="nfdvvalnum">6</span>%</span></p>
    <p class="nfnutrient">Sugars 4.3g </p>
</div>
<p class="nfnutrient"><span class="nfmajornutrient">Protein</span> 21.7g </p>
    <div class="nfvitbar"></div>
    <div class="nfvit">
        <span class="nfvitleft">
        <span class="nfvitname">Vitamin A</span>
        <span class="nfvitpct">11%</span>
        </span>
•                        <span class="nfvitright">
            <span class="nfvitname">Vitamin C</span>
            <span class="nfvitpct">13%</span>
        </span>

    </div>
    <div class="nfvit">
        <span class="nfvitleft">
        <span class="nfvitname">Calcium</span>
        <span class="nfvitpct">18%</span>
        </span>
    
•                        <span class="nfvitright">
            <span class="nfvitname">Iron</span>
            <span class="nfvitpct">30%</span>
        </span>
    </div>
<div class="nfdisclaimer">*Percent Daily Values (DV) are based on a 2,000 calorie diet.</div>
</div>
<div class="ingred_allergen ia_img">
    <p><strong>INGREDIENTS:</strong> Blended Burger Patty (Halal Ground Beef, Onion, Roasted Mushroom, Quinoa, Garlic Salt, Pepper), Vegan Hamburger Bun (Water, Flour, Whole Wheat Flour, Vital Wheat Gluten, Sugar, Canola Oil, Sea Salt, Yeast), Tomato, American Cheese, Red Onion, Green Leaf Lettuce, Pickles, Butter, Mayonnaise, Kosher Salt, Pepper</p>
            <p><strong>ALLERGENS*:</strong> Milk, Eggs, Wheat, Soybeans, Gluten</p>
    <p><strong>*This item’s ingredients contain or are produced in a facility that processes the indicated allergens.</strong></p>
</div>
<img alt="Bruin Cheeseburger" class="recipeimage" src="./bruin-cheeseburger_files/400317.jpg">
</div>
        "#;
        let doc = scraper::Html::parse_document(html);
        let parsed = parse_item(&doc);
        let expected = ItemDetails {
            description: Some("Blended Patty, American Cheese, Lettuce, Tomato, Pickle, Red Onion, Mayo, House-made Bun".into()),
            ingredients: Some("Blended Burger Patty (Halal Ground Beef, Onion, Roasted Mushroom, Quinoa, Garlic Salt, Pepper), Vegan Hamburger Bun (Water, Flour, Whole Wheat Flour, Vital Wheat Gluten, Sugar, Canola Oil, Sea Salt, Yeast), Tomato, American Cheese, Red Onion, Green Leaf Lettuce, Pickles, Butter, Mayonnaise, Kosher Salt, Pepper".into()),
            allergens: Some("Milk, Eggs, Wheat, Soybeans, Gluten".into()),
        };
        assert_eq!(parsed, expected);
    }
}
