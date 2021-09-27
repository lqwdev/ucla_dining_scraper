use crate::model::item::Item;
use crate::model::menu::{Menu, Section};
use crate::request::menu_request::MenuRequest;
use scraper::element_ref::ElementRef;
use scraper::{Html, Selector};
use url::Url;

pub fn parse(doc: &str, request: &MenuRequest) -> Menu {
    let sections = parse_sections(&Html::parse_document(doc));
    Menu {
        date: request.date.clone(),
        restaurant: request.restaurant.clone(),
        meal: request.meal.clone(),
        sections: sections,
    }
}

fn parse_sections(doc: &Html) -> Vec<Section> {
    doc.select(&Selector::parse("li").unwrap())
        .filter(|e| e.value().attr("class") == Some("sect-item"))
        .map(|section| parse_section(&section))
        .collect()
}

fn parse_section(section: &ElementRef) -> Section {
    Section {
        name: parse_section_name(section),
        items: parse_section_items(section),
    }
}

fn parse_section_name(section: &ElementRef) -> String {
    section.text().nth(0).unwrap().trim().into()
}

fn parse_section_items(section: &ElementRef) -> Vec<Item> {
    section
        .select(&Selector::parse("li").unwrap())
        .filter(|e| e.value().attr("class") == Some("menu-item"))
        .map(|item| parse_item(&item))
        .collect()
}

fn parse_item(item: &ElementRef) -> Item {
    let node = item
        .select(&Selector::parse("a").unwrap())
        .filter(|e| e.value().attr("class") == Some("recipelink"))
        .nth(0)
        .unwrap();

    let recipe_link = parse_item_recipe_link(&node);

    Item {
        id: parse_id(&recipe_link),
        name: parse_item_name(&node),
        recipe_link: recipe_link,
        details: None,
    }
}

fn parse_item_name(item: &ElementRef) -> String {
    item.text().nth(0).unwrap().into()
}

fn parse_item_recipe_link(item: &ElementRef) -> String {
    item.value().attr("href").unwrap().into()
}

