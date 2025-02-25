impl Content
{
    pub fn get_repository(pool : sqlx :: Pool < sqlx :: Postgres >) ->
    ContentRepository { ContentRepository :: new(pool) }
} impl Content
{
    pub fn base_sql() -> String { format! ("SELECT * FROM contents",) } pub fn
    group_by() -> String { "".to_string() } pub fn query_builder() ->
    ContentRepositoryQueryBuilder
    {
        let base_sql = format! ("SELECT * FROM contents",); let g = Content ::
        group_by(); ContentRepositoryQueryBuilder :: from(& base_sql, & g)
    }
} impl ContentSummary
{
    pub fn base_sql_with(where_and_statements : & str) -> String
    {
        tracing :: debug!
        ("{} base_sql_with group: {}",
        "SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, title, thumbnail_image, source FROM contents",
        ""); let query = if where_and_statements.is_empty()
        {
            format!
            ("{} {}", format!
            ("SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, title, thumbnail_image, source FROM contents",),
            "")
        } else
        {
            if where_and_statements.to_lowercase().starts_with("where")
            {
                format!
                ("{} {} {}", format!
                ("SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, title, thumbnail_image, source FROM contents",),
                where_and_statements, "")
            } else
            {
                format!
                ("{} WHERE {} {}", format!
                ("SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, title, thumbnail_image, source FROM contents",),
                where_and_statements, "")
            }
        }; query
    } pub fn query_builder() -> ContentRepositoryQueryBuilder
    {
        let base_sql = format!
        ("SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, title, thumbnail_image, source FROM contents",);
        ContentRepositoryQueryBuilder :: from(& base_sql, "").with_count()
    }
} #[derive(Debug, Clone)] pub struct ContentRepository
{ pool : sqlx :: Pool < sqlx :: Postgres > , }
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
ContentRepositoryUpdateRequest
{
    pub title : Option < String > , pub thumbnail_image : Option < String > ,
    pub source : Option < String > , pub description : Option < String > , pub
    creator_id : Option < i64 >
} impl ContentRepositoryUpdateRequest
{
    pub fn new() -> Self { Self :: default() } pub fn
    with_title(mut self, title : String) -> Self
    { self.title = Some(title); self } pub fn
    with_thumbnail_image(mut self, thumbnail_image : String) -> Self
    { self.thumbnail_image = Some(thumbnail_image); self } pub fn
    with_source(mut self, source : String) -> Self
    { self.source = Some(source); self } pub fn
    with_description(mut self, description : String) -> Self
    { self.description = Some(description); self } pub fn
    with_creator_id(mut self, creator_id : i64) -> Self
    { self.creator_id = Some(creator_id); self }
} impl ContentRepository
{
    pub fn new(pool : sqlx :: Pool < sqlx :: Postgres >) -> Self
    { Self { pool } } pub fn queries(& self) -> Vec < & 'static str >
    {
        vec!
        ["CREATE TABLE IF NOT EXISTS contents (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,title TEXT NOT NULL,thumbnail_image TEXT NOT NULL,source TEXT NOT NULL,description TEXT NOT NULL,creator_id BIGINT NOT NULL, FOREIGN KEY (creator_id) REFERENCES users(id) ON DELETE CASCADE);",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_created_at_on_contents'\n        AND tgrelid = 'contents'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_created_at_on_contents\n        BEFORE INSERT ON contents\n        FOR EACH ROW\n        EXECUTE FUNCTION set_created_at();\n    END IF;\nEND $$",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_updated_at_on_contents'\n        AND tgrelid = 'contents'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_updated_at_on_contents\n        BEFORE INSERT OR UPDATE ON contents\n        FOR EACH ROW\n        EXECUTE FUNCTION set_updated_at();\n    END IF;\nEND $$",
        "CREATE INDEX IF NOT EXISTS idx_contents_creator_id ON contents(creator_id);"]
    } pub async fn create_this_table(& self) -> std :: result :: Result < (),
    sqlx :: Error >
    {
        tracing :: trace!
        ("Create table: {}",
        "CREATE TABLE IF NOT EXISTS contents (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,title TEXT NOT NULL,thumbnail_image TEXT NOT NULL,source TEXT NOT NULL,description TEXT NOT NULL,creator_id BIGINT NOT NULL, FOREIGN KEY (creator_id) REFERENCES users(id) ON DELETE CASCADE);");
        sqlx ::
        query("CREATE TABLE IF NOT EXISTS contents (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,title TEXT NOT NULL,thumbnail_image TEXT NOT NULL,source TEXT NOT NULL,description TEXT NOT NULL,creator_id BIGINT NOT NULL, FOREIGN KEY (creator_id) REFERENCES users(id) ON DELETE CASCADE);").execute(&
        self.pool).await ? ; Ok(())
    } pub async fn create_related_tables(& self) -> std :: result :: Result <
    (), sqlx :: Error >
    {
        for query in
        ["DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_created_at_on_contents'\n        AND tgrelid = 'contents'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_created_at_on_contents\n        BEFORE INSERT ON contents\n        FOR EACH ROW\n        EXECUTE FUNCTION set_created_at();\n    END IF;\nEND $$",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_updated_at_on_contents'\n        AND tgrelid = 'contents'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_updated_at_on_contents\n        BEFORE INSERT OR UPDATE ON contents\n        FOR EACH ROW\n        EXECUTE FUNCTION set_updated_at();\n    END IF;\nEND $$",
        "CREATE INDEX IF NOT EXISTS idx_contents_creator_id ON contents(creator_id);"]
        {
            tracing :: trace! ("Execute queries: {}", query); sqlx ::
            query(query).execute(& self.pool).await ? ;
        } Ok(())
    } pub async fn create_table(& self) -> std :: result :: Result < (), sqlx
    :: Error >
    {
        sqlx ::
        query("CREATE TABLE IF NOT EXISTS contents (id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,created_at BIGINT NOT NULL,updated_at BIGINT NOT NULL,title TEXT NOT NULL,thumbnail_image TEXT NOT NULL,source TEXT NOT NULL,description TEXT NOT NULL,creator_id BIGINT NOT NULL, FOREIGN KEY (creator_id) REFERENCES users(id) ON DELETE CASCADE);").execute(&
        self.pool).await ? ; for query in
        ["DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_created_at_on_contents'\n        AND tgrelid = 'contents'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_created_at_on_contents\n        BEFORE INSERT ON contents\n        FOR EACH ROW\n        EXECUTE FUNCTION set_created_at();\n    END IF;\nEND $$",
        "DO $$\nBEGIN\n    IF NOT EXISTS (\n        SELECT 1\n        FROM pg_trigger\n        WHERE tgname = 'trigger_updated_at_on_contents'\n        AND tgrelid = 'contents'::regclass\n    ) THEN\n        CREATE TRIGGER trigger_updated_at_on_contents\n        BEFORE INSERT OR UPDATE ON contents\n        FOR EACH ROW\n        EXECUTE FUNCTION set_updated_at();\n    END IF;\nEND $$",
        "CREATE INDEX IF NOT EXISTS idx_contents_creator_id ON contents(creator_id);"]
        {
            tracing :: trace! ("Execute queries: {}", query); sqlx ::
            query(query).execute(& self.pool).await ? ;
        } Ok(())
    } pub async fn drop_table(& self) -> std :: result :: Result < (), sqlx ::
    Error >
    {
        sqlx ::
        query("DROP TABLE IF EXISTS contents;").execute(& self.pool).await ? ;
        Ok(())
    } pub async fn
    insert(& self, title : String, thumbnail_image : String, source : String,
    description : String, creator_id : i64) -> crate::Result < Content >
    {
        tracing :: trace!
        ("insert query: {}",
        "INSERT INTO contents (title, thumbnail_image, source, description, creator_id) VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, updated_at, title, thumbnail_image, source, description, creator_id");
        let row = sqlx ::
        query("INSERT INTO contents (title, thumbnail_image, source, description, creator_id) VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, updated_at, title, thumbnail_image, source, description, creator_id").bind(title).bind(thumbnail_image).bind(source).bind(description).bind(creator_id).map(|
        row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; Content
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), title :
                row.try_get("title").unwrap_or_default(), thumbnail_image :
                row.try_get("thumbnail_image").unwrap_or_default(), source :
                row.try_get("source").unwrap_or_default(), description :
                row.try_get("description").unwrap_or_default(), creator_id :
                row.try_get("creator_id").unwrap_or_default()
            }
        }).fetch_one(& self.pool).await ? ; Ok(row)
    } pub async fn insert_with_tx < 'e, 'c : 'e, E >
    (& self, tx : E, title : String, thumbnail_image : String, source :
    String, description : String, creator_id : i64) -> crate::Result < Option
    < Content >> where E : sqlx :: Executor < 'c, Database = sqlx :: postgres
    :: Postgres > ,
    {
        tracing :: trace!
        ("insert query: {}",
        "INSERT INTO contents (title, thumbnail_image, source, description, creator_id) VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, updated_at, title, thumbnail_image, source, description, creator_id");
        let row = sqlx ::
        query("INSERT INTO contents (title, thumbnail_image, source, description, creator_id) VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, updated_at, title, thumbnail_image, source, description, creator_id").bind(title).bind(thumbnail_image).bind(source).bind(description).bind(creator_id).map(|
        row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; Content
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), title :
                row.try_get("title").unwrap_or_default(), thumbnail_image :
                row.try_get("thumbnail_image").unwrap_or_default(), source :
                row.try_get("source").unwrap_or_default(), description :
                row.try_get("description").unwrap_or_default(), creator_id :
                row.try_get("creator_id").unwrap_or_default()
            }
        }).fetch_optional(tx).await ? ; Ok(row)
    } pub async fn
    insert_without_returning(& self, title : String, thumbnail_image : String,
    source : String, description : String, creator_id : i64) -> crate::Result
    < () >
    {
        tracing :: trace!
        ("insert query: {}",
        "INSERT INTO contents (title, thumbnail_image, source, description, creator_id) VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, updated_at, title, thumbnail_image, source, description, creator_id");
        sqlx ::
        query("INSERT INTO contents (title, thumbnail_image, source, description, creator_id) VALUES ($1, $2, $3, $4, $5)").bind(title).bind(thumbnail_image).bind(source).bind(description).bind(creator_id).execute(&
        self.pool).await ? ; Ok(())
    } pub async fn
    update(& self, id : i64, content_repository_update_request :
    ContentRepositoryUpdateRequest) -> crate::Result < Content >
    {
        let mut i = 1; let mut update_values = vec! []; if
        content_repository_update_request.title.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "title", i)); } if
        content_repository_update_request.thumbnail_image.is_some()
        {
            i += 1;
            update_values.push(format! ("{} = ${}", "thumbnail_image", i));
        } if content_repository_update_request.source.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "source", i)); } if
        content_repository_update_request.description.is_some()
        {
            i += 1;
            update_values.push(format! ("{} = ${}", "description", i));
        } if content_repository_update_request.creator_id.is_some()
        { i += 1; update_values.push(format! ("{} = ${}", "creator_id", i)); }
        let query = format!
        ("UPDATE contents SET {} WHERE id = $1 RETURNING id, created_at, updated_at, title, thumbnail_image, source, description, creator_id",
        update_values.join(", "),); tracing :: trace!
        ("insert query: {}", query); let mut q = sqlx ::
        query(& query).bind(id); if let Some(title) =
        content_repository_update_request.title { q = q.bind(title); } if let
        Some(thumbnail_image) =
        content_repository_update_request.thumbnail_image
        { q = q.bind(thumbnail_image); } if let Some(source) =
        content_repository_update_request.source { q = q.bind(source); } if
        let Some(description) = content_repository_update_request.description
        { q = q.bind(description); } if let Some(creator_id) =
        content_repository_update_request.creator_id
        { q = q.bind(creator_id); } let row =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; Content
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), title :
                row.try_get("title").unwrap_or_default(), thumbnail_image :
                row.try_get("thumbnail_image").unwrap_or_default(), source :
                row.try_get("source").unwrap_or_default(), description :
                row.try_get("description").unwrap_or_default(), creator_id :
                row.try_get("creator_id").unwrap_or_default()
            }
        }).fetch_one(& self.pool).await ? ; Ok(row)
    } pub async fn delete(& self, id : i64) -> crate::Result < () >
    {
        sqlx ::
        query("DELETE FROM contents WHERE id = $1").bind(id).execute(&
        self.pool).await ? ; Ok(())
    } pub async fn find_one(& self, param : & ContentReadAction) ->
    crate::Result < Content >
    {
        let mut query = format! ("{}", Content :: base_sql());
        query.push_str(" "); query.push_str(Content :: group_by().as_str());
        tracing :: trace!
        ("{} query {}: {:?}", "ContentRepository::find_one", query, param);
        let mut q = sqlx :: query(& query); let row =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; Content
            {
                id : row.try_get("id").unwrap_or_default(), created_at :
                row.try_get("created_at").unwrap_or_default(), updated_at :
                row.try_get("updated_at").unwrap_or_default(), title :
                row.try_get("title").unwrap_or_default(), thumbnail_image :
                row.try_get("thumbnail_image").unwrap_or_default(), source :
                row.try_get("source").unwrap_or_default(), description :
                row.try_get("description").unwrap_or_default(), creator_id :
                row.try_get("creator_id").unwrap_or_default()
            }
        }).fetch_one(& self.pool).await ? ; Ok(row)
    } pub async fn find(& self, param : & ContentQuery) -> crate::Result <
    by_types::QueryResponse<ContentSummary> >
    {
        let mut i = 2; let mut where_clause = vec! []; if let
        Some(description) = & param.description
        { i += 1; where_clause.push(format! ("{} = ${}", "description", i)); }
        let where_clause_str = where_clause.join(" AND "); let query = if
        where_clause.len() > 0
        {
            format!
            ("{} WHERE {} {}", "SELECT * FROM contents", where_clause_str,
            "LIMIT $1 OFFSET $2")
        } else
        { format! ("{} {}", "SELECT * FROM contents", "LIMIT $1 OFFSET $2") };
        let count_query = if where_clause.len() > 0
        {
            format!
            ("{} WHERE {}", "SELECT COUNT(*) FROM contents", where_clause_str)
        } else { format! ("{}", "SELECT COUNT(*) FROM contents") }; let query
        = format!
        ("WITH data AS ({}) SELECT ({}) AS total_count, data.* FROM data;",
        query, count_query); tracing :: trace!
        ("{} query {}", "ContentRepository::find_one", query); let offset :
        i32 = (param.size as i32) * (param.page() - 1); let mut q = sqlx ::
        query(& query).bind(param.size as i32).bind(offset); if let
        Some(description) = & param.description
        {
            tracing :: trace!
            ("{} binding {} = {}", "ContentRepository::find_one",
            "description", description); q = q.bind(description);
        } let mut total : i64 = 0; let rows =
        q.map(| row : sqlx :: postgres :: PgRow |
        {
            use sqlx :: Row; total = row.get("total_count"); row.into()
        }).fetch_all(& self.pool).await ? ; Ok((rows, total).into())
    }
} impl From < sqlx :: postgres :: PgRow > for Content
{
    fn from(row : sqlx :: postgres :: PgRow) -> Self
    {
        use sqlx :: Row; Content
        {
            id : row.try_get("id").unwrap_or_default(), created_at :
            row.try_get("created_at").unwrap_or_default(), updated_at :
            row.try_get("updated_at").unwrap_or_default(), title :
            row.try_get("title").unwrap_or_default(), thumbnail_image :
            row.try_get("thumbnail_image").unwrap_or_default(), source :
            row.try_get("source").unwrap_or_default(), description :
            row.try_get("description").unwrap_or_default(), creator_id :
            row.try_get("creator_id").unwrap_or_default()
        }
    }
} impl From < sqlx :: postgres :: PgRow > for ContentSummary
{
    fn from(row : sqlx :: postgres :: PgRow) -> Self
    {
        use sqlx :: Row; ContentSummary
        {
            id : row.try_get("id").unwrap_or_default(), created_at :
            row.try_get("created_at").unwrap_or_default(), updated_at :
            row.try_get("updated_at").unwrap_or_default(), title :
            row.try_get("title").unwrap_or_default(), thumbnail_image :
            row.try_get("thumbnail_image").unwrap_or_default(), source :
            row.try_get("source").unwrap_or_default()
        }
    }
} #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default)]
pub struct ContentRepositoryQueryBuilder
{
    pub base_sql : String, pub group_by : String, pub count : bool, pub
    conditions : Vec < by_types :: Conditions > , pub order : by_types ::
    Order, pub limit : Option < i32 > , pub page : Option < i32 > ,
} impl ContentRepositoryQueryBuilder
{
    pub fn from(base_sql : & str, group_by : & str) -> Self
    {
        Self
        {
            base_sql : base_sql.to_string(), group_by : group_by.to_string(),
            .. Default :: default()
        }
    } pub fn with_count(mut self) -> Self { self.count = true; self } pub fn
    new() -> Self { Self :: default() } pub fn limit(mut self, limit : i32) ->
    Self { self.limit = Some(limit); self } pub fn page(mut self, page : i32)
    -> Self { self.page = Some(page); self } pub fn build_where(& self) ->
    String
    {
        let mut where_clause = vec! []; let mut i = 1; tracing :: debug!
        ("Building where clause with group: {}", self.group_by); let prefix =
        if self.group_by.is_empty() { "" } else { "p." }; for condition in
        self.conditions.iter()
        {
            let (q, new_i) = condition.to_binder(i); i = new_i;
            where_clause.push(format! ("{}{}", prefix, q));
        } where_clause.join(" AND ")
    } pub fn sql(& self) -> String
    {
        let w = self.build_where(); let mut query = if w.is_empty()
        { format! ("{} {} {}", self.base_sql, self.group_by, self.order) }
        else
        {
            format!
            ("{} WHERE {} {} {}", self.base_sql, w, self.group_by, self.order)
        }; if self.count && !
        query.starts_with("SELECT COUNT(*) OVER() as total_count")
        {
            query =
            query.replacen("SELECT", "SELECT COUNT(*) OVER() as total_count,",
            1);
        } if let Some(limit) = self.limit
        {
            if let Some(page) = self.page
            {
                format!
                ("{} LIMIT {} OFFSET {}", query, limit, (limit * (page - 1)))
            } else { format! ("{} LIMIT {}", query, limit) }
        } else { query }
    } pub fn query(& self,) -> sqlx :: query :: Query < 'static, sqlx ::
    Postgres, < sqlx :: Postgres as sqlx :: Database > :: Arguments < 'static
    > , >
    {
        let mut query = self.sql(); let s : Box < String > = Box ::
        new(query); let query : & 'static str = Box :: leak(s); let mut q =
        sqlx :: query(query); for condition in self.conditions.clone()
        {
            q = match condition
            {
                by_types :: Conditions :: BetweenBigint(_, from, to) =>
                {
                    tracing :: debug!
                    ("Binding BetweenBigint {} and {}", from, to);
                    q.bind(from).bind(to)
                }, by_types :: Conditions :: EqualsBigint(_, value) =>
                {
                    tracing :: debug! ("Binding EqualsBigint {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: NotEqualsBigint(_, value) =>
                {
                    tracing :: debug! ("Binding NotEqualsBigint {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: GreaterThanBigint(_, value) =>
                {
                    tracing :: debug! ("Binding GreaterThanBigint {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: LessThanBigint(_, value) =>
                {
                    tracing :: debug! ("Binding LessThanBigint {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: GreaterThanEqualsBigint(_, value)
                =>
                {
                    tracing :: debug!
                    ("Binding GreaterThanEqualsBigint {}", value); q.bind(value)
                }, by_types :: Conditions :: LessThanEqualsBigint(_, value) =>
                {
                    tracing :: debug!
                    ("Binding LessThanEqualsBigint {}", value); q.bind(value)
                }, by_types :: Conditions :: BetweenInteger(_, from, to) =>
                {
                    tracing :: debug!
                    ("Binding BetweenInteger {} and {}", from, to);
                    q.bind(from).bind(to)
                }, by_types :: Conditions :: EqualsInteger(_, value) =>
                {
                    tracing :: debug! ("Binding EqualsInteger {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: NotEqualsInteger(_, value) =>
                {
                    tracing :: debug! ("Binding NotEqualsInteger {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: GreaterThanInteger(_, value) =>
                {
                    tracing :: debug! ("Binding GreaterThanInteger {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: LessThanInteger(_, value) =>
                {
                    tracing :: debug! ("Binding LessThanInteger {}", value);
                    q.bind(value)
                }, by_types :: Conditions ::
                GreaterThanEqualsInteger(_, value) =>
                {
                    tracing :: debug!
                    ("Binding GreaterThanEqualsInteger {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: LessThanEqualsInteger(_, value)
                =>
                {
                    tracing :: debug!
                    ("Binding LessThanEqualsInteger {}", value); q.bind(value)
                }, by_types :: Conditions :: EqualsText(_, value) =>
                {
                    tracing :: debug! ("Binding EqualsText {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: NotEqualsText(_, value) =>
                {
                    tracing :: debug! ("Binding NotEqualsText {}", value);
                    q.bind(value)
                }, by_types :: Conditions :: ContainsText(_, value) =>
                {
                    let value = format! ("%{}%", value); tracing :: debug!
                    ("Binding ContainsText {}", value); q.bind(value)
                }, by_types :: Conditions :: NotContainsText(_, value) =>
                {
                    let value = format! ("%{}%", value); tracing :: debug!
                    ("Binding NotContainsText {}", value); q.bind(value)
                } by_types :: Conditions :: StartsWithText(_, value) =>
                {
                    let value = format! ("{}%", value); tracing :: debug!
                    ("Binding StartsWithText {}", value); q.bind(value)
                } by_types :: Conditions :: NotStartsWithText(_, value) =>
                {
                    let value = format! ("{}%", value); tracing :: debug!
                    ("Binding NotStartsWithText {}", value); q.bind(value)
                } by_types :: Conditions :: EndsWithText(_, value) =>
                {
                    let value = format! ("%{}", value); tracing :: debug!
                    ("Binding EndsWithText {}", value); q.bind(value)
                } by_types :: Conditions :: NotEndsWithText(_, value) =>
                {
                    let value = format! ("%{}", value); tracing :: debug!
                    ("Binding NotEndsWithText {}", value); q.bind(value)
                } by_types :: Conditions :: TrueBoolean(_) =>
                { tracing :: debug! ("(Not)Binding TrueBoolean"); q } by_types
                :: Conditions :: FalseBoolean(_) =>
                { tracing :: debug! ("(Not)Binding FalseBoolean"); q }
            };
        } q
    } pub fn id_equals(mut self, id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsBigint("id".to_string(), id)); self
    } pub fn id_not_equals(mut self, id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsBigint("id".to_string(), id)); self
    } pub fn id_greater_than(mut self, id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanBigint("id".to_string(), id)); self
    } pub fn id_less_than(mut self, id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanBigint("id".to_string(), id)); self
    } pub fn id_greater_than_equals(mut self, id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanEqualsBigint("id".to_string(), id)); self
    } pub fn id_less_than_equals(mut self, id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanEqualsBigint("id".to_string(), id)); self
    } pub fn id_between(mut self, from : i64, to : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        BetweenBigint("id".to_string(), from, to)); self
    } pub fn order_by_id_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "id")); } else
        { self.order = by_types :: Order :: Asc(vec! ["id".to_string()]); }
        self
    } pub fn order_by_id_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "id")); } else
        { self.order = by_types :: Order :: Desc(vec! ["id".to_string()]); }
        self
    } pub fn created_at_equals(mut self, created_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsBigint("created_at".to_string(), created_at)); self
    } pub fn created_at_not_equals(mut self, created_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsBigint("created_at".to_string(), created_at)); self
    } pub fn created_at_greater_than(mut self, created_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanBigint("created_at".to_string(), created_at)); self
    } pub fn created_at_less_than(mut self, created_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanBigint("created_at".to_string(), created_at)); self
    } pub fn created_at_greater_than_equals(mut self, created_at : i64) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanEqualsBigint("created_at".to_string(), created_at)); self
    } pub fn created_at_less_than_equals(mut self, created_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanEqualsBigint("created_at".to_string(), created_at)); self
    } pub fn created_at_between(mut self, from : i64, to : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        BetweenBigint("created_at".to_string(), from, to)); self
    } pub fn order_by_created_at_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "created_at")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["created_at".to_string()]);
        } self
    } pub fn order_by_created_at_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "created_at")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["created_at".to_string()]);
        } self
    } pub fn updated_at_equals(mut self, updated_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsBigint("updated_at".to_string(), updated_at)); self
    } pub fn updated_at_not_equals(mut self, updated_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsBigint("updated_at".to_string(), updated_at)); self
    } pub fn updated_at_greater_than(mut self, updated_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanBigint("updated_at".to_string(), updated_at)); self
    } pub fn updated_at_less_than(mut self, updated_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanBigint("updated_at".to_string(), updated_at)); self
    } pub fn updated_at_greater_than_equals(mut self, updated_at : i64) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanEqualsBigint("updated_at".to_string(), updated_at)); self
    } pub fn updated_at_less_than_equals(mut self, updated_at : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanEqualsBigint("updated_at".to_string(), updated_at)); self
    } pub fn updated_at_between(mut self, from : i64, to : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        BetweenBigint("updated_at".to_string(), from, to)); self
    } pub fn order_by_updated_at_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "updated_at")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["updated_at".to_string()]);
        } self
    } pub fn order_by_updated_at_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "updated_at")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["updated_at".to_string()]);
        } self
    } pub fn title_equals(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("title".to_string(), title)); self
    } pub fn title_not_equals(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("title".to_string(), title)); self
    } pub fn title_contains(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("title".to_string(), title)); self
    } pub fn title_not_contains(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("title".to_string(), title)); self
    } pub fn title_starts_with(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("title".to_string(), title)); self
    } pub fn title_not_starts_with(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("title".to_string(), title)); self
    } pub fn title_ends_with(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("title".to_string(), title)); self
    } pub fn title_not_ends_with(mut self, title : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("title".to_string(), title)); self
    } pub fn order_by_title_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "title")); } else
        { self.order = by_types :: Order :: Asc(vec! ["title".to_string()]); }
        self
    } pub fn order_by_title_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "title")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["title".to_string()]);
        } self
    } pub fn thumbnail_image_equals(mut self, thumbnail_image : String) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("thumbnail_image".to_string(), thumbnail_image)); self
    } pub fn thumbnail_image_not_equals(mut self, thumbnail_image : String) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("thumbnail_image".to_string(), thumbnail_image)); self
    } pub fn thumbnail_image_contains(mut self, thumbnail_image : String) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("thumbnail_image".to_string(), thumbnail_image)); self
    } pub fn thumbnail_image_not_contains(mut self, thumbnail_image : String)
    -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("thumbnail_image".to_string(), thumbnail_image)); self
    } pub fn thumbnail_image_starts_with(mut self, thumbnail_image : String)
    -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("thumbnail_image".to_string(), thumbnail_image)); self
    } pub fn
    thumbnail_image_not_starts_with(mut self, thumbnail_image : String) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("thumbnail_image".to_string(), thumbnail_image));
        self
    } pub fn thumbnail_image_ends_with(mut self, thumbnail_image : String) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("thumbnail_image".to_string(), thumbnail_image)); self
    } pub fn thumbnail_image_not_ends_with(mut self, thumbnail_image : String)
    -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("thumbnail_image".to_string(), thumbnail_image)); self
    } pub fn order_by_thumbnail_image_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "thumbnail_image")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["thumbnail_image".to_string()]);
        } self
    } pub fn order_by_thumbnail_image_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "thumbnail_image")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["thumbnail_image".to_string()]);
        } self
    } pub fn source_equals(mut self, source : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("source".to_string(), source)); self
    } pub fn source_not_equals(mut self, source : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("source".to_string(), source)); self
    } pub fn source_contains(mut self, source : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("source".to_string(), source)); self
    } pub fn source_not_contains(mut self, source : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("source".to_string(), source)); self
    } pub fn source_starts_with(mut self, source : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("source".to_string(), source)); self
    } pub fn source_not_starts_with(mut self, source : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("source".to_string(), source)); self
    } pub fn source_ends_with(mut self, source : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("source".to_string(), source)); self
    } pub fn source_not_ends_with(mut self, source : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("source".to_string(), source)); self
    } pub fn order_by_source_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "source")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["source".to_string()]);
        } self
    } pub fn order_by_source_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "source")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["source".to_string()]);
        } self
    } pub fn description_equals(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsText("description".to_string(), description)); self
    } pub fn description_not_equals(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsText("description".to_string(), description)); self
    } pub fn description_contains(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        ContainsText("description".to_string(), description)); self
    } pub fn description_not_contains(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotContainsText("description".to_string(), description)); self
    } pub fn description_starts_with(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        StartsWithText("description".to_string(), description)); self
    } pub fn description_not_starts_with(mut self, description : String) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotStartsWithText("description".to_string(), description)); self
    } pub fn description_ends_with(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EndsWithText("description".to_string(), description)); self
    } pub fn description_not_ends_with(mut self, description : String) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEndsWithText("description".to_string(), description)); self
    } pub fn order_by_description_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "description")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["description".to_string()]);
        } self
    } pub fn order_by_description_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "description")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["description".to_string()]);
        } self
    } pub fn creator_id_equals(mut self, creator_id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        EqualsBigint("creator_id".to_string(), creator_id)); self
    } pub fn creator_id_not_equals(mut self, creator_id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        NotEqualsBigint("creator_id".to_string(), creator_id)); self
    } pub fn creator_id_greater_than(mut self, creator_id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanBigint("creator_id".to_string(), creator_id)); self
    } pub fn creator_id_less_than(mut self, creator_id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanBigint("creator_id".to_string(), creator_id)); self
    } pub fn creator_id_greater_than_equals(mut self, creator_id : i64) ->
    Self
    {
        self.conditions.push(by_types :: Conditions ::
        GreaterThanEqualsBigint("creator_id".to_string(), creator_id)); self
    } pub fn creator_id_less_than_equals(mut self, creator_id : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        LessThanEqualsBigint("creator_id".to_string(), creator_id)); self
    } pub fn creator_id_between(mut self, from : i64, to : i64) -> Self
    {
        self.conditions.push(by_types :: Conditions ::
        BetweenBigint("creator_id".to_string(), from, to)); self
    } pub fn order_by_creator_id_asc(mut self) -> Self
    {
        if let by_types :: Order :: Asc(ref mut field) = self.order
        { field.push(format! (",{}", "creator_id")); } else
        {
            self.order = by_types :: Order ::
            Asc(vec! ["creator_id".to_string()]);
        } self
    } pub fn order_by_creator_id_desc(mut self) -> Self
    {
        if let by_types :: Order :: Desc(ref mut field) = self.order
        { field.push(format! (",{}", "creator_id")); } else
        {
            self.order = by_types :: Order ::
            Desc(vec! ["creator_id".to_string()]);
        } self
    }
} /// Content is a generated struct that represents the model
///
/// For making API calls related to this model, use `Content::get_client(endpoint: &str)`.
/// It will returns ContentClient struct that implements the API calls.
///
/// In server side, you can use `Content::get_repository()` to interact with the database.
/// Recommend to use `ContentRepository` to insert or update the model.
/// To query the model, use `Content::query_builder()`.
/// For more detail, refer to the documentation of the query builder.
#[derive(Debug, Clone, serde :: Deserialize, serde :: Serialize, Default,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct Content
{
    pub id : i64, pub created_at : i64, pub updated_at : i64, pub title :
    String, #[validate(url)] pub thumbnail_image : String, #[validate(url)]
    pub source : String, #[validate(length(min = 1, max = 300))] pub
    description : String, pub creator_id : i64
} #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub enum ContentAction
{ CreateBulk(ContentCreateBulkRequest), Create(ContentCreateRequest), } impl
validator :: Validate for ContentAction
{
    fn validate(& self) -> std :: result :: Result < (), validator ::
    ValidationErrors >
    {
        match self
        {
            ContentAction :: CreateBulk(req) => req.validate(), ContentAction
            :: Create(req) => req.validate(),
        }
    }
} #[derive(validator :: Validate)]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
ContentCreateBulkRequest { pub items : Vec<ContentCreateRequest> , }
#[derive(validator :: Validate)]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
ContentCreateRequest
{
    pub title : String, #[validate(url)] pub thumbnail_image : String,
    #[validate(url)] pub source : String,
    #[validate(length(min = 1, max = 300))] pub description : String, pub
    creator_id : i64,
} impl ContentClient
{
    pub async fn act(& self, params : ContentAction) -> crate::Result <
    Content >
    {
        let path = format! ("/v1/contents",); let endpoint = format!
        ("{}{}", self.endpoint, path); rest_api ::
        post(& endpoint, params).await
    } pub async fn create_bulk(& self, items : Vec<ContentCreateRequest> ,) ->
    crate::Result < Content >
    {
        let path = format! ("/v1/contents",); let endpoint = format!
        ("{}{}", self.endpoint, path); let req = ContentAction ::
        CreateBulk(ContentCreateBulkRequest { items, }); rest_api ::
        post(& endpoint, req).await
    } pub async fn
    create(& self, title : String, thumbnail_image : String, source : String,
    description : String, creator_id : i64,) -> crate::Result < Content >
    {
        let path = format! ("/v1/contents",); let endpoint = format!
        ("{}{}", self.endpoint, path); let req = ContentAction ::
        Create(ContentCreateRequest
        { title, thumbnail_image, source, description, creator_id, });
        rest_api :: post(& endpoint, req).await
    }
} #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub enum
ContentByIdAction { Mint(ContentMintRequest), } impl validator :: Validate for
ContentByIdAction
{
    fn validate(& self) -> std :: result :: Result < (), validator ::
    ValidationErrors >
    { match self { ContentByIdAction :: Mint(req) => req.validate(), } }
} #[derive(validator :: Validate)]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
ContentMintRequest {} impl Into < ContentRepositoryUpdateRequest > for
ContentMintRequest
{
    fn into(self) -> ContentRepositoryUpdateRequest
    { ContentRepositoryUpdateRequest { .. Default :: default() } }
} impl ContentClient
{
    pub async fn act_by_id(& self, id : i64, params : ContentByIdAction) ->
    crate::Result < Content >
    {
        let path = format! ("/v1/contents",); let endpoint = format!
        ("{}{}/{}", self.endpoint, path, id); rest_api ::
        post(& endpoint, params).await
    } pub async fn mint(& self, id : i64,) -> crate::Result < Content >
    {
        let path = format! ("/v1/contents",); let endpoint = format!
        ("{}{}/{}", self.endpoint, path, id); let req = ContentByIdAction ::
        Mint(ContentMintRequest {}); rest_api :: post(& endpoint, req).await
    }
} impl Into < ContentRepositoryUpdateRequest > for ContentByIdAction
{
    fn into(self) -> ContentRepositoryUpdateRequest
    { match self { ContentByIdAction :: Mint(req) => req.into(), } }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo, sqlx :: FromRow))] pub
