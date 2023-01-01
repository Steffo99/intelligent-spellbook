use axum::http::StatusCode;
use axum::extract::Query;


pub struct GeneratePostQuery {
    pub name: String
}


pub async fn post(
    Query(GeneratePostQuery {name}): Query<GeneratePostQuery>,
) {
    log::info!("Generating spell: {}", &name);
    todo!();

    /*
    log::trace!("Checking for user authentication...");
    log::trace!("Checking if user has enough credit...");
    log::trace!("Requesting generation from OpenAI API...");
    log::debug!("Spell generation returned: {}", &text);
    log::trace!("Responding to the API request...");
     */
}
