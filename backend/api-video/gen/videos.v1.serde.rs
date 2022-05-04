// @generated
impl serde::Serialize for CreateVideoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.visibility != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("videos.v1.CreateVideoRequest", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.visibility != 0 {
            let v = Visibility::from_i32(self.visibility)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.visibility)))?;
            struct_ser.serialize_field("visibility", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateVideoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "visibility",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Visibility,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "visibility" => Ok(GeneratedField::Visibility),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateVideoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct videos.v1.CreateVideoRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateVideoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title = None;
                let mut description = None;
                let mut visibility = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                        GeneratedField::Visibility => {
                            if visibility.is_some() {
                                return Err(serde::de::Error::duplicate_field("visibility"));
                            }
                            visibility = Some(map.next_value::<Visibility>()? as i32);
                        }
                    }
                }
                Ok(CreateVideoRequest {
                    title: title.unwrap_or_default(),
                    description: description.unwrap_or_default(),
                    visibility: visibility.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("videos.v1.CreateVideoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateVideoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("videos.v1.CreateVideoResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateVideoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateVideoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct videos.v1.CreateVideoResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateVideoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CreateVideoResponse {
                    id: id.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("videos.v1.CreateVideoResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetVideoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("videos.v1.GetVideoRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetVideoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetVideoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct videos.v1.GetVideoRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetVideoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetVideoRequest {
                    id: id.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("videos.v1.GetVideoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetVideoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.video.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("videos.v1.GetVideoResponse", len)?;
        if let Some(v) = self.video.as_ref() {
            struct_ser.serialize_field("video", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetVideoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "video",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Video,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "video" => Ok(GeneratedField::Video),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetVideoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct videos.v1.GetVideoResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetVideoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut video = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Video => {
                            if video.is_some() {
                                return Err(serde::de::Error::duplicate_field("video"));
                            }
                            video = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetVideoResponse {
                    video,
                })
            }
        }
        deserializer.deserialize_struct("videos.v1.GetVideoResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STATUS_UNSPECIFIED",
            Self::Finished => "STATUS_FINISHED",
            Self::Processing => "STATUS_PROCESSING",
            Self::Draft => "STATUS_DRAFT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATUS_UNSPECIFIED",
            "STATUS_FINISHED",
            "STATUS_PROCESSING",
            "STATUS_DRAFT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Status;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(Status::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(Status::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STATUS_UNSPECIFIED" => Ok(Status::Unspecified),
                    "STATUS_FINISHED" => Ok(Status::Finished),
                    "STATUS_PROCESSING" => Ok(Status::Processing),
                    "STATUS_DRAFT" => Ok(Status::Draft),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateVideoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.video.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("videos.v1.UpdateVideoRequest", len)?;
        if let Some(v) = self.video.as_ref() {
            struct_ser.serialize_field("video", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateVideoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "video",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Video,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "video" => Ok(GeneratedField::Video),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateVideoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct videos.v1.UpdateVideoRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateVideoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut video = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Video => {
                            if video.is_some() {
                                return Err(serde::de::Error::duplicate_field("video"));
                            }
                            video = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateVideoRequest {
                    video,
                })
            }
        }
        deserializer.deserialize_struct("videos.v1.UpdateVideoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateVideoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.video.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("videos.v1.UpdateVideoResponse", len)?;
        if let Some(v) = self.video.as_ref() {
            struct_ser.serialize_field("video", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateVideoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "video",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Video,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "video" => Ok(GeneratedField::Video),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateVideoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct videos.v1.UpdateVideoResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateVideoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut video = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Video => {
                            if video.is_some() {
                                return Err(serde::de::Error::duplicate_field("video"));
                            }
                            video = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateVideoResponse {
                    video,
                })
            }
        }
        deserializer.deserialize_struct("videos.v1.UpdateVideoResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Video {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.author.is_empty() {
            len += 1;
        }
        if !self.date.is_empty() {
            len += 1;
        }
        if self.visibility != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("videos.v1.Video", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.author.is_empty() {
            struct_ser.serialize_field("author", &self.author)?;
        }
        if !self.date.is_empty() {
            struct_ser.serialize_field("date", &self.date)?;
        }
        if self.visibility != 0 {
            let v = Visibility::from_i32(self.visibility)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.visibility)))?;
            struct_ser.serialize_field("visibility", &v)?;
        }
        if self.status != 0 {
            let v = Status::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Video {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "title",
            "description",
            "author",
            "date",
            "visibility",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Title,
            Description,
            Author,
            Date,
            Visibility,
            Status,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "author" => Ok(GeneratedField::Author),
                            "date" => Ok(GeneratedField::Date),
                            "visibility" => Ok(GeneratedField::Visibility),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Video;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct videos.v1.Video")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Video, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut title = None;
                let mut description = None;
                let mut author = None;
                let mut date = None;
                let mut visibility = None;
                let mut status = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                        GeneratedField::Author => {
                            if author.is_some() {
                                return Err(serde::de::Error::duplicate_field("author"));
                            }
                            author = Some(map.next_value()?);
                        }
                        GeneratedField::Date => {
                            if date.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            date = Some(map.next_value()?);
                        }
                        GeneratedField::Visibility => {
                            if visibility.is_some() {
                                return Err(serde::de::Error::duplicate_field("visibility"));
                            }
                            visibility = Some(map.next_value::<Visibility>()? as i32);
                        }
                        GeneratedField::Status => {
                            if status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status = Some(map.next_value::<Status>()? as i32);
                        }
                    }
                }
                Ok(Video {
                    id: id.unwrap_or_default(),
                    title: title.unwrap_or_default(),
                    description: description.unwrap_or_default(),
                    author: author.unwrap_or_default(),
                    date: date.unwrap_or_default(),
                    visibility: visibility.unwrap_or_default(),
                    status: status.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("videos.v1.Video", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Visibility {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VISIBILITY_UNSPECIFIED",
            Self::Public => "VISIBILITY_PUBLIC",
            Self::Private => "VISIBILITY_PRIVATE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Visibility {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VISIBILITY_UNSPECIFIED",
            "VISIBILITY_PUBLIC",
            "VISIBILITY_PRIVATE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Visibility;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(Visibility::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(Visibility::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "VISIBILITY_UNSPECIFIED" => Ok(Visibility::Unspecified),
                    "VISIBILITY_PUBLIC" => Ok(Visibility::Public),
                    "VISIBILITY_PRIVATE" => Ok(Visibility::Private),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
