pub mod item_request;
pub mod menu_request;

pub trait Downloadable {
    fn url(&self) -> String;
}
