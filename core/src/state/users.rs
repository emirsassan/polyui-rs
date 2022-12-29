const USER_DB_TREE: &[u8] = b"users";
/// The set of users stored in the launcher
#[derive(Clone)]
pub(crate) struct Users {

}

impl Users {
    pub fn init(db: crate::prisma::PrismaClient) -> crate::error::Result<Self> {
        Ok(Users { /* fields */ })
    }
}