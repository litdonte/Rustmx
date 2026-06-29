pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

pub trait RouteInfo {
    fn method(&self) -> HttpMethod
    where
        Self: Sized;
    fn path(&self) -> &'static str;
}

#[cfg(test)]
#[allow(dead_code)]
mod route_tests {
    use crate::core::route::{HttpMethod, RouteInfo};
    use truss_macros::routes;

    #[routes]
    pub enum AppRoute {
        #[get("/scenes")]
        Scenes,
        #[get("/scene/{id}")]
        Scene(String),
        #[post("/scene")]
        CreateScene,
    }

    #[test]
    fn test_route_path_scenes() {
        let route = AppRoute::Scenes;
        assert_eq!(route.path(), "/scenes");
    }

    #[test]
    fn test_route_path_scene() {
        let route = AppRoute::Scene("1".to_string());
        assert_eq!(route.path(), "/scene/{id}");
    }

    #[test]
    fn test_route_path_create_scene() {
        let route = AppRoute::CreateScene;
        assert_eq!(route.path(), "/scene");
    }

    #[test]
    fn test_route_method_get() {
        let route = AppRoute::Scenes;
        assert!(matches!(route.method(), HttpMethod::Get));
    }

    #[test]
    fn test_route_method_post() {
        let route = AppRoute::CreateScene;
        assert!(matches!(route.method(), HttpMethod::Post));
    }
}
