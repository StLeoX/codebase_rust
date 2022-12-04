// Entry的大致设计
pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),  // 已存在
    Vacant(VacantEntry<'a, K, V>),  // 未存在
}