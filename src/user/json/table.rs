use sea_query::Iden;

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Name,
    Password,
    Role,
    CreatedAt,
    UpdatedAt,
}