struct ContentSummary
{
    pub id : i64, pub created_at : i64, pub updated_at : i64, pub title :
    String, pub thumbnail_image : String, pub source : String,
} impl From < Content > for ContentSummary
{
    fn from(item : Content) -> Self
    {
        Self
        {
            id : item.id, created_at : item.created_at, updated_at :
            item.updated_at, title : item.title, thumbnail_image :
            item.thumbnail_image, source : item.source,
        }
    }
} impl Into < Content > for ContentSummary
{
    fn into(self) -> Content
    {
        Content
        {
            id : self.id, created_at : self.created_at, updated_at :
            self.updated_at, title : self.title, thumbnail_image :
            self.thumbnail_image, source : self.source, .. Default ::
            default()
        }
    }
} #[derive(validator :: Validate)]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq, by_macros :: QueryDisplay)] #[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct ContentQuery
{
    #[serde(deserialize_with = "parse_size_of_content_query")] pub size :
    usize, pub bookmark : Option < String > , pub action : Option <
    ContentQueryActionType > , #[validate(length(min = 1, max = 300))] pub
    description : Option < String > ,
} pub fn parse_size_of_content_query < 'de, D > (deserializer : D) -> std ::
result :: Result < usize, D :: Error > where D : serde :: Deserializer < 'de >
,
{
    use serde :: Deserialize; let s : Option < String > = Option ::
    deserialize(deserializer) ? ;
    s.unwrap_or_else(|| Default :: default()).parse :: < usize >
    ().map_err(serde :: de :: Error :: custom)
} impl ContentQuery
{
    pub fn new(size : usize) -> Self { Self { size, .. Self :: default() } }
    pub fn with_bookmark(mut self, bookmark : String) -> Self
    { self.bookmark = Some(bookmark); self } pub fn
    with_page(mut self, page : usize) -> Self
    { self.bookmark = Some(page.to_string()); self }
    #[doc = r" Returns the size(i32) of the query"] pub fn size(& self) -> i32
    { self.size as i32 } pub fn page(& self) -> i32
    {
        self.bookmark.as_ref().unwrap_or(&
        "1".to_string()).parse().unwrap_or(1)
    } pub fn search(mut self, description : String,) -> Self
    {
        self.description = Some(description); self.action =
        Some(ContentQueryActionType :: Search); self
    }
} #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub enum
ContentQueryActionType { Search, } impl ContentClient
{
    pub async fn
    search(& self, size : usize, bookmark : Option < String > , description :
    String,) -> crate::Result < by_types::QueryResponse<ContentSummary> >
    {
        let path = format! ("/v1/contents",); let endpoint = format!
        ("{}{}", self.endpoint, path); let params = ContentParam ::
        Query(ContentQuery
        {
            size, bookmark, action : Some(ContentQueryActionType :: Search),
            description : Some(description), .. ContentQuery :: default()
        }); let query = format! ("{}?{}", endpoint, params); rest_api ::
        get(& query).await
    }
} impl Content
{
    pub fn get_client(endpoint : & str) -> ContentClient
    { ContentClient { endpoint : endpoint.to_string() } }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq)] pub struct ContentClient { pub endpoint : String, } impl
