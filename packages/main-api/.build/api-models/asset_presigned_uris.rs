/// AssetPresignedUris is a generated struct that represents the model
///
/// For making API calls related to this model, use `AssetPresignedUris::get_client(endpoint: &str)`.
/// It will returns AssetPresignedUrisClient struct that implements the API calls.
///
/// In server side, you can use `AssetPresignedUris::get_repository()` to interact with the database.
/// Recommend to use `AssetPresignedUrisRepository` to insert or update the model.
/// To query the model, use `AssetPresignedUris::query_builder()`.
/// For more detail, refer to the documentation of the query builder.
#[derive(Debug, Clone, serde :: Deserialize, serde :: Serialize, Default,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
AssetPresignedUris
{
    pub presigned_uris : Vec < String > , pub uris : Vec < String > , pub
    total_count : usize, pub file_type : FileType
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo, sqlx :: FromRow))] pub
struct AssetPresignedUrisSummary {} impl From < AssetPresignedUris > for
AssetPresignedUrisSummary
{ fn from(item : AssetPresignedUris) -> Self { Self {} } } impl Into <
AssetPresignedUris > for AssetPresignedUrisSummary
{
    fn into(self) -> AssetPresignedUris
    { AssetPresignedUris { .. Default :: default() } }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq, by_macros :: QueryDisplay)] #[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
AssetPresignedUrisQuery
{
    #[serde(deserialize_with = "parse_size_of_asset_presigned_uris_query")]
    pub size : usize, pub bookmark : Option < String > ,
} pub fn parse_size_of_asset_presigned_uris_query < 'de, D >
(deserializer : D) -> std :: result :: Result < usize, D :: Error > where D :
serde :: Deserializer < 'de > ,
{
    use serde :: Deserialize; let s : Option < String > = Option ::
    deserialize(deserializer) ? ;
    s.unwrap_or_else(|| Default :: default()).parse :: < usize >
    ().map_err(serde :: de :: Error :: custom)
} impl AssetPresignedUrisQuery
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
    }
} impl AssetPresignedUrisClient {} impl AssetPresignedUris
{
    pub fn get_client(endpoint : & str) -> AssetPresignedUrisClient
    { AssetPresignedUrisClient { endpoint : endpoint.to_string() } }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq)] pub struct AssetPresignedUrisClient { pub endpoint : String, }
impl AssetPresignedUrisClient
{
    pub async fn query(& self, params : AssetPresignedUrisQuery,) -> crate ::
    Result < by_types::QueryResponse<AssetPresignedUrisSummary> >
    {
        let path = format! ("/v1/metadata",); let endpoint = format!
        ("{}{}", self.endpoint, path); let query = format!
        ("{}?{}", endpoint, AssetPresignedUrisParam :: Query(params));
        rest_api :: get(& query).await
    } pub async fn get(& self, id : i64) -> crate :: Result <
    AssetPresignedUris >
    {
        let path = format! ("/v1/metadata",); let endpoint = format!
        ("{}{}/{}", self.endpoint, path, id); rest_api ::
        get(& endpoint).await
    }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, Default,
PartialEq, by_macros :: QueryDisplay)]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub struct
AssetPresignedUrisReadAction
{
    pub action : Option < AssetPresignedUrisReadActionType > ,
    #[serde(deserialize_with =
    "parse_total_count_of_asset_presigned_uris_read")] pub total_count :
    Option < usize > ,
    #[serde(deserialize_with =
    "parse_file_type_of_asset_presigned_uris_read")] pub file_type : Option <
    FileType > ,
} pub fn parse_total_count_of_asset_presigned_uris_read < 'de, D >
(deserializer : D) -> std :: result :: Result < Option < usize > , D :: Error
> where D : serde :: Deserializer < 'de > ,
{
    use serde :: Deserialize; let s : Option < String > = Option ::
    deserialize(deserializer) ? ; match s
    {
        Some(s) =>
        {
            s.parse :: < usize >
            ().map_err(serde :: de :: Error :: custom).map(Some)
        } None => Ok(None),
    }
} pub fn parse_file_type_of_asset_presigned_uris_read < 'de, D >
(deserializer : D) -> std :: result :: Result < Option < FileType > , D ::
Error > where D : serde :: Deserializer < 'de > ,
{
    use serde :: Deserialize; let s : Option < String > = Option ::
    deserialize(deserializer) ? ; match s
    {
        Some(s) =>
        {
            s.parse :: < FileType >
            ().map_err(serde :: de :: Error :: custom).map(Some)
        } None => Ok(None),
    }
} impl AssetPresignedUrisReadAction
{
    pub fn new() -> Self { Self :: default() } pub fn
    get_presigned_uris(mut self, total_count : usize, file_type : FileType,)
    -> Self
    {
        self.total_count = Some(total_count); self.file_type =
        Some(file_type); self.action =
        Some(AssetPresignedUrisReadActionType :: GetPresignedUris); self
    }
} #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub enum
AssetPresignedUrisReadActionType { GetPresignedUris, } impl
AssetPresignedUrisClient
{
    pub async fn
    get_presigned_uris(& self, total_count : usize, file_type : FileType,) ->
    crate :: Result < AssetPresignedUris >
    {
        let path = format! ("/v1/metadata",); let endpoint = format!
        ("{}{}", self.endpoint, path); let params =
        AssetPresignedUrisReadAction ::
        new().get_presigned_uris(total_count, file_type,); let query = format!
        ("{}?{}", endpoint, AssetPresignedUrisParam :: Read(params)); rest_api
        :: get(& query).await
    }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, PartialEq,
by_macros :: QueryDisplay)]
#[cfg_attr(feature = "server", derive(aide :: OperationIo))]
#[serde(tag = "param-type", rename_all = "kebab-case")] pub enum
AssetPresignedUrisParam
{ Query(AssetPresignedUrisQuery), Read(AssetPresignedUrisReadAction), }
#[cfg(feature = "server")] impl schemars :: JsonSchema for
AssetPresignedUrisParam
{
    fn schema_name() -> String { "AssetPresignedUrisParam".to_string() } fn
    json_schema(_gen : & mut schemars :: gen :: SchemaGenerator) -> schemars
    :: schema :: Schema
    {
        let mut schema_obj = schemars :: schema :: SchemaObject :: default();
        schema_obj.metadata =
        Some(Box ::
        new(schemars :: schema :: Metadata
        {
            title : Some("AssetPresignedUris Query Parameters".to_string()),
            .. Default :: default()
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
            Some(vec!
            [serde_json :: Value ::
            String("get_presigned_uris".to_string()),]), .. Default ::
            default()
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
        schema_obj.object().properties.insert("total_count".to_string(), usize
        :: json_schema(_gen));
        schema_obj.object().properties.insert("file_type".to_string(),
        FileType :: json_schema(_gen));
        schema_obj.object().required.insert("size".to_string()); schemars ::
        schema :: Schema :: Object(schema_obj)
    }
} #[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(tag = "param_type")] #[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server",
derive(schemars :: JsonSchema, aide :: OperationIo))] pub enum
AssetPresignedUrisGetResponse
{
    Query(by_types::QueryResponse<AssetPresignedUris>),
    Read(AssetPresignedUris),
}