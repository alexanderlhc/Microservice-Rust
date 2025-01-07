use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(schemas()),
    servers((url = "", description = "Local server")),
)]
pub struct ApiDoc;
