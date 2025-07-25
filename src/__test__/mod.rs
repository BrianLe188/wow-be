#[cfg(test)]
mod tests {
    use crate::utils::tsp::nearest_neighbor;

    #[tokio::test]
    async fn test_nearest_neighbor() {
        let origin = [0.0, 0.0];
        let points = vec![[1.0, 1.0], [2.0, 2.0], [0.0, 1.0]];

        let result = nearest_neighbor(origin, points.clone());

        let expected_result = vec![[0.0, 1.0], [1.0, 1.0], [2.0, 2.0]];

        assert_eq!(result, expected_result);
    }
}
