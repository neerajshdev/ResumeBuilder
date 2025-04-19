// Presentation layer - contains UI components and view models
pub mod components;

// We'll keep the module with just the components
pub mod prelude {
    pub use super::components::*;
} 