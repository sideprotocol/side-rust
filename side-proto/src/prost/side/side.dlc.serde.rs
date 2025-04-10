// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for Dcm {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.desc.is_empty() {
            len += 1;
        }
        if !self.participants.is_empty() {
            len += 1;
        }
        if self.threshold != 0 {
            len += 1;
        }
        if !self.pubkey.is_empty() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.DCM", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        if !self.desc.is_empty() {
            struct_ser.serialize_field("desc", &self.desc)?;
        }
        if !self.participants.is_empty() {
            struct_ser.serialize_field("participants", &self.participants)?;
        }
        if self.threshold != 0 {
            struct_ser.serialize_field("threshold", &self.threshold)?;
        }
        if !self.pubkey.is_empty() {
            struct_ser.serialize_field("pubkey", &self.pubkey)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if self.status != 0 {
            let v = DcmStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Dcm {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "desc",
            "participants",
            "threshold",
            "pubkey",
            "time",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Desc,
            Participants,
            Threshold,
            Pubkey,
            Time,
            Status,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "desc" => Ok(GeneratedField::Desc),
                            "participants" => Ok(GeneratedField::Participants),
                            "threshold" => Ok(GeneratedField::Threshold),
                            "pubkey" => Ok(GeneratedField::Pubkey),
                            "time" => Ok(GeneratedField::Time),
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
            type Value = Dcm;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.DCM")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Dcm, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut desc__ = None;
                let mut participants__ = None;
                let mut threshold__ = None;
                let mut pubkey__ = None;
                let mut time__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Desc => {
                            if desc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("desc"));
                            }
                            desc__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Participants => {
                            if participants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("participants"));
                            }
                            participants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Threshold => {
                            if threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threshold"));
                            }
                            threshold__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Pubkey => {
                            if pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubkey"));
                            }
                            pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<DcmStatus>()? as i32);
                        }
                    }
                }
                Ok(Dcm {
                    id: id__.unwrap_or_default(),
                    desc: desc__.unwrap_or_default(),
                    participants: participants__.unwrap_or_default(),
                    threshold: threshold__.unwrap_or_default(),
                    pubkey: pubkey__.unwrap_or_default(),
                    time: time__,
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.DCM", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DcmStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Pending => "DCM_Status_Pending",
            Self::Failed => "DCM_Status_Failed",
            Self::Timedout => "DCM_Status_Timedout",
            Self::Enable => "DCM_status_Enable",
            Self::Disable => "DCM_status_Disable",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DcmStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DCM_Status_Pending",
            "DCM_Status_Failed",
            "DCM_Status_Timedout",
            "DCM_status_Enable",
            "DCM_status_Disable",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DcmStatus;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DCM_Status_Pending" => Ok(DcmStatus::Pending),
                    "DCM_Status_Failed" => Ok(DcmStatus::Failed),
                    "DCM_Status_Timedout" => Ok(DcmStatus::Timedout),
                    "DCM_status_Enable" => Ok(DcmStatus::Enable),
                    "DCM_status_Disable" => Ok(DcmStatus::Disable),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DlcAttestation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.event_id != 0 {
            len += 1;
        }
        if !self.outcome.is_empty() {
            len += 1;
        }
        if !self.pubkey.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.DLCAttestation", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        if self.event_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventId",
                alloc::string::ToString::to_string(&self.event_id).as_str(),
            )?;
        }
        if !self.outcome.is_empty() {
            struct_ser.serialize_field("outcome", &self.outcome)?;
        }
        if !self.pubkey.is_empty() {
            struct_ser.serialize_field("pubkey", &self.pubkey)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DlcAttestation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "event_id",
            "eventId",
            "outcome",
            "pubkey",
            "signature",
            "time",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            EventId,
            Outcome,
            Pubkey,
            Signature,
            Time,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            "outcome" => Ok(GeneratedField::Outcome),
                            "pubkey" => Ok(GeneratedField::Pubkey),
                            "signature" => Ok(GeneratedField::Signature),
                            "time" => Ok(GeneratedField::Time),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DlcAttestation;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.DLCAttestation")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<DlcAttestation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut event_id__ = None;
                let mut outcome__ = None;
                let mut pubkey__ = None;
                let mut signature__ = None;
                let mut time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::EventId => {
                            if event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventId"));
                            }
                            event_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Outcome => {
                            if outcome__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcome"));
                            }
                            outcome__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pubkey => {
                            if pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubkey"));
                            }
                            pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DlcAttestation {
                    id: id__.unwrap_or_default(),
                    event_id: event_id__.unwrap_or_default(),
                    outcome: outcome__.unwrap_or_default(),
                    pubkey: pubkey__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                    time: time__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.DLCAttestation", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DlcEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        if !self.pubkey.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.outcomes.is_empty() {
            len += 1;
        }
        if self.has_triggered {
            len += 1;
        }
        if self.outcome_index != 0 {
            len += 1;
        }
        if self.publish_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.DLCEvent", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        if self.r#type != 0 {
            let v = DlcEventType::try_from(self.r#type).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.r#type))
            })?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.nonce.is_empty() {
            struct_ser.serialize_field("nonce", &self.nonce)?;
        }
        if !self.pubkey.is_empty() {
            struct_ser.serialize_field("pubkey", &self.pubkey)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.outcomes.is_empty() {
            struct_ser.serialize_field("outcomes", &self.outcomes)?;
        }
        if self.has_triggered {
            struct_ser.serialize_field("hasTriggered", &self.has_triggered)?;
        }
        if self.outcome_index != 0 {
            struct_ser.serialize_field("outcomeIndex", &self.outcome_index)?;
        }
        if let Some(v) = self.publish_at.as_ref() {
            struct_ser.serialize_field("publishAt", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DlcEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "type",
            "nonce",
            "pubkey",
            "description",
            "outcomes",
            "has_triggered",
            "hasTriggered",
            "outcome_index",
            "outcomeIndex",
            "publish_at",
            "publishAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Type,
            Nonce,
            Pubkey,
            Description,
            Outcomes,
            HasTriggered,
            OutcomeIndex,
            PublishAt,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "type" => Ok(GeneratedField::Type),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "pubkey" => Ok(GeneratedField::Pubkey),
                            "description" => Ok(GeneratedField::Description),
                            "outcomes" => Ok(GeneratedField::Outcomes),
                            "hasTriggered" | "has_triggered" => Ok(GeneratedField::HasTriggered),
                            "outcomeIndex" | "outcome_index" => Ok(GeneratedField::OutcomeIndex),
                            "publishAt" | "publish_at" => Ok(GeneratedField::PublishAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DlcEvent;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.DLCEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<DlcEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut r#type__ = None;
                let mut nonce__ = None;
                let mut pubkey__ = None;
                let mut description__ = None;
                let mut outcomes__ = None;
                let mut has_triggered__ = None;
                let mut outcome_index__ = None;
                let mut publish_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<DlcEventType>()? as i32);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pubkey => {
                            if pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubkey"));
                            }
                            pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Outcomes => {
                            if outcomes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcomes"));
                            }
                            outcomes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HasTriggered => {
                            if has_triggered__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasTriggered"));
                            }
                            has_triggered__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutcomeIndex => {
                            if outcome_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcomeIndex"));
                            }
                            outcome_index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PublishAt => {
                            if publish_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publishAt"));
                            }
                            publish_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DlcEvent {
                    id: id__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    pubkey: pubkey__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    outcomes: outcomes__.unwrap_or_default(),
                    has_triggered: has_triggered__.unwrap_or_default(),
                    outcome_index: outcome_index__.unwrap_or_default(),
                    publish_at: publish_at__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.DLCEvent", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DlcNonce {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.index != 0 {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        if !self.oracle_pubkey.is_empty() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.DLCNonce", len)?;
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "index",
                alloc::string::ToString::to_string(&self.index).as_str(),
            )?;
        }
        if !self.nonce.is_empty() {
            struct_ser.serialize_field("nonce", &self.nonce)?;
        }
        if !self.oracle_pubkey.is_empty() {
            struct_ser.serialize_field("oraclePubkey", &self.oracle_pubkey)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DlcNonce {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["index", "nonce", "oracle_pubkey", "oraclePubkey", "time"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Index,
            Nonce,
            OraclePubkey,
            Time,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "index" => Ok(GeneratedField::Index),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "oraclePubkey" | "oracle_pubkey" => Ok(GeneratedField::OraclePubkey),
                            "time" => Ok(GeneratedField::Time),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DlcNonce;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.DLCNonce")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<DlcNonce, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut index__ = None;
                let mut nonce__ = None;
                let mut oracle_pubkey__ = None;
                let mut time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OraclePubkey => {
                            if oracle_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oraclePubkey"));
                            }
                            oracle_pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DlcNonce {
                    index: index__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    oracle_pubkey: oracle_pubkey__.unwrap_or_default(),
                    time: time__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.DLCNonce", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DlcOracle {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.desc.is_empty() {
            len += 1;
        }
        if !self.participants.is_empty() {
            len += 1;
        }
        if self.threshold != 0 {
            len += 1;
        }
        if !self.pubkey.is_empty() {
            len += 1;
        }
        if self.nonce_index != 0 {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.DLCOracle", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        if !self.desc.is_empty() {
            struct_ser.serialize_field("desc", &self.desc)?;
        }
        if !self.participants.is_empty() {
            struct_ser.serialize_field("participants", &self.participants)?;
        }
        if self.threshold != 0 {
            struct_ser.serialize_field("threshold", &self.threshold)?;
        }
        if !self.pubkey.is_empty() {
            struct_ser.serialize_field("pubkey", &self.pubkey)?;
        }
        if self.nonce_index != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "nonceIndex",
                alloc::string::ToString::to_string(&self.nonce_index).as_str(),
            )?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if self.status != 0 {
            let v = DlcOracleStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DlcOracle {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "desc",
            "participants",
            "threshold",
            "pubkey",
            "nonce_index",
            "nonceIndex",
            "time",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Desc,
            Participants,
            Threshold,
            Pubkey,
            NonceIndex,
            Time,
            Status,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "desc" => Ok(GeneratedField::Desc),
                            "participants" => Ok(GeneratedField::Participants),
                            "threshold" => Ok(GeneratedField::Threshold),
                            "pubkey" => Ok(GeneratedField::Pubkey),
                            "nonceIndex" | "nonce_index" => Ok(GeneratedField::NonceIndex),
                            "time" => Ok(GeneratedField::Time),
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
            type Value = DlcOracle;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.DLCOracle")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<DlcOracle, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut desc__ = None;
                let mut participants__ = None;
                let mut threshold__ = None;
                let mut pubkey__ = None;
                let mut nonce_index__ = None;
                let mut time__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Desc => {
                            if desc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("desc"));
                            }
                            desc__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Participants => {
                            if participants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("participants"));
                            }
                            participants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Threshold => {
                            if threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threshold"));
                            }
                            threshold__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Pubkey => {
                            if pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubkey"));
                            }
                            pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NonceIndex => {
                            if nonce_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonceIndex"));
                            }
                            nonce_index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<DlcOracleStatus>()? as i32);
                        }
                    }
                }
                Ok(DlcOracle {
                    id: id__.unwrap_or_default(),
                    desc: desc__.unwrap_or_default(),
                    participants: participants__.unwrap_or_default(),
                    threshold: threshold__.unwrap_or_default(),
                    pubkey: pubkey__.unwrap_or_default(),
                    nonce_index: nonce_index__.unwrap_or_default(),
                    time: time__,
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.DLCOracle", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DlcOracleStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::OracleStatusPending => "Oracle_Status_Pending",
            Self::OracleStatusFailed => "Oracle_Status_Failed",
            Self::OracleStatusTimedout => "Oracle_Status_Timedout",
            Self::OracleStatusEnable => "Oracle_status_Enable",
            Self::OracleStatusDisable => "Oracle_status_Disable",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DlcOracleStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Oracle_Status_Pending",
            "Oracle_Status_Failed",
            "Oracle_Status_Timedout",
            "Oracle_status_Enable",
            "Oracle_status_Disable",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DlcOracleStatus;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "Oracle_Status_Pending" => Ok(DlcOracleStatus::OracleStatusPending),
                    "Oracle_Status_Failed" => Ok(DlcOracleStatus::OracleStatusFailed),
                    "Oracle_Status_Timedout" => Ok(DlcOracleStatus::OracleStatusTimedout),
                    "Oracle_status_Enable" => Ok(DlcOracleStatus::OracleStatusEnable),
                    "Oracle_status_Disable" => Ok(DlcOracleStatus::OracleStatusDisable),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DlcEventType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Price => "PRICE",
            Self::Date => "DATE",
            Self::Lending => "LENDING",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DlcEventType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["UNSPECIFIED", "PRICE", "DATE", "LENDING"];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DlcEventType;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSPECIFIED" => Ok(DlcEventType::Unspecified),
                    "PRICE" => Ok(DlcEventType::Price),
                    "DATE" => Ok(DlcEventType::Date),
                    "LENDING" => Ok(DlcEventType::Lending),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for GenesisState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        if !self.attestations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        if !self.attestations.is_empty() {
            struct_ser.serialize_field("attestations", &self.attestations)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GenesisState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["params", "events", "attestations"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            Events,
            Attestations,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "params" => Ok(GeneratedField::Params),
                            "events" => Ok(GeneratedField::Events),
                            "attestations" => Ok(GeneratedField::Attestations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenesisState;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut events__ = None;
                let mut attestations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Attestations => {
                            if attestations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestations"));
                            }
                            attestations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    events: events__.unwrap_or_default(),
                    attestations: attestations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.GenesisState", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCreateDcm {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.participants.is_empty() {
            len += 1;
        }
        if self.threshold != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.MsgCreateDCM", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.participants.is_empty() {
            struct_ser.serialize_field("participants", &self.participants)?;
        }
        if self.threshold != 0 {
            struct_ser.serialize_field("threshold", &self.threshold)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCreateDcm {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "participants", "threshold"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Participants,
            Threshold,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authority" => Ok(GeneratedField::Authority),
                            "participants" => Ok(GeneratedField::Participants),
                            "threshold" => Ok(GeneratedField::Threshold),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateDcm;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgCreateDCM")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgCreateDcm, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut participants__ = None;
                let mut threshold__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Participants => {
                            if participants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("participants"));
                            }
                            participants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Threshold => {
                            if threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threshold"));
                            }
                            threshold__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgCreateDcm {
                    authority: authority__.unwrap_or_default(),
                    participants: participants__.unwrap_or_default(),
                    threshold: threshold__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.MsgCreateDCM", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCreateDcmResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.dlc.MsgCreateDCMResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCreateDcmResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateDcmResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgCreateDCMResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgCreateDcmResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCreateDcmResponse {})
            }
        }
        deserializer.deserialize_struct("side.dlc.MsgCreateDCMResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCreateOracle {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.participants.is_empty() {
            len += 1;
        }
        if self.threshold != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.MsgCreateOracle", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.participants.is_empty() {
            struct_ser.serialize_field("participants", &self.participants)?;
        }
        if self.threshold != 0 {
            struct_ser.serialize_field("threshold", &self.threshold)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCreateOracle {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "participants", "threshold"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Participants,
            Threshold,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authority" => Ok(GeneratedField::Authority),
                            "participants" => Ok(GeneratedField::Participants),
                            "threshold" => Ok(GeneratedField::Threshold),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateOracle;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgCreateOracle")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgCreateOracle, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut participants__ = None;
                let mut threshold__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Participants => {
                            if participants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("participants"));
                            }
                            participants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Threshold => {
                            if threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threshold"));
                            }
                            threshold__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(MsgCreateOracle {
                    authority: authority__.unwrap_or_default(),
                    participants: participants__.unwrap_or_default(),
                    threshold: threshold__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.MsgCreateOracle", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCreateOracleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.dlc.MsgCreateOracleResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCreateOracleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateOracleResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgCreateOracleResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgCreateOracleResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCreateOracleResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.MsgCreateOracleResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitAttestation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if self.event_id != 0 {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.MsgSubmitAttestation", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if self.event_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventId",
                alloc::string::ToString::to_string(&self.event_id).as_str(),
            )?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitAttestation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "event_id", "eventId", "signature"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            EventId,
            Signature,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitAttestation;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgSubmitAttestation")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitAttestation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut event_id__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventId => {
                            if event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventId"));
                            }
                            event_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitAttestation {
                    sender: sender__.unwrap_or_default(),
                    event_id: event_id__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.MsgSubmitAttestation", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitAttestationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("side.dlc.MsgSubmitAttestationResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitAttestationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitAttestationResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgSubmitAttestationResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitAttestationResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitAttestationResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.MsgSubmitAttestationResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitDcmPubKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.pub_key.is_empty() {
            len += 1;
        }
        if self.dcm_id != 0 {
            len += 1;
        }
        if !self.dcm_pubkey.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.MsgSubmitDCMPubKey", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.pub_key.is_empty() {
            struct_ser.serialize_field("pubKey", &self.pub_key)?;
        }
        if self.dcm_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "dcmId",
                alloc::string::ToString::to_string(&self.dcm_id).as_str(),
            )?;
        }
        if !self.dcm_pubkey.is_empty() {
            struct_ser.serialize_field("dcmPubkey", &self.dcm_pubkey)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitDcmPubKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "pub_key",
            "pubKey",
            "dcm_id",
            "dcmId",
            "dcm_pubkey",
            "dcmPubkey",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            PubKey,
            DcmId,
            DcmPubkey,
            Signature,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "pubKey" | "pub_key" => Ok(GeneratedField::PubKey),
                            "dcmId" | "dcm_id" => Ok(GeneratedField::DcmId),
                            "dcmPubkey" | "dcm_pubkey" => Ok(GeneratedField::DcmPubkey),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitDcmPubKey;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgSubmitDCMPubKey")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitDcmPubKey, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut pub_key__ = None;
                let mut dcm_id__ = None;
                let mut dcm_pubkey__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PubKey => {
                            if pub_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubKey"));
                            }
                            pub_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DcmId => {
                            if dcm_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dcmId"));
                            }
                            dcm_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DcmPubkey => {
                            if dcm_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dcmPubkey"));
                            }
                            dcm_pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitDcmPubKey {
                    sender: sender__.unwrap_or_default(),
                    pub_key: pub_key__.unwrap_or_default(),
                    dcm_id: dcm_id__.unwrap_or_default(),
                    dcm_pubkey: dcm_pubkey__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.MsgSubmitDCMPubKey", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitDcmPubKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.dlc.MsgSubmitDCMPubKeyResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitDcmPubKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitDcmPubKeyResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgSubmitDCMPubKeyResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitDcmPubKeyResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitDcmPubKeyResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.MsgSubmitDCMPubKeyResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitNonce {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if self.event_type != 0 {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        if !self.oracle_pubkey.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.MsgSubmitNonce", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if self.event_type != 0 {
            let v = DlcEventType::try_from(self.event_type).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.event_type))
            })?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if !self.nonce.is_empty() {
            struct_ser.serialize_field("nonce", &self.nonce)?;
        }
        if !self.oracle_pubkey.is_empty() {
            struct_ser.serialize_field("oraclePubkey", &self.oracle_pubkey)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitNonce {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "event_type",
            "eventType",
            "nonce",
            "oracle_pubkey",
            "oraclePubkey",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            EventType,
            Nonce,
            OraclePubkey,
            Signature,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "oraclePubkey" | "oracle_pubkey" => Ok(GeneratedField::OraclePubkey),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitNonce;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgSubmitNonce")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitNonce, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut event_type__ = None;
                let mut nonce__ = None;
                let mut oracle_pubkey__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = Some(map_.next_value::<DlcEventType>()? as i32);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OraclePubkey => {
                            if oracle_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oraclePubkey"));
                            }
                            oracle_pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitNonce {
                    sender: sender__.unwrap_or_default(),
                    event_type: event_type__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    oracle_pubkey: oracle_pubkey__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.MsgSubmitNonce", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitNonceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.dlc.MsgSubmitNonceResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitNonceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitNonceResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgSubmitNonceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitNonceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitNonceResponse {})
            }
        }
        deserializer.deserialize_struct("side.dlc.MsgSubmitNonceResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitOraclePubKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.pub_key.is_empty() {
            len += 1;
        }
        if self.oracle_id != 0 {
            len += 1;
        }
        if !self.oracle_pubkey.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.MsgSubmitOraclePubKey", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.pub_key.is_empty() {
            struct_ser.serialize_field("pubKey", &self.pub_key)?;
        }
        if self.oracle_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "oracleId",
                alloc::string::ToString::to_string(&self.oracle_id).as_str(),
            )?;
        }
        if !self.oracle_pubkey.is_empty() {
            struct_ser.serialize_field("oraclePubkey", &self.oracle_pubkey)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitOraclePubKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "pub_key",
            "pubKey",
            "oracle_id",
            "oracleId",
            "oracle_pubkey",
            "oraclePubkey",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            PubKey,
            OracleId,
            OraclePubkey,
            Signature,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "pubKey" | "pub_key" => Ok(GeneratedField::PubKey),
                            "oracleId" | "oracle_id" => Ok(GeneratedField::OracleId),
                            "oraclePubkey" | "oracle_pubkey" => Ok(GeneratedField::OraclePubkey),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitOraclePubKey;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgSubmitOraclePubKey")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitOraclePubKey, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut pub_key__ = None;
                let mut oracle_id__ = None;
                let mut oracle_pubkey__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PubKey => {
                            if pub_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubKey"));
                            }
                            pub_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OracleId => {
                            if oracle_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleId"));
                            }
                            oracle_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::OraclePubkey => {
                            if oracle_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oraclePubkey"));
                            }
                            oracle_pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitOraclePubKey {
                    sender: sender__.unwrap_or_default(),
                    pub_key: pub_key__.unwrap_or_default(),
                    oracle_id: oracle_id__.unwrap_or_default(),
                    oracle_pubkey: oracle_pubkey__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.MsgSubmitOraclePubKey", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitOraclePubKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("side.dlc.MsgSubmitOraclePubKeyResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitOraclePubKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitOraclePubKeyResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgSubmitOraclePubKeyResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitOraclePubKeyResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitOraclePubKeyResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.MsgSubmitOraclePubKeyResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.MsgUpdateParams", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "params"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Params,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authority" => Ok(GeneratedField::Authority),
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParams;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgUpdateParams")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgUpdateParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateParams {
                    authority: authority__.unwrap_or_default(),
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.MsgUpdateParams", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.dlc.MsgUpdateParamsResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParamsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgUpdateParamsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgUpdateParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateParamsResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.MsgUpdateParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Params {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.price_event_nonce_queue_size != 0 {
            len += 1;
        }
        if !self.price_intervals.is_empty() {
            len += 1;
        }
        if self.date_event_nonce_queue_size != 0 {
            len += 1;
        }
        if self.date_interval.is_some() {
            len += 1;
        }
        if self.lending_event_nonce_queue_size != 0 {
            len += 1;
        }
        if self.dkg_timeout_period.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.Params", len)?;
        if self.price_event_nonce_queue_size != 0 {
            struct_ser.serialize_field(
                "priceEventNonceQueueSize",
                &self.price_event_nonce_queue_size,
            )?;
        }
        if !self.price_intervals.is_empty() {
            struct_ser.serialize_field("priceIntervals", &self.price_intervals)?;
        }
        if self.date_event_nonce_queue_size != 0 {
            struct_ser
                .serialize_field("dateEventNonceQueueSize", &self.date_event_nonce_queue_size)?;
        }
        if let Some(v) = self.date_interval.as_ref() {
            struct_ser.serialize_field("dateInterval", v)?;
        }
        if self.lending_event_nonce_queue_size != 0 {
            struct_ser.serialize_field(
                "lendingEventNonceQueueSize",
                &self.lending_event_nonce_queue_size,
            )?;
        }
        if let Some(v) = self.dkg_timeout_period.as_ref() {
            struct_ser.serialize_field("dkgTimeoutPeriod", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price_event_nonce_queue_size",
            "priceEventNonceQueueSize",
            "price_intervals",
            "priceIntervals",
            "date_event_nonce_queue_size",
            "dateEventNonceQueueSize",
            "date_interval",
            "dateInterval",
            "lending_event_nonce_queue_size",
            "lendingEventNonceQueueSize",
            "dkg_timeout_period",
            "dkgTimeoutPeriod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceEventNonceQueueSize,
            PriceIntervals,
            DateEventNonceQueueSize,
            DateInterval,
            LendingEventNonceQueueSize,
            DkgTimeoutPeriod,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "priceEventNonceQueueSize" | "price_event_nonce_queue_size" => {
                                Ok(GeneratedField::PriceEventNonceQueueSize)
                            }
                            "priceIntervals" | "price_intervals" => {
                                Ok(GeneratedField::PriceIntervals)
                            }
                            "dateEventNonceQueueSize" | "date_event_nonce_queue_size" => {
                                Ok(GeneratedField::DateEventNonceQueueSize)
                            }
                            "dateInterval" | "date_interval" => Ok(GeneratedField::DateInterval),
                            "lendingEventNonceQueueSize" | "lending_event_nonce_queue_size" => {
                                Ok(GeneratedField::LendingEventNonceQueueSize)
                            }
                            "dkgTimeoutPeriod" | "dkg_timeout_period" => {
                                Ok(GeneratedField::DkgTimeoutPeriod)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Params;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_event_nonce_queue_size__ = None;
                let mut price_intervals__ = None;
                let mut date_event_nonce_queue_size__ = None;
                let mut date_interval__ = None;
                let mut lending_event_nonce_queue_size__ = None;
                let mut dkg_timeout_period__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceEventNonceQueueSize => {
                            if price_event_nonce_queue_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "priceEventNonceQueueSize",
                                ));
                            }
                            price_event_nonce_queue_size__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PriceIntervals => {
                            if price_intervals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceIntervals"));
                            }
                            price_intervals__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DateEventNonceQueueSize => {
                            if date_event_nonce_queue_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "dateEventNonceQueueSize",
                                ));
                            }
                            date_event_nonce_queue_size__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DateInterval => {
                            if date_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateInterval"));
                            }
                            date_interval__ = map_.next_value()?;
                        }
                        GeneratedField::LendingEventNonceQueueSize => {
                            if lending_event_nonce_queue_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "lendingEventNonceQueueSize",
                                ));
                            }
                            lending_event_nonce_queue_size__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DkgTimeoutPeriod => {
                            if dkg_timeout_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dkgTimeoutPeriod"));
                            }
                            dkg_timeout_period__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Params {
                    price_event_nonce_queue_size: price_event_nonce_queue_size__
                        .unwrap_or_default(),
                    price_intervals: price_intervals__.unwrap_or_default(),
                    date_event_nonce_queue_size: date_event_nonce_queue_size__.unwrap_or_default(),
                    date_interval: date_interval__,
                    lending_event_nonce_queue_size: lending_event_nonce_queue_size__
                        .unwrap_or_default(),
                    dkg_timeout_period: dkg_timeout_period__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.Params", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PriceInterval {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_pair.is_empty() {
            len += 1;
        }
        if self.interval != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.PriceInterval", len)?;
        if !self.price_pair.is_empty() {
            struct_ser.serialize_field("pricePair", &self.price_pair)?;
        }
        if self.interval != 0 {
            struct_ser.serialize_field("interval", &self.interval)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PriceInterval {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price_pair", "pricePair", "interval"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PricePair,
            Interval,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pricePair" | "price_pair" => Ok(GeneratedField::PricePair),
                            "interval" => Ok(GeneratedField::Interval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PriceInterval;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.PriceInterval")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<PriceInterval, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price_pair__ = None;
                let mut interval__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PricePair => {
                            if price_pair__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pricePair"));
                            }
                            price_pair__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(PriceInterval {
                    price_pair: price_pair__.unwrap_or_default(),
                    interval: interval__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.PriceInterval", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryAttestationByEventRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event_id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.dlc.QueryAttestationByEventRequest", len)?;
        if self.event_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventId",
                alloc::string::ToString::to_string(&self.event_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAttestationByEventRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["event_id", "eventId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventId,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAttestationByEventRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryAttestationByEventRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryAttestationByEventRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventId => {
                            if event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventId"));
                            }
                            event_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryAttestationByEventRequest {
                    event_id: event_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.QueryAttestationByEventRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryAttestationByEventResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attestation.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.dlc.QueryAttestationByEventResponse", len)?;
        if let Some(v) = self.attestation.as_ref() {
            struct_ser.serialize_field("attestation", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAttestationByEventResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["attestation"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attestation,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attestation" => Ok(GeneratedField::Attestation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAttestationByEventResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryAttestationByEventResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryAttestationByEventResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut attestation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attestation => {
                            if attestation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestation"));
                            }
                            attestation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAttestationByEventResponse {
                    attestation: attestation__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.QueryAttestationByEventResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryAttestationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.dlc.QueryAttestationRequest", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAttestationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
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
            type Value = QueryAttestationRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryAttestationRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryAttestationRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryAttestationRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.QueryAttestationRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryAttestationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attestation.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.dlc.QueryAttestationResponse", len)?;
        if let Some(v) = self.attestation.as_ref() {
            struct_ser.serialize_field("attestation", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAttestationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["attestation"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attestation,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attestation" => Ok(GeneratedField::Attestation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAttestationResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryAttestationResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryAttestationResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut attestation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attestation => {
                            if attestation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestation"));
                            }
                            attestation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAttestationResponse {
                    attestation: attestation__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.QueryAttestationResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryAttestationsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.dlc.QueryAttestationsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAttestationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAttestationsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryAttestationsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryAttestationsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAttestationsRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.QueryAttestationsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryAttestationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attestations.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.dlc.QueryAttestationsResponse", len)?;
        if !self.attestations.is_empty() {
            struct_ser.serialize_field("attestations", &self.attestations)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAttestationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["attestations", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attestations,
            Pagination,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attestations" => Ok(GeneratedField::Attestations),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAttestationsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryAttestationsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryAttestationsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut attestations__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attestations => {
                            if attestations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestations"));
                            }
                            attestations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAttestationsResponse {
                    attestations: attestations__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.QueryAttestationsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryCountNoncesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.dlc.QueryCountNoncesRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryCountNoncesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCountNoncesRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryCountNoncesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryCountNoncesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryCountNoncesRequest {})
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.QueryCountNoncesRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryCountNoncesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.counts.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.dlc.QueryCountNoncesResponse", len)?;
        if !self.counts.is_empty() {
            struct_ser.serialize_field("counts", &self.counts)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryCountNoncesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["counts"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Counts,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "counts" => Ok(GeneratedField::Counts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCountNoncesResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryCountNoncesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryCountNoncesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut counts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Counts => {
                            if counts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counts"));
                            }
                            counts__ =
                                Some(map_.next_value::<alloc::vec::Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(QueryCountNoncesResponse {
                    counts: counts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.QueryCountNoncesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDcMsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryDCMsRequest", len)?;
        if self.status != 0 {
            let v = DcmStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDcMsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["status", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Pagination,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDcMsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryDCMsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryDcMsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<DcmStatus>()? as i32);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryDcMsRequest {
                    status: status__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryDCMsRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDcMsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dcms.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryDCMsResponse", len)?;
        if !self.dcms.is_empty() {
            struct_ser.serialize_field("dcms", &self.dcms)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDcMsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["dcms", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Dcms,
            Pagination,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dcms" => Ok(GeneratedField::Dcms),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDcMsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryDCMsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryDcMsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut dcms__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Dcms => {
                            if dcms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dcms"));
                            }
                            dcms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryDcMsResponse {
                    dcms: dcms__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryDCMsResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryEventRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryEventRequest", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryEventRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
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
            type Value = QueryEventRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryEventRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryEventRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryEventRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryEventRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryEventResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryEventResponse", len)?;
        if let Some(v) = self.event.as_ref() {
            struct_ser.serialize_field("event", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryEventResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["event"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Event,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "event" => Ok(GeneratedField::Event),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryEventResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryEventResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryEventResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Event => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            event__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryEventResponse { event: event__ })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryEventResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryEventsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.triggered {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryEventsRequest", len)?;
        if self.triggered {
            struct_ser.serialize_field("triggered", &self.triggered)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryEventsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["triggered", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Triggered,
            Pagination,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "triggered" => Ok(GeneratedField::Triggered),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryEventsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryEventsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryEventsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut triggered__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Triggered => {
                            if triggered__.is_some() {
                                return Err(serde::de::Error::duplicate_field("triggered"));
                            }
                            triggered__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryEventsRequest {
                    triggered: triggered__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryEventsRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryEventsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.events.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryEventsResponse", len)?;
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryEventsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["events", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Events,
            Pagination,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "events" => Ok(GeneratedField::Events),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryEventsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryEventsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryEventsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut events__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryEventsResponse {
                    events: events__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryEventsResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryNonceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.oracle_id != 0 {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryNonceRequest", len)?;
        if self.oracle_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "oracleId",
                alloc::string::ToString::to_string(&self.oracle_id).as_str(),
            )?;
        }
        if self.index != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "index",
                alloc::string::ToString::to_string(&self.index).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryNonceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["oracle_id", "oracleId", "index"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OracleId,
            Index,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "oracleId" | "oracle_id" => Ok(GeneratedField::OracleId),
                            "index" => Ok(GeneratedField::Index),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryNonceRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryNonceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryNonceRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut oracle_id__ = None;
                let mut index__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OracleId => {
                            if oracle_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleId"));
                            }
                            oracle_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryNonceRequest {
                    oracle_id: oracle_id__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryNonceRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryNonceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nonce.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryNonceResponse", len)?;
        if let Some(v) = self.nonce.as_ref() {
            struct_ser.serialize_field("nonce", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryNonceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["nonce"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonce,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nonce" => Ok(GeneratedField::Nonce),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryNonceResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryNonceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryNonceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nonce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryNonceResponse { nonce: nonce__ })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryNonceResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryNoncesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.oracle_id != 0 {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryNoncesRequest", len)?;
        if self.oracle_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "oracleId",
                alloc::string::ToString::to_string(&self.oracle_id).as_str(),
            )?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryNoncesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["oracle_id", "oracleId", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OracleId,
            Pagination,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "oracleId" | "oracle_id" => Ok(GeneratedField::OracleId),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryNoncesRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryNoncesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryNoncesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut oracle_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OracleId => {
                            if oracle_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleId"));
                            }
                            oracle_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryNoncesRequest {
                    oracle_id: oracle_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryNoncesRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryNoncesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nonces.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryNoncesResponse", len)?;
        if !self.nonces.is_empty() {
            struct_ser.serialize_field("nonces", &self.nonces)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryNoncesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["nonces", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nonces,
            Pagination,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nonces" => Ok(GeneratedField::Nonces),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryNoncesResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryNoncesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryNoncesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nonces__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nonces => {
                            if nonces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonces"));
                            }
                            nonces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryNoncesResponse {
                    nonces: nonces__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryNoncesResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOraclesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryOraclesRequest", len)?;
        if self.status != 0 {
            let v = DlcOracleStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOraclesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["status", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Pagination,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOraclesRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryOraclesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryOraclesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<DlcOracleStatus>()? as i32);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryOraclesRequest {
                    status: status__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryOraclesRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryOraclesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.oracles.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryOraclesResponse", len)?;
        if !self.oracles.is_empty() {
            struct_ser.serialize_field("oracles", &self.oracles)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryOraclesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["oracles", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Oracles,
            Pagination,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "oracles" => Ok(GeneratedField::Oracles),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryOraclesResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryOraclesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryOraclesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut oracles__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Oracles => {
                            if oracles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracles"));
                            }
                            oracles__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryOraclesResponse {
                    oracles: oracles__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryOraclesResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.dlc.QueryParamsRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryParamsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryParamsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryParamsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {})
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryParamsRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["params"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
        }
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(
                        &self,
                        formatter: &mut core::fmt::Formatter<'_>,
                    ) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryParamsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse { params: params__ })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryParamsResponse", FIELDS, GeneratedVisitor)
    }
}
