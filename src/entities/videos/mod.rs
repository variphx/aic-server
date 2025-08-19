#[derive(Debug, Clone, diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = crate::schema::videos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VideoEntity {
    id: i64,
    l: i16,
    v: i16,
}

impl VideoEntity {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn l(&self) -> i16 {
        self.l
    }

    pub fn v(&self) -> i16 {
        self.v
    }
}
