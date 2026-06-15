pub mod core;

pub mod prelude {
    pub use crate::core::Id;
    pub use crate::core::component::Component;
    pub use crate::core::dom::Html;
    pub use crate::core::route::HttpMethod::*;
    pub use crate::core::route::Route;
}
