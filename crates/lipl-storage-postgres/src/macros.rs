macro_rules! query {
    (
        $name:ident,
        $action:ident,
        $return_type:ty,
        $sql:path,
        $types:path,
        $f:expr
        $(, $param_name:ident : $param_type:ty)* $(,)?
    ) => {
        async fn $name(&self, $($param_name: $param_type,)*) -> Result<$return_type> {
            let client = self.pool.get().await.map_err(postgres_error)?;
            let statement = client.prepare_typed($sql, $types,).await.map_err(postgres_error)?;
            let query_result = client.$action(&statement, &[$(&$param_name,)*]).await.map_err(postgres_error)?;
            let result = $f(query_result)?;
            Ok(result)
        }
    };
}

pub(crate) use query;
