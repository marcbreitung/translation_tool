use diesel::prelude::*;
use uuid::Uuid;

use crate::models::languages::Language;

pub fn find_languages(conn: &PgConnection) -> Result<Vec<Language>, diesel::result::Error> {
    use crate::schema::languages::dsl::*;

    let result = languages.load::<Language>(conn)?;

    Ok(result)
}

pub fn find_language_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<Language>, diesel::result::Error> {
    use crate::schema::languages::dsl::*;

    let language = languages
        .filter(id.eq(uid.to_string()))
        .first::<Language>(conn)
        .optional()?;

    Ok(language)
}

pub fn update_language(
    uid: Uuid,
    update_name: &str,
    update_lang: &str,
    update_territory: &str,
    conn: &PgConnection,
) -> Result<Language, diesel::result::Error> {
    use crate::schema::languages;
    use crate::schema::languages::dsl::*;

    let language = Language::new(
        uid.to_string(),
        update_name.to_owned(),
        update_lang.to_owned(),
        update_territory.to_owned(),
    );

    diesel::update(languages::table)
        .set(&language)
        .filter(id.eq(uid.to_string()))
        .execute(conn)?;

    Ok(language)
}
pub fn delete_language(uid: Uuid, conn: &PgConnection) -> Result<usize, diesel::result::Error> {
    use crate::schema::languages;
    use crate::schema::languages::dsl::*;

    let deleted_language = diesel::delete(languages::table)
        .filter(id.eq(uid.to_string()))
        .execute(conn)?;

    Ok(deleted_language)
}

pub fn add_language(
    name: &str,
    lang: &str,
    terrirory: &str,
    conn: &PgConnection,
) -> Result<Language, diesel::result::Error> {
    use crate::schema::languages;

    let language = Language::new(
        Uuid::new_v4().to_string(),
        name.to_owned(),
        lang.to_owned(),
        terrirory.to_owned(),
    );

    diesel::insert_into(languages::table)
        .values(&language)
        .execute(conn)?;

    Ok(language)
}
