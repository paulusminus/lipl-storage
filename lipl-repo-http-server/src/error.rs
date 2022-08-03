use warp::reject::Reject;

#[derive(Debug)]
pub struct PostError;

impl Reject for PostError {}
