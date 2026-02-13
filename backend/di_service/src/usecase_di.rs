use usecase_division::DivisionUsecaseService;
use usecase_user::UserUsecaseService;
use usecase_book::BookUsecaseService;
use repository_impl_division::DivisionRepoImpl;
use repository_impl_user::UserRepoImpl;
use repository_impl_book::BookRepoImpl;
use feature_division::DivisionFeatureService;
use feature_user::UserFeatureService;
use feature_book::BookFeatureService;
use crate::feature_di;

// Type aliases for complex usecase types
pub type DivisionUsecase = DivisionUsecaseService<DivisionFeatureService<DivisionRepoImpl>>;
pub type UserUsecase = UserUsecaseService<UserFeatureService<UserRepoImpl>>;
pub type BookUsecase = BookUsecaseService<BookFeatureService<BookRepoImpl>, UserFeatureService<UserRepoImpl>>;

pub fn build_division_usecase() -> DivisionUsecase {
    DivisionUsecaseService::new(feature_di::build_division_feature_service())
}

pub fn build_user_usecase() -> UserUsecase {
    UserUsecaseService::new(feature_di::build_user_feature_service())
}

pub fn build_book_usecase() -> BookUsecase {
    BookUsecaseService::new(
        feature_di::build_book_feature_service(),
        feature_di::build_user_feature_service()
    )
}
