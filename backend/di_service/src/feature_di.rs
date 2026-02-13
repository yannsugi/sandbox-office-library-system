use repository_impl_division::DivisionRepoImpl;
use repository_impl_user::UserRepoImpl;
use repository_impl_book::BookRepoImpl;
use feature_division::DivisionFeatureService;
use feature_user::UserFeatureService;
use feature_book::BookFeatureService;

pub fn build_division_feature_service() -> DivisionFeatureService<DivisionRepoImpl> {
    DivisionFeatureService::new(DivisionRepoImpl::new())
}

pub fn build_user_feature_service() -> UserFeatureService<UserRepoImpl> {
    UserFeatureService::new(UserRepoImpl::new())
}

pub fn build_book_feature_service() -> BookFeatureService<BookRepoImpl> {
    BookFeatureService::new(BookRepoImpl::new())
}
