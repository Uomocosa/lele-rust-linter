use derive_more::Deref;

#[derive(Resource, Deref)]
pub struct PeerNames(pub Vec<String>);
