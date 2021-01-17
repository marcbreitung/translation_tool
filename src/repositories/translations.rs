use diesel::prelude::*;
use uuid::Uuid;

use crate::models::translations::Translation;

pub fn find_translations(conn: &PgConnection) -> Result<Vec<Translation>, diesel::result::Error> {
    use crate::schema::translations::dsl::*;

    let result = translations.load::<Translation>(conn)?;

    Ok(result)
}

pub fn find_translation_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<Translation>, diesel::result::Error> {
    use crate::schema::translations::dsl::*;

    let translation = translations
        .filter(id.eq(uid.to_string()))
        .first::<Translation>(conn)
        .optional()?;

    Ok(translation)
}

pub fn update_translation(
    uid: Uuid,
    update_key: &str,
    update_target: &str,
    update_language: &str,
    conn: &PgConnection,
) -> Result<Translation, diesel::result::Error> {
    use crate::schema::translations;
    use crate::schema::translations::dsl::*;

    let translation = Translation::new(
        uid.to_string(),
        update_key.to_owned(),
        update_target.to_owned(),
        update_language.to_owned(),
    );

    diesel::update(translations::table)
        .set(&translation)
        .filter(id.eq(uid.to_string()))
        .execute(conn)?;

    Ok(translation)
}
pub fn delete_translation(uid: Uuid, conn: &PgConnection) -> Result<usize, diesel::result::Error> {
    use crate::schema::translations;
    use crate::schema::translations::dsl::*;

    let deleted_translation = diesel::delete(translations::table)
        .filter(id.eq(uid.to_string()))
        .execute(conn)?;

    Ok(deleted_translation)
}

pub fn add_translation(
    key: &str,
    target: &str,
    language: &str,
    conn: &PgConnection,
) -> Result<Translation, diesel::result::Error> {
    use crate::schema::translations;

    let translation = Translation::new(
        Uuid::new_v4().to_string(),
        key.to_owned(),
        target.to_owned(),
        language.to_owned(),
    );

    diesel::insert_into(translations::table)
        .values(&translation)
        .execute(conn)?;

    Ok(translation)
}
