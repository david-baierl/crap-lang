pub trait Singleton<T> {
    fn instance() -> &'static T;
}