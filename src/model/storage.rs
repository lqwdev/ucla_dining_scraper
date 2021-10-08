use crate::model::{Item, Menu, MenuMeal, RestaurantMenu, Section};
use serde_json::json;

pub trait Storage {
    fn to_json(&self) -> serde_json::Value;
    fn to_json_min(&self) -> serde_json::Value;
}

impl Storage for Menu {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "name": self.name.name(),
            "meals": self.meals.iter().map(|s| s.to_json()).collect::<Vec<serde_json::Value>>(),
        })
    }

    fn to_json_min(&self) -> serde_json::Value {
        json!([
            self.name.name(),
            self.meals
                .iter()
                .map(|s| s.to_json_min())
                .collect::<Vec<serde_json::Value>>()
        ])
    }
}

impl Storage for MenuMeal {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "name": self.name.name(),
            "sections": self.sections.iter().map(|s| s.to_json()).collect::<Vec<serde_json::Value>>(),
        })
    }

    fn to_json_min(&self) -> serde_json::Value {
        json!([
            self.name.name(),
            self.sections
                .iter()
                .map(|s| s.to_json_min())
                .collect::<Vec<serde_json::Value>>()
        ])
    }
}

impl Storage for Item {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "id": self.id,
            "name": self.name,
        })
    }

    fn to_json_min(&self) -> serde_json::Value {
        json!([self.id, self.name])
    }
}

impl Storage for Section {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "name": self.name,
            "items": self.items.iter().map(|e| e.to_json()).collect::<Vec<serde_json::Value>>(),
        })
    }

    fn to_json_min(&self) -> serde_json::Value {
        json!([
            self.name,
            self.items
                .iter()
                .map(|e| e.to_json_min())
                .collect::<Vec<serde_json::Value>>(),
        ])
    }
}

impl Storage for RestaurantMenu {
    fn to_json(&self) -> serde_json::Value {
        json!({
            "date": self.date,
            "name": self.restaurant.name(),
            "meal": self.meal.name(),
            "sections": self.sections.iter().map(|s| s.to_json()).collect::<Vec<serde_json::Value>>(),
        })
    }

