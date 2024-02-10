use crate::data::DbId;
use crate::domain::clip;
use crate::{ClipError, ShortCode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;

#[derive(Debug, sqlx::FromRow)]
 struct Clip {
     clip_id: String,
    /// The code used to access this clip from the service.
     shortcode: String,
    /// The content of the Clip.
     content: String,
    /// The title of the Clip.
     title: Option<String>,
    /// The date that this Clip was posted to the service.
     posted: NaiveDateTime,
    /// The date that this Clip will expire.
     expires: Option<NaiveDateTime>,
    /// The password needed to view this Clip.
     password: Option<String>,
    /// The number of hits received by this Clip.
     hits: i64,
}

impl TryFrom<Clip> for crate::domain::clip {
    type Error = ClipError;
    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        Ok(
            Self{
                    clip_id: field::ClipId::new(DbId::try_from(clip.clip_id.as_str())?),
                    shortcode: field::,
                    content: String,
                    title: Option<String>,
                    posted: NaiveDateTime,
                    expires: Option<NaiveDateTime>,
                    password: Option<String>,
                    hits: i64,
            }
        )
    }
}

