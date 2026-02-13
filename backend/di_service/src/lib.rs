pub mod feature_di;
pub mod usecase_di;

// Re-export for backward compatibility
pub use usecase_di::{
    build_division_usecase,
    build_user_usecase,
    build_book_usecase,
};
