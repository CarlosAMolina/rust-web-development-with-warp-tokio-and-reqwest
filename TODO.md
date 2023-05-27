Continue: 7.5

Check in  server/src/store.rs what is used:
- Option 1:
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
- Option 2:
            Err(e) => Err(e),

