// pub async fn get_random_unique_id<T, Fut>(
//     by_id: impl FnOnce(&u64, &ServerPool) -> Fut,
//     conn: &ServerPool,
// ) -> u64
// where
//     Fut: Future<Output = Option<T>>,
// {
//     loop {
//         let new_id = uuid::Uuid::new_v4().as_u128() as u64;

//         if by_id(&new_id, conn).await.is_none() {
//             return new_id;
//         }
//     }
// }

// async fn example<Fut>(f: impl FnOnce(i32, i32) -> Fut)
// where
//     Fut: Future<Output = bool>,
// {
//     f(1, 2).await;
// }
