// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for Agency {
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
        if !self.address.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.Agency", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
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
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.status != 0 {
            let v = AgencyStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Agency {
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
            "address",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Desc,
            Participants,
            Threshold,
            Address,
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
                            "address" => Ok(GeneratedField::Address),
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
            type Value = Agency;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.Agency")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Agency, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut desc__ = None;
                let mut participants__ = None;
                let mut threshold__ = None;
                let mut address__ = None;
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
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<AgencyStatus>()? as i32);
                        }
                    }
                }
                Ok(Agency {
                    id: id__.unwrap_or_default(),
                    desc: desc__.unwrap_or_default(),
                    participants: participants__.unwrap_or_default(),
                    threshold: threshold__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.Agency", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for AgencyStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Pending => "Agency_Status_Pending",
            Self::Enable => "Agency_status_Enable",
            Self::Disable => "Agency_status_Disable",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AgencyStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Agency_Status_Pending",
            "Agency_status_Enable",
            "Agency_status_Disable",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgencyStatus;

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
                    "Agency_Status_Pending" => Ok(AgencyStatus::Pending),
                    "Agency_status_Enable" => Ok(AgencyStatus::Enable),
                    "Agency_status_Disable" => Ok(AgencyStatus::Disable),
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
        if !self.id.is_empty() {
            len += 1;
        }
        if self.event_id != 0 {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if !self.pubkey.is_empty() {
            len += 1;
        }
        if !self.outcome.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.DLCAttestation", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.event_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventId",
                alloc::string::ToString::to_string(&self.event_id).as_str(),
            )?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if !self.pubkey.is_empty() {
            struct_ser.serialize_field("pubkey", &self.pubkey)?;
        }
        if !self.outcome.is_empty() {
            struct_ser.serialize_field("outcome", &self.outcome)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
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
            "time",
            "pubkey",
            "outcome",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            EventId,
            Time,
            Pubkey,
            Outcome,
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
                            "id" => Ok(GeneratedField::Id),
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            "time" => Ok(GeneratedField::Time),
                            "pubkey" => Ok(GeneratedField::Pubkey),
                            "outcome" => Ok(GeneratedField::Outcome),
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
                let mut time__ = None;
                let mut pubkey__ = None;
                let mut outcome__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
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
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                        GeneratedField::Pubkey => {
                            if pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubkey"));
                            }
                            pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Outcome => {
                            if outcome__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcome"));
                            }
                            outcome__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DlcAttestation {
                    id: id__.unwrap_or_default(),
                    event_id: event_id__.unwrap_or_default(),
                    time: time__,
                    pubkey: pubkey__.unwrap_or_default(),
                    outcome: outcome__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.DLCAttestation", FIELDS, GeneratedVisitor)
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
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.pubkey.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.nonce_index != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.DLCOracle", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
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
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.pubkey.is_empty() {
            struct_ser.serialize_field("pubkey", &self.pubkey)?;
        }
        if self.status != 0 {
            let v = DlcOracleStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        if self.nonce_index != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "nonceIndex",
                alloc::string::ToString::to_string(&self.nonce_index).as_str(),
            )?;
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
            "address",
            "pubkey",
            "status",
            "nonce_index",
            "nonceIndex",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Desc,
            Participants,
            Threshold,
            Address,
            Pubkey,
            Status,
            NonceIndex,
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
                            "address" => Ok(GeneratedField::Address),
                            "pubkey" => Ok(GeneratedField::Pubkey),
                            "status" => Ok(GeneratedField::Status),
                            "nonceIndex" | "nonce_index" => Ok(GeneratedField::NonceIndex),
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
                let mut address__ = None;
                let mut pubkey__ = None;
                let mut status__ = None;
                let mut nonce_index__ = None;
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
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pubkey => {
                            if pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubkey"));
                            }
                            pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<DlcOracleStatus>()? as i32);
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
                    }
                }
                Ok(DlcOracle {
                    id: id__.unwrap_or_default(),
                    desc: desc__.unwrap_or_default(),
                    participants: participants__.unwrap_or_default(),
                    threshold: threshold__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    pubkey: pubkey__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    nonce_index: nonce_index__.unwrap_or_default(),
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
impl serde::Serialize for DlcPriceEvent {
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
        if self.trigger_price != 0 {
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
        if self.has_triggered {
            len += 1;
        }
        if self.publish_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.DLCPriceEvent", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        if self.trigger_price != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "triggerPrice",
                alloc::string::ToString::to_string(&self.trigger_price).as_str(),
            )?;
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
        if self.has_triggered {
            struct_ser.serialize_field("hasTriggered", &self.has_triggered)?;
        }
        if let Some(v) = self.publish_at.as_ref() {
            struct_ser.serialize_field("publishAt", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DlcPriceEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "trigger_price",
            "triggerPrice",
            "nonce",
            "pubkey",
            "description",
            "has_triggered",
            "hasTriggered",
            "publish_at",
            "publishAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            TriggerPrice,
            Nonce,
            Pubkey,
            Description,
            HasTriggered,
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
                            "triggerPrice" | "trigger_price" => Ok(GeneratedField::TriggerPrice),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "pubkey" => Ok(GeneratedField::Pubkey),
                            "description" => Ok(GeneratedField::Description),
                            "hasTriggered" | "has_triggered" => Ok(GeneratedField::HasTriggered),
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
            type Value = DlcPriceEvent;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.DLCPriceEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<DlcPriceEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut trigger_price__ = None;
                let mut nonce__ = None;
                let mut pubkey__ = None;
                let mut description__ = None;
                let mut has_triggered__ = None;
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
                        GeneratedField::TriggerPrice => {
                            if trigger_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("triggerPrice"));
                            }
                            trigger_price__ = Some(
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
                        GeneratedField::HasTriggered => {
                            if has_triggered__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasTriggered"));
                            }
                            has_triggered__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublishAt => {
                            if publish_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publishAt"));
                            }
                            publish_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DlcPriceEvent {
                    id: id__.unwrap_or_default(),
                    trigger_price: trigger_price__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    pubkey: pubkey__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    has_triggered: has_triggered__.unwrap_or_default(),
                    publish_at: publish_at__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.DLCPriceEvent", FIELDS, GeneratedVisitor)
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
        if !self.announcements.is_empty() {
            len += 1;
        }
        if !self.attestations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.announcements.is_empty() {
            struct_ser.serialize_field("announcements", &self.announcements)?;
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
        const FIELDS: &[&str] = &["params", "announcements", "attestations"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            Announcements,
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
                            "announcements" => Ok(GeneratedField::Announcements),
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
                let mut announcements__ = None;
                let mut attestations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::Announcements => {
                            if announcements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("announcements"));
                            }
                            announcements__ = Some(map_.next_value()?);
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
                    announcements: announcements__.unwrap_or_default(),
                    attestations: attestations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.GenesisState", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitAgencyAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.MsgSubmitAgencyAddress", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitAgencyAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["id", "sender", "address", "signature"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Sender,
            Address,
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
                            "id" => Ok(GeneratedField::Id),
                            "sender" => Ok(GeneratedField::Sender),
                            "address" => Ok(GeneratedField::Address),
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
            type Value = MsgSubmitAgencyAddress;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgSubmitAgencyAddress")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitAgencyAddress, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut sender__ = None;
                let mut address__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitAgencyAddress {
                    id: id__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.MsgSubmitAgencyAddress", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitAgencyAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("side.dlc.MsgSubmitAgencyAddressResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitAgencyAddressResponse {
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
            type Value = MsgSubmitAgencyAddressResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgSubmitAgencyAddressResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitAgencyAddressResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitAgencyAddressResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.MsgSubmitAgencyAddressResponse",
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
        if self.announcement_id != 0 {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.MsgSubmitAttestation", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if self.announcement_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "announcementId",
                alloc::string::ToString::to_string(&self.announcement_id).as_str(),
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
        const FIELDS: &[&str] = &["sender", "announcement_id", "announcementId", "signature"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            AnnouncementId,
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
                            "announcementId" | "announcement_id" => {
                                Ok(GeneratedField::AnnouncementId)
                            }
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
                let mut announcement_id__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnouncementId => {
                            if announcement_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("announcementId"));
                            }
                            announcement_id__ = Some(
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
                    announcement_id: announcement_id__.unwrap_or_default(),
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
        if !self.nonce.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.MsgSubmitNonce", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.nonce.is_empty() {
            struct_ser.serialize_field("nonce", &self.nonce)?;
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
        const FIELDS: &[&str] = &["sender", "nonce", "signature"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Nonce,
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
                            "nonce" => Ok(GeneratedField::Nonce),
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
                let mut nonce__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(map_.next_value()?);
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
                    nonce: nonce__.unwrap_or_default(),
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
impl serde::Serialize for MsgSubmitOraclePubkey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.oracle_id.is_empty() {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.pubkey.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.MsgSubmitOraclePubkey", len)?;
        if !self.oracle_id.is_empty() {
            struct_ser.serialize_field("oracleId", &self.oracle_id)?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.pubkey.is_empty() {
            struct_ser.serialize_field("pubkey", &self.pubkey)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitOraclePubkey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["oracle_id", "oracleId", "sender", "pubkey", "signature"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OracleId,
            Sender,
            Pubkey,
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
                            "oracleId" | "oracle_id" => Ok(GeneratedField::OracleId),
                            "sender" => Ok(GeneratedField::Sender),
                            "pubkey" => Ok(GeneratedField::Pubkey),
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
            type Value = MsgSubmitOraclePubkey;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgSubmitOraclePubkey")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitOraclePubkey, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut oracle_id__ = None;
                let mut sender__ = None;
                let mut pubkey__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OracleId => {
                            if oracle_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleId"));
                            }
                            oracle_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(MsgSubmitOraclePubkey {
                    oracle_id: oracle_id__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    pubkey: pubkey__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.MsgSubmitOraclePubkey", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitOraclePubkeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("side.dlc.MsgSubmitOraclePubkeyResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitOraclePubkeyResponse {
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
            type Value = MsgSubmitOraclePubkeyResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.MsgSubmitOraclePubkeyResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitOraclePubkeyResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitOraclePubkeyResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.MsgSubmitOraclePubkeyResponse",
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
        if self.nonce_queue_size != 0 {
            len += 1;
        }
        if !self.price_interval.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.Params", len)?;
        if self.nonce_queue_size != 0 {
            struct_ser.serialize_field("nonceQueueSize", &self.nonce_queue_size)?;
        }
        if !self.price_interval.is_empty() {
            struct_ser.serialize_field("priceInterval", &self.price_interval)?;
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
            "nonce_queue_size",
            "nonceQueueSize",
            "price_interval",
            "priceInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NonceQueueSize,
            PriceInterval,
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
                            "nonceQueueSize" | "nonce_queue_size" => {
                                Ok(GeneratedField::NonceQueueSize)
                            }
                            "priceInterval" | "price_interval" => Ok(GeneratedField::PriceInterval),
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
                let mut nonce_queue_size__ = None;
                let mut price_interval__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NonceQueueSize => {
                            if nonce_queue_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonceQueueSize"));
                            }
                            nonce_queue_size__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PriceInterval => {
                            if price_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceInterval"));
                            }
                            price_interval__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    nonce_queue_size: nonce_queue_size__.unwrap_or_default(),
                    price_interval: price_interval__.unwrap_or_default(),
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
impl serde::Serialize for QueryAgenciesRequest {
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
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryAgenciesRequest", len)?;
        if self.status != 0 {
            let v = AgencyStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAgenciesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["status"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = QueryAgenciesRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryAgenciesRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryAgenciesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<AgencyStatus>()? as i32);
                        }
                    }
                }
                Ok(QueryAgenciesRequest {
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryAgenciesRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryAgenciesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.agencies.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryAgenciesResponse", len)?;
        if !self.agencies.is_empty() {
            struct_ser.serialize_field("agencies", &self.agencies)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAgenciesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["agencies"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Agencies,
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
                            "agencies" => Ok(GeneratedField::Agencies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAgenciesResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryAgenciesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryAgenciesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut agencies__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Agencies => {
                            if agencies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agencies"));
                            }
                            agencies__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryAgenciesResponse {
                    agencies: agencies__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryAgenciesResponse", FIELDS, GeneratedVisitor)
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
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.dlc.QueryAttestationRequest", len)?;
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
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryAttestationRequest {})
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
        if !self.attestations.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.dlc.QueryAttestationResponse", len)?;
        if !self.attestations.is_empty() {
            struct_ser.serialize_field("attestations", &self.attestations)?;
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
        const FIELDS: &[&str] = &["attestations"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                let mut attestations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attestations => {
                            if attestations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestations"));
                            }
                            attestations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryAttestationResponse {
                    attestations: attestations__.unwrap_or_default(),
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
impl serde::Serialize for QueryNoncesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.dlc.QueryNoncesRequest", len)?;
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
            type Value = QueryNoncesRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryNoncesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryNoncesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryNoncesRequest {})
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
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryOraclesRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for QueryOraclesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["status"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<DlcOracleStatus>()? as i32);
                        }
                    }
                }
                Ok(QueryOraclesRequest {
                    status: status__.unwrap_or_default(),
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
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryOraclesResponse", len)?;
        if !self.oracles.is_empty() {
            struct_ser.serialize_field("oracles", &self.oracles)?;
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
        const FIELDS: &[&str] = &["oracles"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Oracles,
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
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Oracles => {
                            if oracles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracles"));
                            }
                            oracles__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryOraclesResponse {
                    oracles: oracles__.unwrap_or_default(),
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
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPriceEventRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.has_triggered {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryPriceEventRequest", len)?;
        if self.has_triggered {
            struct_ser.serialize_field("hasTriggered", &self.has_triggered)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPriceEventRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["has_triggered", "hasTriggered", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HasTriggered,
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
                            "hasTriggered" | "has_triggered" => Ok(GeneratedField::HasTriggered),
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
            type Value = QueryPriceEventRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryPriceEventRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryPriceEventRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut has_triggered__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HasTriggered => {
                            if has_triggered__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasTriggered"));
                            }
                            has_triggered__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryPriceEventRequest {
                    has_triggered: has_triggered__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryPriceEventRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPriceEventResponse {
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
        let mut struct_ser =
            serializer.serialize_struct("side.dlc.QueryPriceEventResponse", len)?;
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
impl<'de> serde::Deserialize<'de> for QueryPriceEventResponse {
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
            type Value = QueryPriceEventResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryPriceEventResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryPriceEventResponse, V::Error>
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
                Ok(QueryPriceEventResponse {
                    events: events__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.dlc.QueryPriceEventResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPriceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.symbol.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryPriceRequest", len)?;
        if !self.symbol.is_empty() {
            struct_ser.serialize_field("symbol", &self.symbol)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPriceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["symbol"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Symbol,
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
                            "symbol" => Ok(GeneratedField::Symbol),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPriceRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryPriceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryPriceRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut symbol__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Symbol => {
                            if symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symbol"));
                            }
                            symbol__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPriceRequest {
                    symbol: symbol__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryPriceRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPriceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.price != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.dlc.QueryPriceResponse", len)?;
        if self.price != 0 {
            struct_ser.serialize_field("price", &self.price)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPriceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["price"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Price,
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
                            "price" => Ok(GeneratedField::Price),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPriceResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.dlc.QueryPriceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryPriceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryPriceResponse {
                    price: price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.dlc.QueryPriceResponse", FIELDS, GeneratedVisitor)
    }
}
