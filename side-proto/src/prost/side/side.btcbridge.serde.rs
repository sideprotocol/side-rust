// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for AssetType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ASSET_TYPE_UNSPECIFIED",
            Self::Btc => "ASSET_TYPE_BTC",
            Self::Brc20 => "ASSET_TYPE_BRC20",
            Self::Runes => "ASSET_TYPE_RUNES",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AssetType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ASSET_TYPE_UNSPECIFIED",
            "ASSET_TYPE_BTC",
            "ASSET_TYPE_BRC20",
            "ASSET_TYPE_RUNES",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetType;

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
                    "ASSET_TYPE_UNSPECIFIED" => Ok(AssetType::Unspecified),
                    "ASSET_TYPE_BTC" => Ok(AssetType::Btc),
                    "ASSET_TYPE_BRC20" => Ok(AssetType::Brc20),
                    "ASSET_TYPE_RUNES" => Ok(AssetType::Runes),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BlockHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if !self.previous_block_hash.is_empty() {
            len += 1;
        }
        if !self.merkle_root.is_empty() {
            len += 1;
        }
        if self.nonce != 0 {
            len += 1;
        }
        if !self.bits.is_empty() {
            len += 1;
        }
        if self.time != 0 {
            len += 1;
        }
        if self.ntx != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.BlockHeader", len)?;
        if self.version != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("version", alloc::string::ToString::to_string(&self.version).as_str())?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", alloc::string::ToString::to_string(&self.height).as_str())?;
        }
        if !self.previous_block_hash.is_empty() {
            struct_ser.serialize_field("previousBlockHash", &self.previous_block_hash)?;
        }
        if !self.merkle_root.is_empty() {
            struct_ser.serialize_field("merkleRoot", &self.merkle_root)?;
        }
        if self.nonce != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("nonce", alloc::string::ToString::to_string(&self.nonce).as_str())?;
        }
        if !self.bits.is_empty() {
            struct_ser.serialize_field("bits", &self.bits)?;
        }
        if self.time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("time", alloc::string::ToString::to_string(&self.time).as_str())?;
        }
        if self.ntx != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("ntx", alloc::string::ToString::to_string(&self.ntx).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BlockHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "hash",
            "height",
            "previous_block_hash",
            "previousBlockHash",
            "merkle_root",
            "merkleRoot",
            "nonce",
            "bits",
            "time",
            "ntx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Hash,
            Height,
            PreviousBlockHash,
            MerkleRoot,
            Nonce,
            Bits,
            Time,
            Ntx,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "hash" => Ok(GeneratedField::Hash),
                            "height" => Ok(GeneratedField::Height),
                            "previousBlockHash" | "previous_block_hash" => Ok(GeneratedField::PreviousBlockHash),
                            "merkleRoot" | "merkle_root" => Ok(GeneratedField::MerkleRoot),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "bits" => Ok(GeneratedField::Bits),
                            "time" => Ok(GeneratedField::Time),
                            "ntx" => Ok(GeneratedField::Ntx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockHeader;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.BlockHeader")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<BlockHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ =None;
                let mut hash__ =None;
                let mut height__ =None;
                let mut previous_block_hash__ =None;
                let mut merkle_root__ =None;
                let mut nonce__ =None;
                let mut bits__ =None;
                let mut time__ =None;
                let mut ntx__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PreviousBlockHash => {
                            if previous_block_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousBlockHash"));
                            }
                            previous_block_hash__ =Some(map_.next_value()?);
                        }
                        GeneratedField::MerkleRoot => {
                            if merkle_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("merkleRoot"));
                            }
                            merkle_root__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Bits => {
                            if bits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bits"));
                            }
                            bits__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Ntx => {
                            if ntx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ntx"));
                            }
                            ntx__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BlockHeader {
                    version: version__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    previous_block_hash: previous_block_hash__.unwrap_or_default(),
                    merkle_root: merkle_root__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    bits: bits__.unwrap_or_default(),
                    time: time__.unwrap_or_default(),
                    ntx: ntx__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.BlockHeader", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for BtcConsolidation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.target_threshold != 0 {
            len += 1;
        }
        if self.max_num != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.BtcConsolidation", len)?;
        if self.target_threshold != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("targetThreshold", alloc::string::ToString::to_string(&self.target_threshold).as_str())?;
        }
        if self.max_num != 0 {
            struct_ser.serialize_field("maxNum", &self.max_num)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BtcConsolidation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target_threshold",
            "targetThreshold",
            "max_num",
            "maxNum",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TargetThreshold,
            MaxNum,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "targetThreshold" | "target_threshold" => Ok(GeneratedField::TargetThreshold),
                            "maxNum" | "max_num" => Ok(GeneratedField::MaxNum),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BtcConsolidation;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.BtcConsolidation")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<BtcConsolidation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target_threshold__ =None;
                let mut max_num__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TargetThreshold => {
                            if target_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetThreshold"));
                            }
                            target_threshold__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxNum => {
                            if max_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxNum"));
                            }
                            max_num__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BtcConsolidation {
                    target_threshold: target_threshold__.unwrap_or_default(),
                    max_num: max_num__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.BtcConsolidation", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DkgCompletionRequest {
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
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.vaults.is_empty() {
            len += 1;
        }
        if !self.consensus_address.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.DKGCompletionRequest", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.vaults.is_empty() {
            struct_ser.serialize_field("vaults", &self.vaults)?;
        }
        if !self.consensus_address.is_empty() {
            struct_ser.serialize_field("consensusAddress", &self.consensus_address)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DkgCompletionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "sender",
            "vaults",
            "consensus_address",
            "consensusAddress",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Sender,
            Vaults,
            ConsensusAddress,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
                            "vaults" => Ok(GeneratedField::Vaults),
                            "consensusAddress" | "consensus_address" => Ok(GeneratedField::ConsensusAddress),
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
            type Value = DkgCompletionRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.DKGCompletionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<DkgCompletionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ =None;
                let mut sender__ =None;
                let mut vaults__ =None;
                let mut consensus_address__ =None;
                let mut signature__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Vaults => {
                            if vaults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaults"));
                            }
                            vaults__ =Some(map_.next_value()?);
                        }
                        GeneratedField::ConsensusAddress => {
                            if consensus_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusAddress"));
                            }
                            consensus_address__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DkgCompletionRequest {
                    id: id__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    vaults: vaults__.unwrap_or_default(),
                    consensus_address: consensus_address__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.DKGCompletionRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DkgParticipant {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.moniker.is_empty() {
            len += 1;
        }
        if !self.operator_address.is_empty() {
            len += 1;
        }
        if !self.consensus_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.DKGParticipant", len)?;
        if !self.moniker.is_empty() {
            struct_ser.serialize_field("moniker", &self.moniker)?;
        }
        if !self.operator_address.is_empty() {
            struct_ser.serialize_field("operatorAddress", &self.operator_address)?;
        }
        if !self.consensus_address.is_empty() {
            struct_ser.serialize_field("consensusAddress", &self.consensus_address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DkgParticipant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "moniker",
            "operator_address",
            "operatorAddress",
            "consensus_address",
            "consensusAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Moniker,
            OperatorAddress,
            ConsensusAddress,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "moniker" => Ok(GeneratedField::Moniker),
                            "operatorAddress" | "operator_address" => Ok(GeneratedField::OperatorAddress),
                            "consensusAddress" | "consensus_address" => Ok(GeneratedField::ConsensusAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DkgParticipant;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.DKGParticipant")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<DkgParticipant, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut moniker__ =None;
                let mut operator_address__ =None;
                let mut consensus_address__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Moniker => {
                            if moniker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moniker"));
                            }
                            moniker__ =Some(map_.next_value()?);
                        }
                        GeneratedField::OperatorAddress => {
                            if operator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operatorAddress"));
                            }
                            operator_address__ =Some(map_.next_value()?);
                        }
                        GeneratedField::ConsensusAddress => {
                            if consensus_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusAddress"));
                            }
                            consensus_address__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DkgParticipant {
                    moniker: moniker__.unwrap_or_default(),
                    operator_address: operator_address__.unwrap_or_default(),
                    consensus_address: consensus_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.DKGParticipant", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DkgRequest {
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
        if !self.participants.is_empty() {
            len += 1;
        }
        if self.threshold != 0 {
            len += 1;
        }
        if !self.vault_types.is_empty() {
            len += 1;
        }
        if self.enable_transfer {
            len += 1;
        }
        if self.target_utxo_num != 0 {
            len += 1;
        }
        if self.expiration.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.DKGRequest", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        if !self.participants.is_empty() {
            struct_ser.serialize_field("participants", &self.participants)?;
        }
        if self.threshold != 0 {
            struct_ser.serialize_field("threshold", &self.threshold)?;
        }
        if !self.vault_types.is_empty() {
            let v = self.vault_types.iter().cloned().map(|v| {
                AssetType::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(alloc::format!("Invalid variant {}", v)))
                }).collect::<Result<alloc::vec::Vec<_>, _>>()?;
            struct_ser.serialize_field("vaultTypes", &v)?;
        }
        if self.enable_transfer {
            struct_ser.serialize_field("enableTransfer", &self.enable_transfer)?;
        }
        if self.target_utxo_num != 0 {
            struct_ser.serialize_field("targetUtxoNum", &self.target_utxo_num)?;
        }
        if let Some(v) = self.expiration.as_ref() {
            struct_ser.serialize_field("expiration", v)?;
        }
        if self.status != 0 {
            let v = DkgRequestStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DkgRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "participants",
            "threshold",
            "vault_types",
            "vaultTypes",
            "enable_transfer",
            "enableTransfer",
            "target_utxo_num",
            "targetUtxoNum",
            "expiration",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Participants,
            Threshold,
            VaultTypes,
            EnableTransfer,
            TargetUtxoNum,
            Expiration,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "participants" => Ok(GeneratedField::Participants),
                            "threshold" => Ok(GeneratedField::Threshold),
                            "vaultTypes" | "vault_types" => Ok(GeneratedField::VaultTypes),
                            "enableTransfer" | "enable_transfer" => Ok(GeneratedField::EnableTransfer),
                            "targetUtxoNum" | "target_utxo_num" => Ok(GeneratedField::TargetUtxoNum),
                            "expiration" => Ok(GeneratedField::Expiration),
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
            type Value = DkgRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.DKGRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<DkgRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ =None;
                let mut participants__ =None;
                let mut threshold__ =None;
                let mut vault_types__ =None;
                let mut enable_transfer__ =None;
                let mut target_utxo_num__ =None;
                let mut expiration__ =None;
                let mut status__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Participants => {
                            if participants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("participants"));
                            }
                            participants__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Threshold => {
                            if threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threshold"));
                            }
                            threshold__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VaultTypes => {
                            if vault_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultTypes"));
                            }
                            vault_types__ =Some(map_.next_value::<alloc::vec::Vec<AssetType>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::EnableTransfer => {
                            if enable_transfer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableTransfer"));
                            }
                            enable_transfer__ =Some(map_.next_value()?);
                        }
                        GeneratedField::TargetUtxoNum => {
                            if target_utxo_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetUtxoNum"));
                            }
                            target_utxo_num__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Expiration => {
                            if expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiration"));
                            }
                            expiration__ =map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ =Some(map_.next_value::<DkgRequestStatus>()? as i32);
                        }
                    }
                }
                Ok(DkgRequest {
                    id: id__.unwrap_or_default(),
                    participants: participants__.unwrap_or_default(),
                    threshold: threshold__.unwrap_or_default(),
                    vault_types: vault_types__.unwrap_or_default(),
                    enable_transfer: enable_transfer__.unwrap_or_default(),
                    target_utxo_num: target_utxo_num__.unwrap_or_default(),
                    expiration: expiration__,
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.DKGRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DkgRequestStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DKG_REQUEST_STATUS_UNSPECIFIED",
            Self::Pending => "DKG_REQUEST_STATUS_PENDING",
            Self::Completed => "DKG_REQUEST_STATUS_COMPLETED",
            Self::Failed => "DKG_REQUEST_STATUS_FAILED",
            Self::Timedout => "DKG_REQUEST_STATUS_TIMEDOUT",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DkgRequestStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DKG_REQUEST_STATUS_UNSPECIFIED",
            "DKG_REQUEST_STATUS_PENDING",
            "DKG_REQUEST_STATUS_COMPLETED",
            "DKG_REQUEST_STATUS_FAILED",
            "DKG_REQUEST_STATUS_TIMEDOUT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DkgRequestStatus;

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
                    "DKG_REQUEST_STATUS_UNSPECIFIED" => Ok(DkgRequestStatus::Unspecified),
                    "DKG_REQUEST_STATUS_PENDING" => Ok(DkgRequestStatus::Pending),
                    "DKG_REQUEST_STATUS_COMPLETED" => Ok(DkgRequestStatus::Completed),
                    "DKG_REQUEST_STATUS_FAILED" => Ok(DkgRequestStatus::Failed),
                    "DKG_REQUEST_STATUS_TIMEDOUT" => Ok(DkgRequestStatus::Timedout),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Edict {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id.is_some() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.output != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.Edict", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if self.output != 0 {
            struct_ser.serialize_field("output", &self.output)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Edict {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "amount",
            "output",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Amount,
            Output,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "amount" => Ok(GeneratedField::Amount),
                            "output" => Ok(GeneratedField::Output),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Edict;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.Edict")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Edict, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ =None;
                let mut amount__ =None;
                let mut output__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ =map_.next_value()?;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Output => {
                            if output__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output"));
                            }
                            output__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Edict {
                    id: id__,
                    amount: amount__.unwrap_or_default(),
                    output: output__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.Edict", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for FeeRate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0 {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.FeeRate", len)?;
        if self.value != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", alloc::string::ToString::to_string(&self.value).as_str())?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", alloc::string::ToString::to_string(&self.height).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FeeRate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "height",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Height,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            "height" => Ok(GeneratedField::Height),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FeeRate;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.FeeRate")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<FeeRate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ =None;
                let mut height__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(FeeRate {
                    value: value__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.FeeRate", FIELDS, GeneratedVisitor)
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
        if self.best_block_header.is_some() {
            len += 1;
        }
        if !self.block_headers.is_empty() {
            len += 1;
        }
        if !self.utxos.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if let Some(v) = self.best_block_header.as_ref() {
            struct_ser.serialize_field("bestBlockHeader", v)?;
        }
        if !self.block_headers.is_empty() {
            struct_ser.serialize_field("blockHeaders", &self.block_headers)?;
        }
        if !self.utxos.is_empty() {
            struct_ser.serialize_field("utxos", &self.utxos)?;
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
        const FIELDS: &[&str] = &[
            "params",
            "best_block_header",
            "bestBlockHeader",
            "block_headers",
            "blockHeaders",
            "utxos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            BestBlockHeader,
            BlockHeaders,
            Utxos,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "params" => Ok(GeneratedField::Params),
                            "bestBlockHeader" | "best_block_header" => Ok(GeneratedField::BestBlockHeader),
                            "blockHeaders" | "block_headers" => Ok(GeneratedField::BlockHeaders),
                            "utxos" => Ok(GeneratedField::Utxos),
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
                formatter.write_str("struct side.btcbridge.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<GenesisState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ =None;
                let mut best_block_header__ =None;
                let mut block_headers__ =None;
                let mut utxos__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ =map_.next_value()?;
                        }
                        GeneratedField::BestBlockHeader => {
                            if best_block_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bestBlockHeader"));
                            }
                            best_block_header__ =map_.next_value()?;
                        }
                        GeneratedField::BlockHeaders => {
                            if block_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeaders"));
                            }
                            block_headers__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Utxos => {
                            if utxos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("utxos"));
                            }
                            utxos__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    best_block_header: best_block_header__,
                    block_headers: block_headers__.unwrap_or_default(),
                    utxos: utxos__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.GenesisState", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCompleteDkg {
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
        if self.id != 0 {
            len += 1;
        }
        if !self.vaults.is_empty() {
            len += 1;
        }
        if !self.consensus_address.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgCompleteDKG", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        if !self.vaults.is_empty() {
            struct_ser.serialize_field("vaults", &self.vaults)?;
        }
        if !self.consensus_address.is_empty() {
            struct_ser.serialize_field("consensusAddress", &self.consensus_address)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCompleteDkg {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "id",
            "vaults",
            "consensus_address",
            "consensusAddress",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Id,
            Vaults,
            ConsensusAddress,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "id" => Ok(GeneratedField::Id),
                            "vaults" => Ok(GeneratedField::Vaults),
                            "consensusAddress" | "consensus_address" => Ok(GeneratedField::ConsensusAddress),
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
            type Value = MsgCompleteDkg;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgCompleteDKG")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgCompleteDkg, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ =None;
                let mut id__ =None;
                let mut vaults__ =None;
                let mut consensus_address__ =None;
                let mut signature__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vaults => {
                            if vaults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaults"));
                            }
                            vaults__ =Some(map_.next_value()?);
                        }
                        GeneratedField::ConsensusAddress => {
                            if consensus_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusAddress"));
                            }
                            consensus_address__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgCompleteDkg {
                    sender: sender__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    vaults: vaults__.unwrap_or_default(),
                    consensus_address: consensus_address__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgCompleteDKG", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCompleteDkgResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgCompleteDKGResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCompleteDkgResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MsgCompleteDkgResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgCompleteDKGResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgCompleteDkgResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCompleteDkgResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgCompleteDKGResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgConsolidateVaults {
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
        if self.vault_version != 0 {
            len += 1;
        }
        if self.btc_consolidation.is_some() {
            len += 1;
        }
        if !self.runes_consolidations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgConsolidateVaults", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if self.vault_version != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("vaultVersion", alloc::string::ToString::to_string(&self.vault_version).as_str())?;
        }
        if let Some(v) = self.btc_consolidation.as_ref() {
            struct_ser.serialize_field("btcConsolidation", v)?;
        }
        if !self.runes_consolidations.is_empty() {
            struct_ser.serialize_field("runesConsolidations", &self.runes_consolidations)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgConsolidateVaults {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "vault_version",
            "vaultVersion",
            "btc_consolidation",
            "btcConsolidation",
            "runes_consolidations",
            "runesConsolidations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            VaultVersion,
            BtcConsolidation,
            RunesConsolidations,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authority" => Ok(GeneratedField::Authority),
                            "vaultVersion" | "vault_version" => Ok(GeneratedField::VaultVersion),
                            "btcConsolidation" | "btc_consolidation" => Ok(GeneratedField::BtcConsolidation),
                            "runesConsolidations" | "runes_consolidations" => Ok(GeneratedField::RunesConsolidations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgConsolidateVaults;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgConsolidateVaults")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgConsolidateVaults, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ =None;
                let mut vault_version__ =None;
                let mut btc_consolidation__ =None;
                let mut runes_consolidations__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ =Some(map_.next_value()?);
                        }
                        GeneratedField::VaultVersion => {
                            if vault_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultVersion"));
                            }
                            vault_version__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BtcConsolidation => {
                            if btc_consolidation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("btcConsolidation"));
                            }
                            btc_consolidation__ =map_.next_value()?;
                        }
                        GeneratedField::RunesConsolidations => {
                            if runes_consolidations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runesConsolidations"));
                            }
                            runes_consolidations__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgConsolidateVaults {
                    authority: authority__.unwrap_or_default(),
                    vault_version: vault_version__.unwrap_or_default(),
                    btc_consolidation: btc_consolidation__,
                    runes_consolidations: runes_consolidations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgConsolidateVaults", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgConsolidateVaultsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgConsolidateVaultsResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgConsolidateVaultsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MsgConsolidateVaultsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgConsolidateVaultsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgConsolidateVaultsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgConsolidateVaultsResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgConsolidateVaultsResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgInitiateDkg {
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
        if !self.vault_types.is_empty() {
            len += 1;
        }
        if self.enable_transfer {
            len += 1;
        }
        if self.target_utxo_num != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgInitiateDKG", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.participants.is_empty() {
            struct_ser.serialize_field("participants", &self.participants)?;
        }
        if self.threshold != 0 {
            struct_ser.serialize_field("threshold", &self.threshold)?;
        }
        if !self.vault_types.is_empty() {
            let v = self.vault_types.iter().cloned().map(|v| {
                AssetType::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(alloc::format!("Invalid variant {}", v)))
                }).collect::<Result<alloc::vec::Vec<_>, _>>()?;
            struct_ser.serialize_field("vaultTypes", &v)?;
        }
        if self.enable_transfer {
            struct_ser.serialize_field("enableTransfer", &self.enable_transfer)?;
        }
        if self.target_utxo_num != 0 {
            struct_ser.serialize_field("targetUtxoNum", &self.target_utxo_num)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgInitiateDkg {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "participants",
            "threshold",
            "vault_types",
            "vaultTypes",
            "enable_transfer",
            "enableTransfer",
            "target_utxo_num",
            "targetUtxoNum",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Participants,
            Threshold,
            VaultTypes,
            EnableTransfer,
            TargetUtxoNum,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
                            "vaultTypes" | "vault_types" => Ok(GeneratedField::VaultTypes),
                            "enableTransfer" | "enable_transfer" => Ok(GeneratedField::EnableTransfer),
                            "targetUtxoNum" | "target_utxo_num" => Ok(GeneratedField::TargetUtxoNum),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgInitiateDkg;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgInitiateDKG")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgInitiateDkg, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ =None;
                let mut participants__ =None;
                let mut threshold__ =None;
                let mut vault_types__ =None;
                let mut enable_transfer__ =None;
                let mut target_utxo_num__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Participants => {
                            if participants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("participants"));
                            }
                            participants__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Threshold => {
                            if threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threshold"));
                            }
                            threshold__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VaultTypes => {
                            if vault_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultTypes"));
                            }
                            vault_types__ =Some(map_.next_value::<alloc::vec::Vec<AssetType>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::EnableTransfer => {
                            if enable_transfer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableTransfer"));
                            }
                            enable_transfer__ =Some(map_.next_value()?);
                        }
                        GeneratedField::TargetUtxoNum => {
                            if target_utxo_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetUtxoNum"));
                            }
                            target_utxo_num__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MsgInitiateDkg {
                    authority: authority__.unwrap_or_default(),
                    participants: participants__.unwrap_or_default(),
                    threshold: threshold__.unwrap_or_default(),
                    vault_types: vault_types__.unwrap_or_default(),
                    enable_transfer: enable_transfer__.unwrap_or_default(),
                    target_utxo_num: target_utxo_num__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgInitiateDKG", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgInitiateDkgResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgInitiateDKGResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgInitiateDkgResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MsgInitiateDkgResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgInitiateDKGResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgInitiateDkgResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgInitiateDkgResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgInitiateDKGResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitBlockHeaders {
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
        if !self.block_headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgSubmitBlockHeaders", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.block_headers.is_empty() {
            struct_ser.serialize_field("blockHeaders", &self.block_headers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitBlockHeaders {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "block_headers",
            "blockHeaders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            BlockHeaders,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "blockHeaders" | "block_headers" => Ok(GeneratedField::BlockHeaders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitBlockHeaders;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgSubmitBlockHeaders")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitBlockHeaders, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ =None;
                let mut block_headers__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ =Some(map_.next_value()?);
                        }
                        GeneratedField::BlockHeaders => {
                            if block_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeaders"));
                            }
                            block_headers__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitBlockHeaders {
                    sender: sender__.unwrap_or_default(),
                    block_headers: block_headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgSubmitBlockHeaders", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitBlockHeadersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgSubmitBlockHeadersResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitBlockHeadersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MsgSubmitBlockHeadersResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgSubmitBlockHeadersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitBlockHeadersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitBlockHeadersResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgSubmitBlockHeadersResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitDepositTransaction {
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
        if !self.blockhash.is_empty() {
            len += 1;
        }
        if !self.prev_tx_bytes.is_empty() {
            len += 1;
        }
        if !self.tx_bytes.is_empty() {
            len += 1;
        }
        if !self.proof.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgSubmitDepositTransaction", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.blockhash.is_empty() {
            struct_ser.serialize_field("blockhash", &self.blockhash)?;
        }
        if !self.prev_tx_bytes.is_empty() {
            struct_ser.serialize_field("prevTxBytes", &self.prev_tx_bytes)?;
        }
        if !self.tx_bytes.is_empty() {
            struct_ser.serialize_field("txBytes", &self.tx_bytes)?;
        }
        if !self.proof.is_empty() {
            struct_ser.serialize_field("proof", &self.proof)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitDepositTransaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "blockhash",
            "prev_tx_bytes",
            "prevTxBytes",
            "tx_bytes",
            "txBytes",
            "proof",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Blockhash,
            PrevTxBytes,
            TxBytes,
            Proof,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "blockhash" => Ok(GeneratedField::Blockhash),
                            "prevTxBytes" | "prev_tx_bytes" => Ok(GeneratedField::PrevTxBytes),
                            "txBytes" | "tx_bytes" => Ok(GeneratedField::TxBytes),
                            "proof" => Ok(GeneratedField::Proof),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitDepositTransaction;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgSubmitDepositTransaction")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitDepositTransaction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ =None;
                let mut blockhash__ =None;
                let mut prev_tx_bytes__ =None;
                let mut tx_bytes__ =None;
                let mut proof__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Blockhash => {
                            if blockhash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockhash"));
                            }
                            blockhash__ =Some(map_.next_value()?);
                        }
                        GeneratedField::PrevTxBytes => {
                            if prev_tx_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prevTxBytes"));
                            }
                            prev_tx_bytes__ =Some(map_.next_value()?);
                        }
                        GeneratedField::TxBytes => {
                            if tx_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txBytes"));
                            }
                            tx_bytes__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Proof => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proof"));
                            }
                            proof__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitDepositTransaction {
                    sender: sender__.unwrap_or_default(),
                    blockhash: blockhash__.unwrap_or_default(),
                    prev_tx_bytes: prev_tx_bytes__.unwrap_or_default(),
                    tx_bytes: tx_bytes__.unwrap_or_default(),
                    proof: proof__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgSubmitDepositTransaction", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitDepositTransactionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgSubmitDepositTransactionResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitDepositTransactionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MsgSubmitDepositTransactionResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgSubmitDepositTransactionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitDepositTransactionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitDepositTransactionResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgSubmitDepositTransactionResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitFeeRate {
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
        if self.fee_rate != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgSubmitFeeRate", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if self.fee_rate != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("feeRate", alloc::string::ToString::to_string(&self.fee_rate).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitFeeRate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "fee_rate",
            "feeRate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            FeeRate,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "feeRate" | "fee_rate" => Ok(GeneratedField::FeeRate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitFeeRate;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgSubmitFeeRate")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitFeeRate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ =None;
                let mut fee_rate__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ =Some(map_.next_value()?);
                        }
                        GeneratedField::FeeRate => {
                            if fee_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeRate"));
                            }
                            fee_rate__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MsgSubmitFeeRate {
                    sender: sender__.unwrap_or_default(),
                    fee_rate: fee_rate__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgSubmitFeeRate", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitFeeRateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgSubmitFeeRateResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitFeeRateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MsgSubmitFeeRateResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgSubmitFeeRateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitFeeRateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitFeeRateResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgSubmitFeeRateResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitSignatures {
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
        if !self.txid.is_empty() {
            len += 1;
        }
        if !self.psbt.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgSubmitSignatures", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.txid.is_empty() {
            struct_ser.serialize_field("txid", &self.txid)?;
        }
        if !self.psbt.is_empty() {
            struct_ser.serialize_field("psbt", &self.psbt)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitSignatures {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "txid",
            "psbt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Txid,
            Psbt,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "txid" => Ok(GeneratedField::Txid),
                            "psbt" => Ok(GeneratedField::Psbt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitSignatures;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgSubmitSignatures")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitSignatures, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ =None;
                let mut txid__ =None;
                let mut psbt__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Txid => {
                            if txid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txid"));
                            }
                            txid__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Psbt => {
                            if psbt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("psbt"));
                            }
                            psbt__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitSignatures {
                    sender: sender__.unwrap_or_default(),
                    txid: txid__.unwrap_or_default(),
                    psbt: psbt__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgSubmitSignatures", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitSignaturesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgSubmitSignaturesResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitSignaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MsgSubmitSignaturesResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgSubmitSignaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitSignaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitSignaturesResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgSubmitSignaturesResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitWithdrawTransaction {
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
        if !self.blockhash.is_empty() {
            len += 1;
        }
        if !self.tx_bytes.is_empty() {
            len += 1;
        }
        if !self.proof.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgSubmitWithdrawTransaction", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.blockhash.is_empty() {
            struct_ser.serialize_field("blockhash", &self.blockhash)?;
        }
        if !self.tx_bytes.is_empty() {
            struct_ser.serialize_field("txBytes", &self.tx_bytes)?;
        }
        if !self.proof.is_empty() {
            struct_ser.serialize_field("proof", &self.proof)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitWithdrawTransaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "blockhash",
            "tx_bytes",
            "txBytes",
            "proof",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Blockhash,
            TxBytes,
            Proof,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "blockhash" => Ok(GeneratedField::Blockhash),
                            "txBytes" | "tx_bytes" => Ok(GeneratedField::TxBytes),
                            "proof" => Ok(GeneratedField::Proof),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSubmitWithdrawTransaction;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgSubmitWithdrawTransaction")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitWithdrawTransaction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ =None;
                let mut blockhash__ =None;
                let mut tx_bytes__ =None;
                let mut proof__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Blockhash => {
                            if blockhash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockhash"));
                            }
                            blockhash__ =Some(map_.next_value()?);
                        }
                        GeneratedField::TxBytes => {
                            if tx_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txBytes"));
                            }
                            tx_bytes__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Proof => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proof"));
                            }
                            proof__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitWithdrawTransaction {
                    sender: sender__.unwrap_or_default(),
                    blockhash: blockhash__.unwrap_or_default(),
                    tx_bytes: tx_bytes__.unwrap_or_default(),
                    proof: proof__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgSubmitWithdrawTransaction", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitWithdrawTransactionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgSubmitWithdrawTransactionResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitWithdrawTransactionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MsgSubmitWithdrawTransactionResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgSubmitWithdrawTransactionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitWithdrawTransactionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitWithdrawTransactionResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgSubmitWithdrawTransactionResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgTransferVault {
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
        if self.source_version != 0 {
            len += 1;
        }
        if self.dest_version != 0 {
            len += 1;
        }
        if self.asset_type != 0 {
            len += 1;
        }
        if !self.psbts.is_empty() {
            len += 1;
        }
        if self.target_utxo_num != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgTransferVault", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if self.source_version != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sourceVersion", alloc::string::ToString::to_string(&self.source_version).as_str())?;
        }
        if self.dest_version != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("destVersion", alloc::string::ToString::to_string(&self.dest_version).as_str())?;
        }
        if self.asset_type != 0 {
            let v = AssetType::try_from(self.asset_type)
                .map_err(|_| serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.asset_type)))?;
            struct_ser.serialize_field("assetType", &v)?;
        }
        if !self.psbts.is_empty() {
            struct_ser.serialize_field("psbts", &self.psbts)?;
        }
        if self.target_utxo_num != 0 {
            struct_ser.serialize_field("targetUtxoNum", &self.target_utxo_num)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgTransferVault {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "source_version",
            "sourceVersion",
            "dest_version",
            "destVersion",
            "asset_type",
            "assetType",
            "psbts",
            "target_utxo_num",
            "targetUtxoNum",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            SourceVersion,
            DestVersion,
            AssetType,
            Psbts,
            TargetUtxoNum,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authority" => Ok(GeneratedField::Authority),
                            "sourceVersion" | "source_version" => Ok(GeneratedField::SourceVersion),
                            "destVersion" | "dest_version" => Ok(GeneratedField::DestVersion),
                            "assetType" | "asset_type" => Ok(GeneratedField::AssetType),
                            "psbts" => Ok(GeneratedField::Psbts),
                            "targetUtxoNum" | "target_utxo_num" => Ok(GeneratedField::TargetUtxoNum),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgTransferVault;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgTransferVault")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgTransferVault, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ =None;
                let mut source_version__ =None;
                let mut dest_version__ =None;
                let mut asset_type__ =None;
                let mut psbts__ =None;
                let mut target_utxo_num__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ =Some(map_.next_value()?);
                        }
                        GeneratedField::SourceVersion => {
                            if source_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceVersion"));
                            }
                            source_version__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DestVersion => {
                            if dest_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destVersion"));
                            }
                            dest_version__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AssetType => {
                            if asset_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetType"));
                            }
                            asset_type__ =Some(map_.next_value::<AssetType>()? as i32);
                        }
                        GeneratedField::Psbts => {
                            if psbts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("psbts"));
                            }
                            psbts__ =Some(map_.next_value()?);
                        }
                        GeneratedField::TargetUtxoNum => {
                            if target_utxo_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetUtxoNum"));
                            }
                            target_utxo_num__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MsgTransferVault {
                    authority: authority__.unwrap_or_default(),
                    source_version: source_version__.unwrap_or_default(),
                    dest_version: dest_version__.unwrap_or_default(),
                    asset_type: asset_type__.unwrap_or_default(),
                    psbts: psbts__.unwrap_or_default(),
                    target_utxo_num: target_utxo_num__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgTransferVault", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgTransferVaultResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgTransferVaultResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgTransferVaultResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MsgTransferVaultResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgTransferVaultResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgTransferVaultResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgTransferVaultResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgTransferVaultResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgUpdateParams", len)?;
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
        const FIELDS: &[&str] = &[
            "authority",
            "params",
        ];

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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
                formatter.write_str("struct side.btcbridge.MsgUpdateParams")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgUpdateParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ =None;
                let mut params__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ =map_.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateParams {
                    authority: authority__.unwrap_or_default(),
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgUpdateParams", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgUpdateParamsResponse", len)?;
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
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
                formatter.write_str("struct side.btcbridge.MsgUpdateParamsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgUpdateParamsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateParamsResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgUpdateParamsResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateTrustedNonBtcRelayers {
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
        if !self.relayers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgUpdateTrustedNonBtcRelayers", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.relayers.is_empty() {
            struct_ser.serialize_field("relayers", &self.relayers)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateTrustedNonBtcRelayers {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "relayers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Relayers,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "relayers" => Ok(GeneratedField::Relayers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateTrustedNonBtcRelayers;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgUpdateTrustedNonBtcRelayers")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgUpdateTrustedNonBtcRelayers, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ =None;
                let mut relayers__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Relayers => {
                            if relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayers"));
                            }
                            relayers__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateTrustedNonBtcRelayers {
                    sender: sender__.unwrap_or_default(),
                    relayers: relayers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgUpdateTrustedNonBtcRelayers", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateTrustedNonBtcRelayersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgUpdateTrustedNonBtcRelayersResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateTrustedNonBtcRelayersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MsgUpdateTrustedNonBtcRelayersResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgUpdateTrustedNonBtcRelayersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgUpdateTrustedNonBtcRelayersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateTrustedNonBtcRelayersResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgUpdateTrustedNonBtcRelayersResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateTrustedOracles {
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
        if !self.oracles.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgUpdateTrustedOracles", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.oracles.is_empty() {
            struct_ser.serialize_field("oracles", &self.oracles)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateTrustedOracles {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "oracles",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
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
            type Value = MsgUpdateTrustedOracles;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgUpdateTrustedOracles")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgUpdateTrustedOracles, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ =None;
                let mut oracles__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Oracles => {
                            if oracles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracles"));
                            }
                            oracles__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateTrustedOracles {
                    sender: sender__.unwrap_or_default(),
                    oracles: oracles__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgUpdateTrustedOracles", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdateTrustedOraclesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgUpdateTrustedOraclesResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdateTrustedOraclesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MsgUpdateTrustedOraclesResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgUpdateTrustedOraclesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgUpdateTrustedOraclesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateTrustedOraclesResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgUpdateTrustedOraclesResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgWithdrawToBitcoin {
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
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.MsgWithdrawToBitcoin", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgWithdrawToBitcoin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "amount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Amount,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sender" => Ok(GeneratedField::Sender),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgWithdrawToBitcoin;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgWithdrawToBitcoin")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgWithdrawToBitcoin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ =None;
                let mut amount__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgWithdrawToBitcoin {
                    sender: sender__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgWithdrawToBitcoin", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgWithdrawToBitcoinResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.MsgWithdrawToBitcoinResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgWithdrawToBitcoinResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MsgWithdrawToBitcoinResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.MsgWithdrawToBitcoinResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgWithdrawToBitcoinResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgWithdrawToBitcoinResponse {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.MsgWithdrawToBitcoinResponse", FIELDS, GeneratedVisitor)
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
        if self.confirmations != 0 {
            len += 1;
        }
        if self.max_acceptable_block_depth != 0 {
            len += 1;
        }
        if !self.btc_voucher_denom.is_empty() {
            len += 1;
        }
        if self.deposit_enabled {
            len += 1;
        }
        if self.withdraw_enabled {
            len += 1;
        }
        if !self.trusted_non_btc_relayers.is_empty() {
            len += 1;
        }
        if !self.trusted_oracles.is_empty() {
            len += 1;
        }
        if self.fee_rate_validity_period != 0 {
            len += 1;
        }
        if !self.vaults.is_empty() {
            len += 1;
        }
        if self.withdraw_params.is_some() {
            len += 1;
        }
        if self.protocol_limits.is_some() {
            len += 1;
        }
        if self.protocol_fees.is_some() {
            len += 1;
        }
        if self.tss_params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.Params", len)?;
        if self.confirmations != 0 {
            struct_ser.serialize_field("confirmations", &self.confirmations)?;
        }
        if self.max_acceptable_block_depth != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxAcceptableBlockDepth", alloc::string::ToString::to_string(&self.max_acceptable_block_depth).as_str())?;
        }
        if !self.btc_voucher_denom.is_empty() {
            struct_ser.serialize_field("btcVoucherDenom", &self.btc_voucher_denom)?;
        }
        if self.deposit_enabled {
            struct_ser.serialize_field("depositEnabled", &self.deposit_enabled)?;
        }
        if self.withdraw_enabled {
            struct_ser.serialize_field("withdrawEnabled", &self.withdraw_enabled)?;
        }
        if !self.trusted_non_btc_relayers.is_empty() {
            struct_ser.serialize_field("trustedNonBtcRelayers", &self.trusted_non_btc_relayers)?;
        }
        if !self.trusted_oracles.is_empty() {
            struct_ser.serialize_field("trustedOracles", &self.trusted_oracles)?;
        }
        if self.fee_rate_validity_period != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("feeRateValidityPeriod", alloc::string::ToString::to_string(&self.fee_rate_validity_period).as_str())?;
        }
        if !self.vaults.is_empty() {
            struct_ser.serialize_field("vaults", &self.vaults)?;
        }
        if let Some(v) = self.withdraw_params.as_ref() {
            struct_ser.serialize_field("withdrawParams", v)?;
        }
        if let Some(v) = self.protocol_limits.as_ref() {
            struct_ser.serialize_field("protocolLimits", v)?;
        }
        if let Some(v) = self.protocol_fees.as_ref() {
            struct_ser.serialize_field("protocolFees", v)?;
        }
        if let Some(v) = self.tss_params.as_ref() {
            struct_ser.serialize_field("tssParams", v)?;
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
            "confirmations",
            "max_acceptable_block_depth",
            "maxAcceptableBlockDepth",
            "btc_voucher_denom",
            "btcVoucherDenom",
            "deposit_enabled",
            "depositEnabled",
            "withdraw_enabled",
            "withdrawEnabled",
            "trusted_non_btc_relayers",
            "trustedNonBtcRelayers",
            "trusted_oracles",
            "trustedOracles",
            "fee_rate_validity_period",
            "feeRateValidityPeriod",
            "vaults",
            "withdraw_params",
            "withdrawParams",
            "protocol_limits",
            "protocolLimits",
            "protocol_fees",
            "protocolFees",
            "tss_params",
            "tssParams",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Confirmations,
            MaxAcceptableBlockDepth,
            BtcVoucherDenom,
            DepositEnabled,
            WithdrawEnabled,
            TrustedNonBtcRelayers,
            TrustedOracles,
            FeeRateValidityPeriod,
            Vaults,
            WithdrawParams,
            ProtocolLimits,
            ProtocolFees,
            TssParams,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "confirmations" => Ok(GeneratedField::Confirmations),
                            "maxAcceptableBlockDepth" | "max_acceptable_block_depth" => Ok(GeneratedField::MaxAcceptableBlockDepth),
                            "btcVoucherDenom" | "btc_voucher_denom" => Ok(GeneratedField::BtcVoucherDenom),
                            "depositEnabled" | "deposit_enabled" => Ok(GeneratedField::DepositEnabled),
                            "withdrawEnabled" | "withdraw_enabled" => Ok(GeneratedField::WithdrawEnabled),
                            "trustedNonBtcRelayers" | "trusted_non_btc_relayers" => Ok(GeneratedField::TrustedNonBtcRelayers),
                            "trustedOracles" | "trusted_oracles" => Ok(GeneratedField::TrustedOracles),
                            "feeRateValidityPeriod" | "fee_rate_validity_period" => Ok(GeneratedField::FeeRateValidityPeriod),
                            "vaults" => Ok(GeneratedField::Vaults),
                            "withdrawParams" | "withdraw_params" => Ok(GeneratedField::WithdrawParams),
                            "protocolLimits" | "protocol_limits" => Ok(GeneratedField::ProtocolLimits),
                            "protocolFees" | "protocol_fees" => Ok(GeneratedField::ProtocolFees),
                            "tssParams" | "tss_params" => Ok(GeneratedField::TssParams),
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
                formatter.write_str("struct side.btcbridge.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Params, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut confirmations__ =None;
                let mut max_acceptable_block_depth__ =None;
                let mut btc_voucher_denom__ =None;
                let mut deposit_enabled__ =None;
                let mut withdraw_enabled__ =None;
                let mut trusted_non_btc_relayers__ =None;
                let mut trusted_oracles__ =None;
                let mut fee_rate_validity_period__ =None;
                let mut vaults__ =None;
                let mut withdraw_params__ =None;
                let mut protocol_limits__ =None;
                let mut protocol_fees__ =None;
                let mut tss_params__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Confirmations => {
                            if confirmations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("confirmations"));
                            }
                            confirmations__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxAcceptableBlockDepth => {
                            if max_acceptable_block_depth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAcceptableBlockDepth"));
                            }
                            max_acceptable_block_depth__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BtcVoucherDenom => {
                            if btc_voucher_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("btcVoucherDenom"));
                            }
                            btc_voucher_denom__ =Some(map_.next_value()?);
                        }
                        GeneratedField::DepositEnabled => {
                            if deposit_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositEnabled"));
                            }
                            deposit_enabled__ =Some(map_.next_value()?);
                        }
                        GeneratedField::WithdrawEnabled => {
                            if withdraw_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withdrawEnabled"));
                            }
                            withdraw_enabled__ =Some(map_.next_value()?);
                        }
                        GeneratedField::TrustedNonBtcRelayers => {
                            if trusted_non_btc_relayers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustedNonBtcRelayers"));
                            }
                            trusted_non_btc_relayers__ =Some(map_.next_value()?);
                        }
                        GeneratedField::TrustedOracles => {
                            if trusted_oracles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustedOracles"));
                            }
                            trusted_oracles__ =Some(map_.next_value()?);
                        }
                        GeneratedField::FeeRateValidityPeriod => {
                            if fee_rate_validity_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeRateValidityPeriod"));
                            }
                            fee_rate_validity_period__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vaults => {
                            if vaults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaults"));
                            }
                            vaults__ =Some(map_.next_value()?);
                        }
                        GeneratedField::WithdrawParams => {
                            if withdraw_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withdrawParams"));
                            }
                            withdraw_params__ =map_.next_value()?;
                        }
                        GeneratedField::ProtocolLimits => {
                            if protocol_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolLimits"));
                            }
                            protocol_limits__ =map_.next_value()?;
                        }
                        GeneratedField::ProtocolFees => {
                            if protocol_fees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolFees"));
                            }
                            protocol_fees__ =map_.next_value()?;
                        }
                        GeneratedField::TssParams => {
                            if tss_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tssParams"));
                            }
                            tss_params__ =map_.next_value()?;
                        }
                    }
                }
                Ok(Params {
                    confirmations: confirmations__.unwrap_or_default(),
                    max_acceptable_block_depth: max_acceptable_block_depth__.unwrap_or_default(),
                    btc_voucher_denom: btc_voucher_denom__.unwrap_or_default(),
                    deposit_enabled: deposit_enabled__.unwrap_or_default(),
                    withdraw_enabled: withdraw_enabled__.unwrap_or_default(),
                    trusted_non_btc_relayers: trusted_non_btc_relayers__.unwrap_or_default(),
                    trusted_oracles: trusted_oracles__.unwrap_or_default(),
                    fee_rate_validity_period: fee_rate_validity_period__.unwrap_or_default(),
                    vaults: vaults__.unwrap_or_default(),
                    withdraw_params: withdraw_params__,
                    protocol_limits: protocol_limits__,
                    protocol_fees: protocol_fees__,
                    tss_params: tss_params__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.Params", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ProtocolFees {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.deposit_fee != 0 {
            len += 1;
        }
        if self.withdraw_fee != 0 {
            len += 1;
        }
        if !self.collector.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.ProtocolFees", len)?;
        if self.deposit_fee != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("depositFee", alloc::string::ToString::to_string(&self.deposit_fee).as_str())?;
        }
        if self.withdraw_fee != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("withdrawFee", alloc::string::ToString::to_string(&self.withdraw_fee).as_str())?;
        }
        if !self.collector.is_empty() {
            struct_ser.serialize_field("collector", &self.collector)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ProtocolFees {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deposit_fee",
            "depositFee",
            "withdraw_fee",
            "withdrawFee",
            "collector",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DepositFee,
            WithdrawFee,
            Collector,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "depositFee" | "deposit_fee" => Ok(GeneratedField::DepositFee),
                            "withdrawFee" | "withdraw_fee" => Ok(GeneratedField::WithdrawFee),
                            "collector" => Ok(GeneratedField::Collector),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProtocolFees;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.ProtocolFees")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<ProtocolFees, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deposit_fee__ =None;
                let mut withdraw_fee__ =None;
                let mut collector__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DepositFee => {
                            if deposit_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositFee"));
                            }
                            deposit_fee__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::WithdrawFee => {
                            if withdraw_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withdrawFee"));
                            }
                            withdraw_fee__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Collector => {
                            if collector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collector"));
                            }
                            collector__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProtocolFees {
                    deposit_fee: deposit_fee__.unwrap_or_default(),
                    withdraw_fee: withdraw_fee__.unwrap_or_default(),
                    collector: collector__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.ProtocolFees", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ProtocolLimits {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.btc_min_deposit != 0 {
            len += 1;
        }
        if self.btc_min_withdraw != 0 {
            len += 1;
        }
        if self.btc_max_withdraw != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.ProtocolLimits", len)?;
        if self.btc_min_deposit != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("btcMinDeposit", alloc::string::ToString::to_string(&self.btc_min_deposit).as_str())?;
        }
        if self.btc_min_withdraw != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("btcMinWithdraw", alloc::string::ToString::to_string(&self.btc_min_withdraw).as_str())?;
        }
        if self.btc_max_withdraw != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("btcMaxWithdraw", alloc::string::ToString::to_string(&self.btc_max_withdraw).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ProtocolLimits {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "btc_min_deposit",
            "btcMinDeposit",
            "btc_min_withdraw",
            "btcMinWithdraw",
            "btc_max_withdraw",
            "btcMaxWithdraw",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BtcMinDeposit,
            BtcMinWithdraw,
            BtcMaxWithdraw,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "btcMinDeposit" | "btc_min_deposit" => Ok(GeneratedField::BtcMinDeposit),
                            "btcMinWithdraw" | "btc_min_withdraw" => Ok(GeneratedField::BtcMinWithdraw),
                            "btcMaxWithdraw" | "btc_max_withdraw" => Ok(GeneratedField::BtcMaxWithdraw),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProtocolLimits;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.ProtocolLimits")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<ProtocolLimits, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut btc_min_deposit__ =None;
                let mut btc_min_withdraw__ =None;
                let mut btc_max_withdraw__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BtcMinDeposit => {
                            if btc_min_deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("btcMinDeposit"));
                            }
                            btc_min_deposit__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BtcMinWithdraw => {
                            if btc_min_withdraw__.is_some() {
                                return Err(serde::de::Error::duplicate_field("btcMinWithdraw"));
                            }
                            btc_min_withdraw__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BtcMaxWithdraw => {
                            if btc_max_withdraw__.is_some() {
                                return Err(serde::de::Error::duplicate_field("btcMaxWithdraw"));
                            }
                            btc_max_withdraw__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ProtocolLimits {
                    btc_min_deposit: btc_min_deposit__.unwrap_or_default(),
                    btc_min_withdraw: btc_min_withdraw__.unwrap_or_default(),
                    btc_max_withdraw: btc_max_withdraw__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.ProtocolLimits", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryAllDkgRequestsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.QueryAllDKGRequestsRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAllDkgRequestsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = QueryAllDkgRequestsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryAllDKGRequestsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryAllDkgRequestsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryAllDkgRequestsRequest {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryAllDKGRequestsRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryAllDkgRequestsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryAllDKGRequestsResponse", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryAllDkgRequestsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requests" => Ok(GeneratedField::Requests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllDkgRequestsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryAllDKGRequestsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryAllDkgRequestsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryAllDkgRequestsResponse {
                    requests: requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryAllDKGRequestsResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBlockHeaderByHashRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryBlockHeaderByHashRequest", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBlockHeaderByHashRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBlockHeaderByHashRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryBlockHeaderByHashRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryBlockHeaderByHashRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBlockHeaderByHashRequest {
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryBlockHeaderByHashRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBlockHeaderByHashResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block_header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryBlockHeaderByHashResponse", len)?;
        if let Some(v) = self.block_header.as_ref() {
            struct_ser.serialize_field("blockHeader", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBlockHeaderByHashResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_header",
            "blockHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockHeader,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockHeader" | "block_header" => Ok(GeneratedField::BlockHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBlockHeaderByHashResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryBlockHeaderByHashResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryBlockHeaderByHashResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_header__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BlockHeader => {
                            if block_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeader"));
                            }
                            block_header__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QueryBlockHeaderByHashResponse {
                    block_header: block_header__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryBlockHeaderByHashResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBlockHeaderByHeightRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryBlockHeaderByHeightRequest", len)?;
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", alloc::string::ToString::to_string(&self.height).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBlockHeaderByHeightRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "height" => Ok(GeneratedField::Height),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBlockHeaderByHeightRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryBlockHeaderByHeightRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryBlockHeaderByHeightRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryBlockHeaderByHeightRequest {
                    height: height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryBlockHeaderByHeightRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryBlockHeaderByHeightResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block_header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryBlockHeaderByHeightResponse", len)?;
        if let Some(v) = self.block_header.as_ref() {
            struct_ser.serialize_field("blockHeader", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryBlockHeaderByHeightResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_header",
            "blockHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockHeader,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockHeader" | "block_header" => Ok(GeneratedField::BlockHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBlockHeaderByHeightResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryBlockHeaderByHeightResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryBlockHeaderByHeightResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_header__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BlockHeader => {
                            if block_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeader"));
                            }
                            block_header__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QueryBlockHeaderByHeightResponse {
                    block_header: block_header__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryBlockHeaderByHeightResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryChainTipRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.QueryChainTipRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryChainTipRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = QueryChainTipRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryChainTipRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryChainTipRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryChainTipRequest {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryChainTipRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryChainTipResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryChainTipResponse", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", alloc::string::ToString::to_string(&self.height).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryChainTipResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "height",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Height,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            "height" => Ok(GeneratedField::Height),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryChainTipResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryChainTipResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryChainTipResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ =None;
                let mut height__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryChainTipResponse {
                    hash: hash__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryChainTipResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDkgCompletionRequestsRequest {
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
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryDKGCompletionRequestsRequest", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDkgCompletionRequestsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
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
        #[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = QueryDkgCompletionRequestsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryDKGCompletionRequestsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryDkgCompletionRequestsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryDkgCompletionRequestsRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryDKGCompletionRequestsRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDkgCompletionRequestsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryDKGCompletionRequestsResponse", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDkgCompletionRequestsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requests" => Ok(GeneratedField::Requests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDkgCompletionRequestsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryDKGCompletionRequestsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryDkgCompletionRequestsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryDkgCompletionRequestsResponse {
                    requests: requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryDKGCompletionRequestsResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDkgRequestRequest {
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
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryDKGRequestRequest", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDkgRequestRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
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
        #[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> core::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = QueryDkgRequestRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryDKGRequestRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryDkgRequestRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryDkgRequestRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryDKGRequestRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDkgRequestResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryDKGRequestResponse", len)?;
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDkgRequestResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "request" => Ok(GeneratedField::Request),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDkgRequestResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryDKGRequestResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryDkgRequestResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QueryDkgRequestResponse {
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryDKGRequestResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDkgRequestsRequest {
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
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryDKGRequestsRequest", len)?;
        if self.status != 0 {
            let v = DkgRequestStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDkgRequestsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
        ];

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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = QueryDkgRequestsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryDKGRequestsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryDkgRequestsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ =Some(map_.next_value::<DkgRequestStatus>()? as i32);
                        }
                    }
                }
                Ok(QueryDkgRequestsRequest {
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryDKGRequestsRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryDkgRequestsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryDKGRequestsResponse", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryDkgRequestsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requests" => Ok(GeneratedField::Requests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDkgRequestsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryDKGRequestsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryDkgRequestsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryDkgRequestsResponse {
                    requests: requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryDKGRequestsResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryFeeRateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.QueryFeeRateRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryFeeRateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = QueryFeeRateRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryFeeRateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryFeeRateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryFeeRateRequest {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryFeeRateRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryFeeRateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fee_rate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryFeeRateResponse", len)?;
        if let Some(v) = self.fee_rate.as_ref() {
            struct_ser.serialize_field("feeRate", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryFeeRateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fee_rate",
            "feeRate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeeRate,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "feeRate" | "fee_rate" => Ok(GeneratedField::FeeRate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryFeeRateResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryFeeRateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryFeeRateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fee_rate__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeeRate => {
                            if fee_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeRate"));
                            }
                            fee_rate__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QueryFeeRateResponse {
                    fee_rate: fee_rate__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryFeeRateResponse", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("side.btcbridge.QueryParamsRequest", len)?;
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
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
                formatter.write_str("struct side.btcbridge.QueryParamsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryParamsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryParamsRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryParamsResponse", len)?;
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
        const FIELDS: &[&str] = &[
            "params",
        ];

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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
                formatter.write_str("struct side.btcbridge.QueryParamsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryParamsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse {
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryParamsResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPendingBtcWithdrawRequestsRequest {
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
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryPendingBtcWithdrawRequestsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPendingBtcWithdrawRequestsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
        ];

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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = QueryPendingBtcWithdrawRequestsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryPendingBtcWithdrawRequestsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryPendingBtcWithdrawRequestsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QueryPendingBtcWithdrawRequestsRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryPendingBtcWithdrawRequestsRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPendingBtcWithdrawRequestsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryPendingBtcWithdrawRequestsResponse", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPendingBtcWithdrawRequestsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requests" => Ok(GeneratedField::Requests),
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
            type Value = QueryPendingBtcWithdrawRequestsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryPendingBtcWithdrawRequestsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryPendingBtcWithdrawRequestsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ =None;
                let mut pagination__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QueryPendingBtcWithdrawRequestsResponse {
                    requests: requests__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryPendingBtcWithdrawRequestsResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QuerySigningRequestByTxHashRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.txid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QuerySigningRequestByTxHashRequest", len)?;
        if !self.txid.is_empty() {
            struct_ser.serialize_field("txid", &self.txid)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QuerySigningRequestByTxHashRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "txid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Txid,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "txid" => Ok(GeneratedField::Txid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySigningRequestByTxHashRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QuerySigningRequestByTxHashRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QuerySigningRequestByTxHashRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut txid__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Txid => {
                            if txid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txid"));
                            }
                            txid__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QuerySigningRequestByTxHashRequest {
                    txid: txid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QuerySigningRequestByTxHashRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QuerySigningRequestByTxHashResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QuerySigningRequestByTxHashResponse", len)?;
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QuerySigningRequestByTxHashResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "request" => Ok(GeneratedField::Request),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySigningRequestByTxHashResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QuerySigningRequestByTxHashResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QuerySigningRequestByTxHashResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QuerySigningRequestByTxHashResponse {
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QuerySigningRequestByTxHashResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QuerySigningRequestsByAddressRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QuerySigningRequestsByAddressRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QuerySigningRequestsByAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
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
            type Value = QuerySigningRequestsByAddressRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QuerySigningRequestsByAddressRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QuerySigningRequestsByAddressRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ =None;
                let mut pagination__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QuerySigningRequestsByAddressRequest {
                    address: address__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QuerySigningRequestsByAddressRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QuerySigningRequestsByAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QuerySigningRequestsByAddressResponse", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QuerySigningRequestsByAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requests" => Ok(GeneratedField::Requests),
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
            type Value = QuerySigningRequestsByAddressResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QuerySigningRequestsByAddressResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QuerySigningRequestsByAddressResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ =None;
                let mut pagination__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QuerySigningRequestsByAddressResponse {
                    requests: requests__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QuerySigningRequestsByAddressResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QuerySigningRequestsRequest {
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
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QuerySigningRequestsRequest", len)?;
        if self.status != 0 {
            let v = SigningStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QuerySigningRequestsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "pagination",
        ];

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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = QuerySigningRequestsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QuerySigningRequestsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QuerySigningRequestsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ =None;
                let mut pagination__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ =Some(map_.next_value::<SigningStatus>()? as i32);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QuerySigningRequestsRequest {
                    status: status__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QuerySigningRequestsRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QuerySigningRequestsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QuerySigningRequestsResponse", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QuerySigningRequestsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requests" => Ok(GeneratedField::Requests),
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
            type Value = QuerySigningRequestsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QuerySigningRequestsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QuerySigningRequestsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ =None;
                let mut pagination__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QuerySigningRequestsResponse {
                    requests: requests__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QuerySigningRequestsResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryUtxoCountAndBalancesByAddressRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryUTXOCountAndBalancesByAddressRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryUtxoCountAndBalancesByAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUtxoCountAndBalancesByAddressRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryUTXOCountAndBalancesByAddressRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryUtxoCountAndBalancesByAddressRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryUtxoCountAndBalancesByAddressRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryUTXOCountAndBalancesByAddressRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryUtxoCountAndBalancesByAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.count != 0 {
            len += 1;
        }
        if self.value != 0 {
            len += 1;
        }
        if !self.rune_balances.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryUTXOCountAndBalancesByAddressResponse", len)?;
        if self.count != 0 {
            struct_ser.serialize_field("count", &self.count)?;
        }
        if self.value != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", alloc::string::ToString::to_string(&self.value).as_str())?;
        }
        if !self.rune_balances.is_empty() {
            struct_ser.serialize_field("runeBalances", &self.rune_balances)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryUtxoCountAndBalancesByAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "count",
            "value",
            "runeBalances",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Count,
            Value,
            RuneBalances,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "count" => Ok(GeneratedField::Count),
                            "value" => Ok(GeneratedField::Value),
                            "runeBalances" => Ok(GeneratedField::RuneBalances),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUtxoCountAndBalancesByAddressResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryUTXOCountAndBalancesByAddressResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryUtxoCountAndBalancesByAddressResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut count__ =None;
                let mut value__ =None;
                let mut rune_balances__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RuneBalances => {
                            if rune_balances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runeBalances"));
                            }
                            rune_balances__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryUtxoCountAndBalancesByAddressResponse {
                    count: count__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    rune_balances: rune_balances__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryUTXOCountAndBalancesByAddressResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryUtxOsByAddressRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryUTXOsByAddressRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryUtxOsByAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUtxOsByAddressRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryUTXOsByAddressRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryUtxOsByAddressRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryUtxOsByAddressRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryUTXOsByAddressRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryUtxOsByAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.utxos.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryUTXOsByAddressResponse", len)?;
        if !self.utxos.is_empty() {
            struct_ser.serialize_field("utxos", &self.utxos)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryUtxOsByAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "utxos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Utxos,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "utxos" => Ok(GeneratedField::Utxos),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUtxOsByAddressResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryUTXOsByAddressResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryUtxOsByAddressResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut utxos__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Utxos => {
                            if utxos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("utxos"));
                            }
                            utxos__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryUtxOsByAddressResponse {
                    utxos: utxos__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryUTXOsByAddressResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryUtxOsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.btcbridge.QueryUTXOsRequest", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryUtxOsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = QueryUtxOsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryUTXOsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryUtxOsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryUtxOsRequest {
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryUTXOsRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryUtxOsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.utxos.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryUTXOsResponse", len)?;
        if !self.utxos.is_empty() {
            struct_ser.serialize_field("utxos", &self.utxos)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryUtxOsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "utxos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Utxos,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "utxos" => Ok(GeneratedField::Utxos),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUtxOsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryUTXOsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryUtxOsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut utxos__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Utxos => {
                            if utxos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("utxos"));
                            }
                            utxos__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryUtxOsResponse {
                    utxos: utxos__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryUTXOsResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryWithdrawRequestsByAddressRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryWithdrawRequestsByAddressRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryWithdrawRequestsByAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
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
            type Value = QueryWithdrawRequestsByAddressRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryWithdrawRequestsByAddressRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryWithdrawRequestsByAddressRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ =None;
                let mut pagination__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QueryWithdrawRequestsByAddressRequest {
                    address: address__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryWithdrawRequestsByAddressRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryWithdrawRequestsByAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryWithdrawRequestsByAddressResponse", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryWithdrawRequestsByAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requests" => Ok(GeneratedField::Requests),
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
            type Value = QueryWithdrawRequestsByAddressResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryWithdrawRequestsByAddressResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryWithdrawRequestsByAddressResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ =None;
                let mut pagination__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ =map_.next_value()?;
                        }
                    }
                }
                Ok(QueryWithdrawRequestsByAddressResponse {
                    requests: requests__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryWithdrawRequestsByAddressResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryWithdrawRequestsByTxHashRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.txid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryWithdrawRequestsByTxHashRequest", len)?;
        if !self.txid.is_empty() {
            struct_ser.serialize_field("txid", &self.txid)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryWithdrawRequestsByTxHashRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "txid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Txid,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "txid" => Ok(GeneratedField::Txid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryWithdrawRequestsByTxHashRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryWithdrawRequestsByTxHashRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryWithdrawRequestsByTxHashRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut txid__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Txid => {
                            if txid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txid"));
                            }
                            txid__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryWithdrawRequestsByTxHashRequest {
                    txid: txid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryWithdrawRequestsByTxHashRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryWithdrawRequestsByTxHashResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryWithdrawRequestsByTxHashResponse", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryWithdrawRequestsByTxHashResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requests" => Ok(GeneratedField::Requests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryWithdrawRequestsByTxHashResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryWithdrawRequestsByTxHashResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryWithdrawRequestsByTxHashResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryWithdrawRequestsByTxHashResponse {
                    requests: requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryWithdrawRequestsByTxHashResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryWithdrawalNetworkFeeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.fee_rate != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryWithdrawalNetworkFeeRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if self.fee_rate != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("feeRate", alloc::string::ToString::to_string(&self.fee_rate).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryWithdrawalNetworkFeeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "amount",
            "fee_rate",
            "feeRate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Amount,
            FeeRate,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            "amount" => Ok(GeneratedField::Amount),
                            "feeRate" | "fee_rate" => Ok(GeneratedField::FeeRate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryWithdrawalNetworkFeeRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryWithdrawalNetworkFeeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryWithdrawalNetworkFeeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ =None;
                let mut amount__ =None;
                let mut fee_rate__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ =Some(map_.next_value()?);
                        }
                        GeneratedField::FeeRate => {
                            if fee_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeRate"));
                            }
                            fee_rate__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QueryWithdrawalNetworkFeeRequest {
                    address: address__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    fee_rate: fee_rate__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryWithdrawalNetworkFeeRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryWithdrawalNetworkFeeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fee_rate != 0 {
            len += 1;
        }
        if !self.fee.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.QueryWithdrawalNetworkFeeResponse", len)?;
        if self.fee_rate != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("feeRate", alloc::string::ToString::to_string(&self.fee_rate).as_str())?;
        }
        if !self.fee.is_empty() {
            struct_ser.serialize_field("fee", &self.fee)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryWithdrawalNetworkFeeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fee_rate",
            "feeRate",
            "fee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeeRate,
            Fee,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "feeRate" | "fee_rate" => Ok(GeneratedField::FeeRate),
                            "fee" => Ok(GeneratedField::Fee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryWithdrawalNetworkFeeResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.QueryWithdrawalNetworkFeeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryWithdrawalNetworkFeeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fee_rate__ =None;
                let mut fee__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeeRate => {
                            if fee_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeRate"));
                            }
                            fee_rate__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryWithdrawalNetworkFeeResponse {
                    fee_rate: fee_rate__.unwrap_or_default(),
                    fee: fee__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.QueryWithdrawalNetworkFeeResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for RuneBalance {
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
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.RuneBalance", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RuneBalance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "amount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Amount,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuneBalance;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.RuneBalance")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<RuneBalance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ =None;
                let mut amount__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RuneBalance {
                    id: id__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.RuneBalance", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for RuneId {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block != 0 {
            len += 1;
        }
        if self.tx != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.RuneId", len)?;
        if self.block != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("block", alloc::string::ToString::to_string(&self.block).as_str())?;
        }
        if self.tx != 0 {
            struct_ser.serialize_field("tx", &self.tx)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RuneId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block",
            "tx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Block,
            Tx,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "block" => Ok(GeneratedField::Block),
                            "tx" => Ok(GeneratedField::Tx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuneId;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.RuneId")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<RuneId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block__ =None;
                let mut tx__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Block => {
                            if block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("block"));
                            }
                            block__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Tx => {
                            if tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tx"));
                            }
                            tx__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RuneId {
                    block: block__.unwrap_or_default(),
                    tx: tx__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.RuneId", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for RunesConsolidation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rune_id.is_empty() {
            len += 1;
        }
        if !self.target_threshold.is_empty() {
            len += 1;
        }
        if self.max_num != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.RunesConsolidation", len)?;
        if !self.rune_id.is_empty() {
            struct_ser.serialize_field("runeId", &self.rune_id)?;
        }
        if !self.target_threshold.is_empty() {
            struct_ser.serialize_field("targetThreshold", &self.target_threshold)?;
        }
        if self.max_num != 0 {
            struct_ser.serialize_field("maxNum", &self.max_num)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RunesConsolidation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rune_id",
            "runeId",
            "target_threshold",
            "targetThreshold",
            "max_num",
            "maxNum",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuneId,
            TargetThreshold,
            MaxNum,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "runeId" | "rune_id" => Ok(GeneratedField::RuneId),
                            "targetThreshold" | "target_threshold" => Ok(GeneratedField::TargetThreshold),
                            "maxNum" | "max_num" => Ok(GeneratedField::MaxNum),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RunesConsolidation;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.RunesConsolidation")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<RunesConsolidation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rune_id__ =None;
                let mut target_threshold__ =None;
                let mut max_num__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuneId => {
                            if rune_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runeId"));
                            }
                            rune_id__ =Some(map_.next_value()?);
                        }
                        GeneratedField::TargetThreshold => {
                            if target_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetThreshold"));
                            }
                            target_threshold__ =Some(map_.next_value()?);
                        }
                        GeneratedField::MaxNum => {
                            if max_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxNum"));
                            }
                            max_num__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RunesConsolidation {
                    rune_id: rune_id__.unwrap_or_default(),
                    target_threshold: target_threshold__.unwrap_or_default(),
                    max_num: max_num__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.RunesConsolidation", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SigningRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.sequence != 0 {
            len += 1;
        }
        if !self.txid.is_empty() {
            len += 1;
        }
        if !self.psbt.is_empty() {
            len += 1;
        }
        if self.creation_time.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.SigningRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.sequence != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sequence", alloc::string::ToString::to_string(&self.sequence).as_str())?;
        }
        if !self.txid.is_empty() {
            struct_ser.serialize_field("txid", &self.txid)?;
        }
        if !self.psbt.is_empty() {
            struct_ser.serialize_field("psbt", &self.psbt)?;
        }
        if let Some(v) = self.creation_time.as_ref() {
            struct_ser.serialize_field("creationTime", v)?;
        }
        if self.status != 0 {
            let v = SigningStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SigningRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "sequence",
            "txid",
            "psbt",
            "creation_time",
            "creationTime",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Sequence,
            Txid,
            Psbt,
            CreationTime,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            "sequence" => Ok(GeneratedField::Sequence),
                            "txid" => Ok(GeneratedField::Txid),
                            "psbt" => Ok(GeneratedField::Psbt),
                            "creationTime" | "creation_time" => Ok(GeneratedField::CreationTime),
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
            type Value = SigningRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.SigningRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<SigningRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ =None;
                let mut sequence__ =None;
                let mut txid__ =None;
                let mut psbt__ =None;
                let mut creation_time__ =None;
                let mut status__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Txid => {
                            if txid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txid"));
                            }
                            txid__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Psbt => {
                            if psbt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("psbt"));
                            }
                            psbt__ =Some(map_.next_value()?);
                        }
                        GeneratedField::CreationTime => {
                            if creation_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationTime"));
                            }
                            creation_time__ =map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ =Some(map_.next_value::<SigningStatus>()? as i32);
                        }
                    }
                }
                Ok(SigningRequest {
                    address: address__.unwrap_or_default(),
                    sequence: sequence__.unwrap_or_default(),
                    txid: txid__.unwrap_or_default(),
                    psbt: psbt__.unwrap_or_default(),
                    creation_time: creation_time__,
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.SigningRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SigningStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SIGNING_STATUS_UNSPECIFIED",
            Self::Pending => "SIGNING_STATUS_PENDING",
            Self::Broadcasted => "SIGNING_STATUS_BROADCASTED",
            Self::Confirmed => "SIGNING_STATUS_CONFIRMED",
            Self::Failed => "SIGNING_STATUS_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SigningStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SIGNING_STATUS_UNSPECIFIED",
            "SIGNING_STATUS_PENDING",
            "SIGNING_STATUS_BROADCASTED",
            "SIGNING_STATUS_CONFIRMED",
            "SIGNING_STATUS_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SigningStatus;

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
                    "SIGNING_STATUS_UNSPECIFIED" => Ok(SigningStatus::Unspecified),
                    "SIGNING_STATUS_PENDING" => Ok(SigningStatus::Pending),
                    "SIGNING_STATUS_BROADCASTED" => Ok(SigningStatus::Broadcasted),
                    "SIGNING_STATUS_CONFIRMED" => Ok(SigningStatus::Confirmed),
                    "SIGNING_STATUS_FAILED" => Ok(SigningStatus::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for TssParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dkg_timeout_period.is_some() {
            len += 1;
        }
        if self.participant_update_transition_period.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.TSSParams", len)?;
        if let Some(v) = self.dkg_timeout_period.as_ref() {
            struct_ser.serialize_field("dkgTimeoutPeriod", v)?;
        }
        if let Some(v) = self.participant_update_transition_period.as_ref() {
            struct_ser.serialize_field("participantUpdateTransitionPeriod", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TssParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dkg_timeout_period",
            "dkgTimeoutPeriod",
            "participant_update_transition_period",
            "participantUpdateTransitionPeriod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DkgTimeoutPeriod,
            ParticipantUpdateTransitionPeriod,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dkgTimeoutPeriod" | "dkg_timeout_period" => Ok(GeneratedField::DkgTimeoutPeriod),
                            "participantUpdateTransitionPeriod" | "participant_update_transition_period" => Ok(GeneratedField::ParticipantUpdateTransitionPeriod),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TssParams;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.TSSParams")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<TssParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dkg_timeout_period__ =None;
                let mut participant_update_transition_period__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DkgTimeoutPeriod => {
                            if dkg_timeout_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dkgTimeoutPeriod"));
                            }
                            dkg_timeout_period__ =map_.next_value()?;
                        }
                        GeneratedField::ParticipantUpdateTransitionPeriod => {
                            if participant_update_transition_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("participantUpdateTransitionPeriod"));
                            }
                            participant_update_transition_period__ =map_.next_value()?;
                        }
                    }
                }
                Ok(TssParams {
                    dkg_timeout_period: dkg_timeout_period__,
                    participant_update_transition_period: participant_update_transition_period__,
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.TSSParams", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Utxo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.txid.is_empty() {
            len += 1;
        }
        if self.vout != 0 {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if self.amount != 0 {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if !self.pub_key_script.is_empty() {
            len += 1;
        }
        if self.is_locked {
            len += 1;
        }
        if !self.runes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.UTXO", len)?;
        if !self.txid.is_empty() {
            struct_ser.serialize_field("txid", &self.txid)?;
        }
        if self.vout != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("vout", alloc::string::ToString::to_string(&self.vout).as_str())?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.amount != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("amount", alloc::string::ToString::to_string(&self.amount).as_str())?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("height", alloc::string::ToString::to_string(&self.height).as_str())?;
        }
        if !self.pub_key_script.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("pubKeyScript", pbjson::private::base64::encode(&self.pub_key_script).as_str())?;
        }
        if self.is_locked {
            struct_ser.serialize_field("isLocked", &self.is_locked)?;
        }
        if !self.runes.is_empty() {
            struct_ser.serialize_field("runes", &self.runes)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Utxo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "txid",
            "vout",
            "address",
            "amount",
            "height",
            "pub_key_script",
            "pubKeyScript",
            "is_locked",
            "isLocked",
            "runes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Txid,
            Vout,
            Address,
            Amount,
            Height,
            PubKeyScript,
            IsLocked,
            Runes,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "txid" => Ok(GeneratedField::Txid),
                            "vout" => Ok(GeneratedField::Vout),
                            "address" => Ok(GeneratedField::Address),
                            "amount" => Ok(GeneratedField::Amount),
                            "height" => Ok(GeneratedField::Height),
                            "pubKeyScript" | "pub_key_script" => Ok(GeneratedField::PubKeyScript),
                            "isLocked" | "is_locked" => Ok(GeneratedField::IsLocked),
                            "runes" => Ok(GeneratedField::Runes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Utxo;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.UTXO")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Utxo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut txid__ =None;
                let mut vout__ =None;
                let mut address__ =None;
                let mut amount__ =None;
                let mut height__ =None;
                let mut pub_key_script__ =None;
                let mut is_locked__ =None;
                let mut runes__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Txid => {
                            if txid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txid"));
                            }
                            txid__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Vout => {
                            if vout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vout"));
                            }
                            vout__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PubKeyScript => {
                            if pub_key_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubKeyScript"));
                            }
                            pub_key_script__ =
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IsLocked => {
                            if is_locked__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isLocked"));
                            }
                            is_locked__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Runes => {
                            if runes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runes"));
                            }
                            runes__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Utxo {
                    txid: txid__.unwrap_or_default(),
                    vout: vout__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    pub_key_script: pub_key_script__.unwrap_or_default(),
                    is_locked: is_locked__.unwrap_or_default(),
                    runes: runes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.UTXO", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Vault {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.pub_key.is_empty() {
            len += 1;
        }
        if self.asset_type != 0 {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.Vault", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.pub_key.is_empty() {
            struct_ser.serialize_field("pubKey", &self.pub_key)?;
        }
        if self.asset_type != 0 {
            let v = AssetType::try_from(self.asset_type)
                .map_err(|_| serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.asset_type)))?;
            struct_ser.serialize_field("assetType", &v)?;
        }
        if self.version != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("version", alloc::string::ToString::to_string(&self.version).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Vault {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "pub_key",
            "pubKey",
            "asset_type",
            "assetType",
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            PubKey,
            AssetType,
            Version,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            "pubKey" | "pub_key" => Ok(GeneratedField::PubKey),
                            "assetType" | "asset_type" => Ok(GeneratedField::AssetType),
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Vault;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.Vault")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Vault, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ =None;
                let mut pub_key__ =None;
                let mut asset_type__ =None;
                let mut version__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ =Some(map_.next_value()?);
                        }
                        GeneratedField::PubKey => {
                            if pub_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubKey"));
                            }
                            pub_key__ =Some(map_.next_value()?);
                        }
                        GeneratedField::AssetType => {
                            if asset_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetType"));
                            }
                            asset_type__ =Some(map_.next_value::<AssetType>()? as i32);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Vault {
                    address: address__.unwrap_or_default(),
                    pub_key: pub_key__.unwrap_or_default(),
                    asset_type: asset_type__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.Vault", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for WithdrawParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_utxo_num != 0 {
            len += 1;
        }
        if self.btc_batch_withdraw_period != 0 {
            len += 1;
        }
        if self.max_btc_batch_withdraw_num != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.WithdrawParams", len)?;
        if self.max_utxo_num != 0 {
            struct_ser.serialize_field("maxUtxoNum", &self.max_utxo_num)?;
        }
        if self.btc_batch_withdraw_period != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("btcBatchWithdrawPeriod", alloc::string::ToString::to_string(&self.btc_batch_withdraw_period).as_str())?;
        }
        if self.max_btc_batch_withdraw_num != 0 {
            struct_ser.serialize_field("maxBtcBatchWithdrawNum", &self.max_btc_batch_withdraw_num)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for WithdrawParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_utxo_num",
            "maxUtxoNum",
            "btc_batch_withdraw_period",
            "btcBatchWithdrawPeriod",
            "max_btc_batch_withdraw_num",
            "maxBtcBatchWithdrawNum",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxUtxoNum,
            BtcBatchWithdrawPeriod,
            MaxBtcBatchWithdrawNum,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxUtxoNum" | "max_utxo_num" => Ok(GeneratedField::MaxUtxoNum),
                            "btcBatchWithdrawPeriod" | "btc_batch_withdraw_period" => Ok(GeneratedField::BtcBatchWithdrawPeriod),
                            "maxBtcBatchWithdrawNum" | "max_btc_batch_withdraw_num" => Ok(GeneratedField::MaxBtcBatchWithdrawNum),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WithdrawParams;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.WithdrawParams")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<WithdrawParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_utxo_num__ =None;
                let mut btc_batch_withdraw_period__ =None;
                let mut max_btc_batch_withdraw_num__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxUtxoNum => {
                            if max_utxo_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUtxoNum"));
                            }
                            max_utxo_num__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BtcBatchWithdrawPeriod => {
                            if btc_batch_withdraw_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("btcBatchWithdrawPeriod"));
                            }
                            btc_batch_withdraw_period__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxBtcBatchWithdrawNum => {
                            if max_btc_batch_withdraw_num__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBtcBatchWithdrawNum"));
                            }
                            max_btc_batch_withdraw_num__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(WithdrawParams {
                    max_utxo_num: max_utxo_num__.unwrap_or_default(),
                    btc_batch_withdraw_period: btc_batch_withdraw_period__.unwrap_or_default(),
                    max_btc_batch_withdraw_num: max_btc_batch_withdraw_num__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.WithdrawParams", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for WithdrawRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.sequence != 0 {
            len += 1;
        }
        if !self.txid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.btcbridge.WithdrawRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if self.sequence != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sequence", alloc::string::ToString::to_string(&self.sequence).as_str())?;
        }
        if !self.txid.is_empty() {
            struct_ser.serialize_field("txid", &self.txid)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for WithdrawRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "amount",
            "sequence",
            "txid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Amount,
            Sequence,
            Txid,
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

                    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> core::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            "amount" => Ok(GeneratedField::Amount),
                            "sequence" => Ok(GeneratedField::Sequence),
                            "txid" => Ok(GeneratedField::Txid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WithdrawRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.btcbridge.WithdrawRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<WithdrawRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ =None;
                let mut amount__ =None;
                let mut sequence__ =None;
                let mut txid__ =None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ =Some(map_.next_value()?);
                        }
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Txid => {
                            if txid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txid"));
                            }
                            txid__ =Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WithdrawRequest {
                    address: address__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    sequence: sequence__.unwrap_or_default(),
                    txid: txid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.btcbridge.WithdrawRequest", FIELDS, GeneratedVisitor)
    }
}