fn parse_id(recipe_link: &String) -> String {
    let parsed_url = Url::parse(recipe_link.as_str()).unwrap();
    return parsed_url.path_segments().unwrap().nth(1).unwrap().into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_id() {
        assert_eq!(
            parse_id(&"http://menu.dining.ucla.edu/Recipes/977026/6".into()),
            "977026".to_string(),
        );
        assert_eq!(
            parse_id(&"http://menu.dining.ucla.edu/Recipes/977085/6".into()),
            "977085".to_string(),
        );
        assert_eq!(
            parse_id(&"http://menu.dining.ucla.edu/Recipes/141301/2".into()),
            "141301".to_string(),
        );
    }

    #[test]
    fn test_parse_document() {
        let html = r#"
<ul class="sect-list">
  <li class="sect-item">
    Flex Bar
    <ul class="item-list">
      <li class="menu-item">
        <span class="tooltip-target-wrapper">
          <a class="recipelink" href="http://menu.dining.ucla.edu/Recipes/977026/6">Italian Minestrone Soup</a>
          <img alt="VG" class="webcode-16px" src="/Content/Images/WebCodes/128px/vg.png" />
          <img alt="AWHT" class="webcode-16px" src="/Content/Images/WebCodes/128px/awht.png" />
          <img alt="AGTN" class="webcode-16px" src="/Content/Images/WebCodes/128px/agtn.png" />
          <img alt="ASOY" class="webcode-16px" src="/Content/Images/WebCodes/128px/asoy.png" />
          <img alt="LC" class="webcode-16px" src="/Content/Images/WebCodes/128px/lc.png" />
        </span>
        <div class="item-description-wrapper">
          <div class="item-description">
            <div class="tt-description">
              Tomato, Onion, Celery, Carrot, Spinach, Chickpea, Kidney Bean, Pasta, Basil, Oregano
            </div>
            <div class="tt-prodwebcode"><img alt="VG" class="webcode-16px" src="/Content/Images/WebCodes/128px/vg.png" />&nbsp;Vegan Menu Option</div>
            <div class="tt-prodwebcode"><img alt="AWHT" class="webcode-16px" src="/Content/Images/WebCodes/128px/awht.png" />&nbsp;Contains Wheat</div>
            <div class="tt-prodwebcode"><img alt="AGTN" class="webcode-16px" src="/Content/Images/WebCodes/128px/agtn.png" />&nbsp;Contains Gluten</div>
            <div class="tt-prodwebcode"><img alt="ASOY" class="webcode-16px" src="/Content/Images/WebCodes/128px/asoy.png" />&nbsp;Contains Soy</div>
            <div class="tt-prodwebcode"><img alt="LC" class="webcode-16px" src="/Content/Images/WebCodes/128px/lc.png" />&nbsp;Low Carbon Footprint</div>
          </div>
        </div>
      </li>
      <li class="menu-item">
        <span class="tooltip-target-wrapper">
          <a class="recipelink" href="http://menu.dining.ucla.edu/Recipes/977085/6">Turkey &amp; Rice Soup</a>
        </span>
        <div class="item-description-wrapper">
        </div>
      </li>
    </ul>
  </li>
  <li class="sect-item">
    The Front Burner
    <ul class="item-list">
      <li class="menu-item">
        <span class="tooltip-target-wrapper">
          <a class="recipelink" href="http://menu.dining.ucla.edu/Recipes/123056/6">Fusilli Fruiti De Mari</a>
            <img alt="AWHT" class="webcode-16px" src="/Content/Images/WebCodes/128px/awht.png" />
            <img alt="AGTN" class="webcode-16px" src="/Content/Images/WebCodes/128px/agtn.png" />
            <img alt="ASOY" class="webcode-16px" src="/Content/Images/WebCodes/128px/asoy.png" />
            <img alt="AMLK" class="webcode-16px" src="/Content/Images/WebCodes/128px/amlk.png" />
            <img alt="AEGG" class="webcode-16px" src="/Content/Images/WebCodes/128px/aegg.png" />
            <img alt="ACSF" class="webcode-16px" src="/Content/Images/WebCodes/128px/acsf.png" />
            <img alt="AFSH" class="webcode-16px" src="/Content/Images/WebCodes/128px/afsh.png" />
        </span>
        <div class="item-description-wrapper">
          <div class="item-description">
            <div class="tt-description">(Prepared with Alcohol)</div>
            <div class="tt-prodwebcode"><img alt="AWHT" class="webcode-16px" src="/Content/Images/WebCodes/128px/awht.png" />&nbsp;Contains Wheat</div>
            <div class="tt-prodwebcode"><img alt="AGTN" class="webcode-16px" src="/Content/Images/WebCodes/128px/agtn.png" />&nbsp;Contains Gluten</div>
            <div class="tt-prodwebcode"><img alt="ASOY" class="webcode-16px" src="/Content/Images/WebCodes/128px/asoy.png" />&nbsp;Contains Soy</div>
            <div class="tt-prodwebcode"><img alt="AMLK" class="webcode-16px" src="/Content/Images/WebCodes/128px/amlk.png" />&nbsp;Contains Dairy</div>
            <div class="tt-prodwebcode"><img alt="AEGG" class="webcode-16px" src="/Content/Images/WebCodes/128px/aegg.png" />&nbsp;Contains Eggs</div>
            <div class="tt-prodwebcode"><img alt="ACSF" class="webcode-16px" src="/Content/Images/WebCodes/128px/acsf.png" />&nbsp;Contains Crustacean Shellfish</div>
            <div class="tt-prodwebcode"><img alt="AFSH" class="webcode-16px" src="/Content/Images/WebCodes/128px/afsh.png" />&nbsp;Contains Fish</div>
          </div>
        </div>
      </li>
      <li class="menu-item">
        <span class="tooltip-target-wrapper">
          w/
          <a class="recipelink" href="http://menu.dining.ucla.edu/Recipes/138012/1">Toasted Herb &amp; Cheese Bread</a>
            <img alt="V" class="webcode-16px" src="/Content/Images/WebCodes/128px/v.png" />
            <img alt="AWHT" class="webcode-16px" src="/Content/Images/WebCodes/128px/awht.png" />
            <img alt="AGTN" class="webcode-16px" src="/Content/Images/WebCodes/128px/agtn.png" />
            <img alt="AMLK" class="webcode-16px" src="/Content/Images/WebCodes/128px/amlk.png" />
        </span>
        <div class="item-description-wrapper">
          <div class="item-description">
            <div class="tt-prodwebcode"><img alt="AWHT" class="webcode-16px" src="/Content/Images/WebCodes/128px/awht.png" />&nbsp;Contains Wheat</div>
            <div class="tt-prodwebcode"><img alt="AGTN" class="webcode-16px" src="/Content/Images/WebCodes/128px/agtn.png" />&nbsp;Contains Gluten</div>
            <div class="tt-prodwebcode"><img alt="AMLK" class="webcode-16px" src="/Content/Images/WebCodes/128px/amlk.png" />&nbsp;Contains Dairy</div>
          </div>
        </div>
      </li>
      <li class="menu-item">
        <span class="tooltip-target-wrapper">
          <a class="recipelink" href="http://menu.dining.ucla.edu/Recipes/141301/2">Roasted Vegetables</a>
            <img alt="VG" class="webcode-16px" src="/Content/Images/WebCodes/128px/vg.png" />
            <img alt="LC" class="webcode-16px" src="/Content/Images/WebCodes/128px/lc.png" />
        </span>
        <div class="item-description-wrapper">
          <div class="item-description">
            <div class="tt-prodwebcode"><img alt="LC" class="webcode-16px" src="/Content/Images/WebCodes/128px/lc.png" />&nbsp;Low Carbon Footprint</div>
          </div>
        </div>
      </li>
    </ul>
  </li>
</ul>
        "#;
        let doc = scraper::Html::parse_document(html);
        let parsed_sections = parse_sections(&doc);
        let expected_sections = vec![
            Section {
                name: "Flex Bar".into(),
                items: vec![
                    Item {
                        id: "977026".into(),
                        name: "Italian Minestrone Soup".into(),
                        recipe_link: "http://menu.dining.ucla.edu/Recipes/977026/6".into(),
                        details: None,
                    },
                    Item {
                        id: "977085".into(),
                        name: "Turkey & Rice Soup".into(),
                        recipe_link: "http://menu.dining.ucla.edu/Recipes/977085/6".into(),
                        details: None,
                    },
                ],
            },
            Section {
                name: "The Front Burner".into(),
                items: vec![
                    Item {
                        id: "123056".into(),
                        name: "Fusilli Fruiti De Mari".into(),
                        recipe_link: "http://menu.dining.ucla.edu/Recipes/123056/6".into(),
                        details: None,
                    },
                    Item {
                        id: "138012".into(),
                        name: "Toasted Herb & Cheese Bread".into(),
                        recipe_link: "http://menu.dining.ucla.edu/Recipes/138012/1".into(),
                        details: None,
                    },
                    Item {
                        id: "141301".into(),
                        name: "Roasted Vegetables".into(),
                        recipe_link: "http://menu.dining.ucla.edu/Recipes/141301/2".into(),
                        details: None,
                    },
                ],
            },
        ];
        assert_eq!(parsed_sections, expected_sections);
    }
}
