use crate::tokenizer::Location;

pub struct GQLError {
    pub message: String,
    pub location: Location,
}

pub fn report_gql_error(error: GQLError) {
    println!(
        "Error({}:{}) -> {}",
        error.location.start, error.location.end, error.message
    );
}