ContentClient
{
    pub async fn query(& self, params : ContentQuery,) -> crate::Result <
    by_types::QueryResponse<ContentSummary> >
    {
        let path = format! ("/v1/contents",); let endpoint = format!
        ("{}{}", self.endpoint, path); let query = format!
        ("{}?{}", endpoint, ContentParam :: Query(params)); rest_api ::
        get(& query).await
    } pub async fn get(& self, id : i64) -> crate::Result < Content >
    {
        let path = format! ("/v1/contents",); let endpoint = format!
        ("{}{}/{}", self.endpoint, path, id); rest_api ::
        get(& endpoint).await
    }
} #[derive(validator :: Validate)]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq, by_macros :: QueryDisplay)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
ContentReadAction {} impl ContentReadAction
{ pub fn new() -> Self { Self :: default() } } impl ContentClient {}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, PartialEq,
by_macros :: QueryDisplay)]
#[cfg_attr(feature = "server", derive(aide :: OperationIo))]
#[serde(tag = "param-type", rename_all = "kebab-case")] pub enum ContentParam
{ Query(ContentQuery), } #[cfg(feature = "server")] impl schemars ::
JsonSchema for ContentParam
{
    fn schema_name() -> String { "ContentParam".to_string() } fn
    json_schema(_gen : & mut schemars :: gen :: SchemaGenerator) -> schemars
    :: schema :: Schema
    {
        let mut schema_obj = schemars :: schema :: SchemaObject :: default();
        schema_obj.metadata =
        Some(Box ::
        new(schemars :: schema :: Metadata
        {
            title : Some("Content Query Parameters".to_string()), .. Default
            :: default()
        }));
        schema_obj.object().properties.insert("action".to_string(), schemars
        :: schema :: Schema ::
        Object(schemars :: schema :: SchemaObject
        {
            metadata :
            Some(Box ::
            new(schemars :: schema :: Metadata
            {
                description : Some("request action type".to_string()), ..
                Default :: default()
            })), instance_type :
            Some(schemars :: schema :: InstanceType :: String.into()),
            enum_values :
            Some(vec! [serde_json :: Value :: String("search".to_string()),]),
            .. Default :: default()
        }),);
        schema_obj.object().properties.insert("size".to_string(), schemars ::
        schema :: Schema ::
        Object(schemars :: schema :: SchemaObject
        {
            metadata :
            Some(Box ::
            new(schemars :: schema :: Metadata
            {
                description : Some("Number of items to return".to_string()),
                .. Default :: default()
            })), instance_type :
            Some(schemars :: schema :: InstanceType :: Integer.into()), ..
            Default :: default()
        }),);
        schema_obj.object().properties.insert("bookmark".to_string(), schemars
        :: schema :: Schema ::
        Object(schemars :: schema :: SchemaObject
        {
            metadata :
            Some(Box ::
            new(schemars :: schema :: Metadata
            {
                description :
                Some("bookmark of page number. Note that you must stringify page number.".to_string()),
                default :
                Some(serde_json :: Value :: String("1".to_string())), ..
                Default :: default()
            })), instance_type :
            Some(schemars :: schema :: InstanceType :: String.into()), ..
            Default :: default()
        }),);
        schema_obj.object().properties.insert("description".to_string(),
        schemars :: schema :: Schema ::
        Object(schemars :: schema :: SchemaObject
        {
            metadata :
            Some(Box ::
            new(schemars :: schema :: Metadata
            {
                description :
                Some("This field is used in the following actions: search".to_string()),
                .. Default :: default()
            })), instance_type :
            Some(schemars :: schema :: InstanceType :: String.into()), ..
            Default :: default()
        }),); schema_obj.object().required.insert("size".to_string());
        schemars :: schema :: Schema :: Object(schema_obj)
    }
} #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(tag = "param_type")] #[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub enum
ContentGetResponse { Query(by_types::QueryResponse<ContentSummary>), }