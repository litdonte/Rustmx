pub mod core;

pub mod prelude {
    pub use crate::core::Id;
    pub use crate::core::component::Component;
    pub use crate::core::dom::Html;
    pub use crate::core::route::HttpMethod::*;
    pub use crate::core::route::Route;
}

#[cfg(test)]
mod tests {
    use crate::core::component::Component;
    use crate::core::dom::Html;
    use rustmx_macros::component;

    struct Scene {
        title: String,
    }

    #[component]
    pub struct ScenePanel {
        scene: Scene,
    }

    impl ScenePanel {
        fn render(&self) -> Html {
            Html::new(self.scene.title.clone())
        }
    }

    #[test]
    fn test_component_has_id() {
        let panel = ScenePanel::new(Scene {
            title: "Act 1".to_string(),
        });
        // id exists and is not empty
        assert!(!panel.id().value().is_empty());
    }

    #[test]
    fn test_component_with_id() {
        let panel = ScenePanel::new(Scene {
            title: "Act 1".to_string(),
        })
        .with_id("primary-panel");
        // id starts with the supplied prefix
        assert!(panel.id().value().starts_with("primary-panel"));
    }
}
