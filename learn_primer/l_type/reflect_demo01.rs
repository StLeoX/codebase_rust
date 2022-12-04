// Rust Reflect

// Any类型，定义于std::any::Any
// TypeId 就是RTTI（Runtime Type ID）
pub trait Any: 'static {
    fn get_type_id(&self) -> TypeId;
}

impl<T: 'static + ?Sized > Any for T {
    fn get_type_id(&self) -> TypeId { TypeId::of::<T>() }
}


////////////////////////////////
// Any type usecase