    fn to_json_min(&self) -> serde_json::Value {
        json!([
            self.date,
            self.restaurant.name(),
            self.meal.name(),
            self.sections
                .iter()
                .map(|s| s.to_json_min())
                .collect::<Vec<serde_json::Value>>(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Meal, Menu, MenuMeal, Restaurant};

    fn get_test_item() -> Item {
        Item {
            id: "141301".into(),
            name: "Roasted Vegetables".into(),
            recipe_link: "http://menu.dining.ucla.edu/Recipes/141301/2".into(),
            details: None,
        }
    }

    #[test]
    fn test_item_json() {
        assert_eq!(
            get_test_item().to_json(),
            json!({
                "id": "141301",
                "name": "Roasted Vegetables",
            }),
        );
    }

    #[test]
    fn test_item_json_min() {
        assert_eq!(
            get_test_item().to_json_min(),
            json!(["141301", "Roasted Vegetables"]),
        );
    }

    fn get_test_section() -> Section {
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
        }
    }

    #[test]
    fn test_section_json() {
        assert_eq!(
            get_test_section().to_json(),
            json!({
                "name": "The Front Burner",
                "items": [
                    {"id": "123056", "name": "Fusilli Fruiti De Mari"},
                    {"id": "138012", "name": "Toasted Herb & Cheese Bread"},
                    {"id": "141301", "name": "Roasted Vegetables"},
                ],
            }),
        );
    }

    #[test]
    fn test_section_json_min() {
        assert_eq!(
            get_test_section().to_json_min(),
            json!([
                "The Front Burner",
                [
                    ["123056", "Fusilli Fruiti De Mari"],
                    ["138012", "Toasted Herb & Cheese Bread"],
                    ["141301", "Roasted Vegetables"],
                ]
            ]),
        )
    }

    fn get_test_menu() -> RestaurantMenu {
        RestaurantMenu {
            date: "2021-09-30".into(),
            restaurant: Restaurant::DeNeve,
            meal: Meal::Lunch,
            sections: vec![
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
            ],
        }
    }

    #[test]
    fn test_menu_json() {
        assert_eq!(
            get_test_menu().to_json(),
            json!({
                "date": "2021-09-30",
                "name": "De Neve",
                "meal": "Lunch",
                "sections": [
                    {
                        "name": "Flex Bar",
                        "items": [
                            {"id": "977026", "name": "Italian Minestrone Soup"},
                            {"id": "977085", "name": "Turkey & Rice Soup"},
                        ],
                    },
                    {
                        "name": "The Front Burner",
                        "items": [
                            {"id": "123056", "name": "Fusilli Fruiti De Mari"},
                            {"id": "138012", "name": "Toasted Herb & Cheese Bread"},
                            {"id": "141301", "name": "Roasted Vegetables"},
                        ],
                    },
                ],
            }),
        )
    }

    #[test]
    fn test_menu_json_min() {
        assert_eq!(
            get_test_menu().to_json_min(),
            json!([
                "2021-09-30",
                "De Neve",
                "Lunch",
                [
                    [
                        "Flex Bar",
                        [
                            ["977026", "Italian Minestrone Soup"],
                            ["977085", "Turkey & Rice Soup"],
                        ],
                    ],
                    [
                        "The Front Burner",
                        [
                            ["123056", "Fusilli Fruiti De Mari"],
                            ["138012", "Toasted Herb & Cheese Bread"],
                            ["141301", "Roasted Vegetables"],
                        ],
                    ],
                ],
            ]),
        )
    }

    fn get_test_meal() -> MenuMeal {
        MenuMeal {
            name: Meal::Lunch,
            sections: vec![
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
            ],
        }
    }

    #[test]
    fn test_meal_json() {
        assert_eq!(
            get_test_meal().to_json(),
            json!({
                "name": "Lunch",
                "sections": [
                    {
                        "name": "Flex Bar",
                        "items": [
                            {"id": "977026", "name": "Italian Minestrone Soup"},
                            {"id": "977085", "name": "Turkey & Rice Soup"},
                        ],
                    },
                    {
                        "name": "The Front Burner",
                        "items": [
                            {"id": "123056", "name": "Fusilli Fruiti De Mari"},
                            {"id": "138012", "name": "Toasted Herb & Cheese Bread"},
                            {"id": "141301", "name": "Roasted Vegetables"},
                        ],
                    },
                ],
            }),
        )
    }

    #[test]
    fn test_meal_json_min() {
        assert_eq!(
            get_test_meal().to_json_min(),
            json!([
                "Lunch",
                [
                    [
                        "Flex Bar",
                        [
                            ["977026", "Italian Minestrone Soup"],
                            ["977085", "Turkey & Rice Soup"],
                        ],
                    ],
                    [
                        "The Front Burner",
                        [
                            ["123056", "Fusilli Fruiti De Mari"],
                            ["138012", "Toasted Herb & Cheese Bread"],
                            ["141301", "Roasted Vegetables"],
                        ],
                    ],
                ],
            ]),
        )
    }

    fn get_test_menu_full() -> Menu {
        Menu {
            name: Restaurant::DeNeve,
            meals: vec![
                MenuMeal {
                    name: Meal::Lunch,
                    sections: vec![
                        Section {
                            name: "Flex Bar".into(),
                            items: vec![
                                Item {
                                    id: "977026".into(),
                                    name: "Italian Minestrone Soup".into(),
                                    recipe_link: "http://menu.dining.ucla.edu/Recipes/977026/6"
                                        .into(),
                                    details: None,
                                },
                                Item {
                                    id: "977085".into(),
                                    name: "Turkey & Rice Soup".into(),
                                    recipe_link: "http://menu.dining.ucla.edu/Recipes/977085/6"
                                        .into(),
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
                                    recipe_link: "http://menu.dining.ucla.edu/Recipes/123056/6"
                                        .into(),
                                    details: None,
                                },
                                Item {
                                    id: "138012".into(),
                                    name: "Toasted Herb & Cheese Bread".into(),
                                    recipe_link: "http://menu.dining.ucla.edu/Recipes/138012/1"
                                        .into(),
                                    details: None,
                                },
                                Item {
                                    id: "141301".into(),
                                    name: "Roasted Vegetables".into(),
                                    recipe_link: "http://menu.dining.ucla.edu/Recipes/141301/2"
                                        .into(),
                                    details: None,
                                },
                            ],
                        },
                    ],
                },
                MenuMeal {
                    name: Meal::Dinner,
                    sections: vec![
                        Section {
                            name: "Flex Bar".into(),
                            items: vec![
                                Item {
                                    id: "977026".into(),
                                    name: "Italian Minestrone Soup".into(),
                                    recipe_link: "http://menu.dining.ucla.edu/Recipes/977026/6"
                                        .into(),
                                    details: None,
                                },
                                Item {
                                    id: "977085".into(),
                                    name: "Turkey & Rice Soup".into(),
                                    recipe_link: "http://menu.dining.ucla.edu/Recipes/977085/6"
                                        .into(),
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
                                    recipe_link: "http://menu.dining.ucla.edu/Recipes/123056/6"
                                        .into(),
                                    details: None,
                                },
                                Item {
                                    id: "138012".into(),
                                    name: "Toasted Herb & Cheese Bread".into(),
                                    recipe_link: "http://menu.dining.ucla.edu/Recipes/138012/1"
                                        .into(),
                                    details: None,
                                },
                                Item {
                                    id: "141301".into(),
                                    name: "Roasted Vegetables".into(),
                                    recipe_link: "http://menu.dining.ucla.edu/Recipes/141301/2"
                                        .into(),
                                    details: None,
                                },
                            ],
                        },
                    ],
                },
            ],
        }
    }

    #[test]
    fn test_menu_full_json() {
        assert_eq!(
            get_test_menu_full().to_json(),
            json!({
                "name": "De Neve",
                "meals": [
                    {
                        "name": "Lunch",
                        "sections": [
                            {
                                "name": "Flex Bar",
                                "items": [
                                    {"id": "977026", "name": "Italian Minestrone Soup"},
                                    {"id": "977085", "name": "Turkey & Rice Soup"},
                                ],
                            },
                            {
                                "name": "The Front Burner",
                                "items": [
                                    {"id": "123056", "name": "Fusilli Fruiti De Mari"},
                                    {"id": "138012", "name": "Toasted Herb & Cheese Bread"},
                                    {"id": "141301", "name": "Roasted Vegetables"},
                                ],
                            },
                        ],
                    },
                    {
                        "name": "Dinner",
                        "sections": [
                            {
                                "name": "Flex Bar",
                                "items": [
                                    {"id": "977026", "name": "Italian Minestrone Soup"},
                                    {"id": "977085", "name": "Turkey & Rice Soup"},
                                ],
                            },
                            {
                                "name": "The Front Burner",
                                "items": [
                                    {"id": "123056", "name": "Fusilli Fruiti De Mari"},
                                    {"id": "138012", "name": "Toasted Herb & Cheese Bread"},
                                    {"id": "141301", "name": "Roasted Vegetables"},
                                ],
                            },
                        ],
                    },
                ]
            }),
        )
    }

    #[test]
    fn test_menu_full_json_min() {
        assert_eq!(
            get_test_menu_full().to_json_min(),
            json!([
                "De Neve",
                [
                    [
                        "Lunch",
                        [
                            [
                                "Flex Bar",
                                [
                                    ["977026", "Italian Minestrone Soup"],
                                    ["977085", "Turkey & Rice Soup"],
                                ],
                            ],
                            [
                                "The Front Burner",
                                [
                                    ["123056", "Fusilli Fruiti De Mari"],
                                    ["138012", "Toasted Herb & Cheese Bread"],
                                    ["141301", "Roasted Vegetables"],
                                ],
                            ],
                        ],
                    ],
                    [
                        "Dinner",
                        [
                            [
                                "Flex Bar",
                                [
                                    ["977026", "Italian Minestrone Soup"],
                                    ["977085", "Turkey & Rice Soup"],
                                ],
                            ],
                            [
                                "The Front Burner",
                                [
                                    ["123056", "Fusilli Fruiti De Mari"],
                                    ["138012", "Toasted Herb & Cheese Bread"],
                                    ["141301", "Roasted Vegetables"],
                                ],
                            ],
                        ],
                    ],
                ]
            ]),
        )
    }
}
