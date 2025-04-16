// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for Cancellation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.loan_id.is_empty() {
            len += 1;
        }
        if !self.txid.is_empty() {
            len += 1;
        }
        if !self.tx.is_empty() {
            len += 1;
        }
        if !self.signatures.is_empty() {
            len += 1;
        }
        if !self.dcm_signatures.is_empty() {
            len += 1;
        }
        if self.create_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.Cancellation", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.txid.is_empty() {
            struct_ser.serialize_field("txid", &self.txid)?;
        }
        if !self.tx.is_empty() {
            struct_ser.serialize_field("tx", &self.tx)?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures)?;
        }
        if !self.dcm_signatures.is_empty() {
            struct_ser.serialize_field("dcmSignatures", &self.dcm_signatures)?;
        }
        if let Some(v) = self.create_at.as_ref() {
            struct_ser.serialize_field("createAt", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Cancellation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "loan_id",
            "loanId",
            "txid",
            "tx",
            "signatures",
            "dcm_signatures",
            "dcmSignatures",
            "create_at",
            "createAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoanId,
            Txid,
            Tx,
            Signatures,
            DcmSignatures,
            CreateAt,
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
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
                            "txid" => Ok(GeneratedField::Txid),
                            "tx" => Ok(GeneratedField::Tx),
                            "signatures" => Ok(GeneratedField::Signatures),
                            "dcmSignatures" | "dcm_signatures" => Ok(GeneratedField::DcmSignatures),
                            "createAt" | "create_at" => Ok(GeneratedField::CreateAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Cancellation;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.Cancellation")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Cancellation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut loan_id__ = None;
                let mut txid__ = None;
                let mut tx__ = None;
                let mut signatures__ = None;
                let mut dcm_signatures__ = None;
                let mut create_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Txid => {
                            if txid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txid"));
                            }
                            txid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tx => {
                            if tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tx"));
                            }
                            tx__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DcmSignatures => {
                            if dcm_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dcmSignatures"));
                            }
                            dcm_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreateAt => {
                            if create_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createAt"));
                            }
                            create_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Cancellation {
                    loan_id: loan_id__.unwrap_or_default(),
                    txid: txid__.unwrap_or_default(),
                    tx: tx__.unwrap_or_default(),
                    signatures: signatures__.unwrap_or_default(),
                    dcm_signatures: dcm_signatures__.unwrap_or_default(),
                    create_at: create_at__,
                })
            }
        }
        deserializer.deserialize_struct("side.lending.Cancellation", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CetInfo {
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
        if self.outcome_index != 0 {
            len += 1;
        }
        if !self.signature_point.is_empty() {
            len += 1;
        }
        if !self.script.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.CetInfo", len)?;
        if self.event_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventId",
                alloc::string::ToString::to_string(&self.event_id).as_str(),
            )?;
        }
        if self.outcome_index != 0 {
            struct_ser.serialize_field("outcomeIndex", &self.outcome_index)?;
        }
        if !self.signature_point.is_empty() {
            struct_ser.serialize_field("signaturePoint", &self.signature_point)?;
        }
        if !self.script.is_empty() {
            struct_ser.serialize_field("script", &self.script)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CetInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_id",
            "eventId",
            "outcome_index",
            "outcomeIndex",
            "signature_point",
            "signaturePoint",
            "script",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventId,
            OutcomeIndex,
            SignaturePoint,
            Script,
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
                            "outcomeIndex" | "outcome_index" => Ok(GeneratedField::OutcomeIndex),
                            "signaturePoint" | "signature_point" => {
                                Ok(GeneratedField::SignaturePoint)
                            }
                            "script" => Ok(GeneratedField::Script),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CetInfo;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.CetInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<CetInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_id__ = None;
                let mut outcome_index__ = None;
                let mut signature_point__ = None;
                let mut script__ = None;
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
                        GeneratedField::OutcomeIndex => {
                            if outcome_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcomeIndex"));
                            }
                            outcome_index__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SignaturePoint => {
                            if signature_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signaturePoint"));
                            }
                            signature_point__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Script => {
                            if script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            script__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CetInfo {
                    event_id: event_id__.unwrap_or_default(),
                    outcome_index: outcome_index__.unwrap_or_default(),
                    signature_point: signature_point__.unwrap_or_default(),
                    script: script__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.CetInfo", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CetType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Liquidation => "LIQUIDATION",
            Self::DefaultLiquidation => "DEFAULT_LIQUIDATION",
            Self::Repayment => "REPAYMENT",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CetType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["LIQUIDATION", "DEFAULT_LIQUIDATION", "REPAYMENT"];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CetType;

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
                    "LIQUIDATION" => Ok(CetType::Liquidation),
                    "DEFAULT_LIQUIDATION" => Ok(CetType::DefaultLiquidation),
                    "REPAYMENT" => Ok(CetType::Repayment),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DlcMeta {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.liquidation_cet.is_some() {
            len += 1;
        }
        if self.default_liquidation_cet.is_some() {
            len += 1;
        }
        if self.repayment_cet.is_some() {
            len += 1;
        }
        if !self.vault_utxos.is_empty() {
            len += 1;
        }
        if !self.internal_key.is_empty() {
            len += 1;
        }
        if !self.multisig_script.is_empty() {
            len += 1;
        }
        if !self.timeout_refund_script.is_empty() {
            len += 1;
        }
        if !self.timeout_refund_tx.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.DLCMeta", len)?;
        if let Some(v) = self.liquidation_cet.as_ref() {
            struct_ser.serialize_field("liquidationCet", v)?;
        }
        if let Some(v) = self.default_liquidation_cet.as_ref() {
            struct_ser.serialize_field("defaultLiquidationCet", v)?;
        }
        if let Some(v) = self.repayment_cet.as_ref() {
            struct_ser.serialize_field("repaymentCet", v)?;
        }
        if !self.vault_utxos.is_empty() {
            struct_ser.serialize_field("vaultUtxos", &self.vault_utxos)?;
        }
        if !self.internal_key.is_empty() {
            struct_ser.serialize_field("internalKey", &self.internal_key)?;
        }
        if !self.multisig_script.is_empty() {
            struct_ser.serialize_field("multisigScript", &self.multisig_script)?;
        }
        if !self.timeout_refund_script.is_empty() {
            struct_ser.serialize_field("timeoutRefundScript", &self.timeout_refund_script)?;
        }
        if !self.timeout_refund_tx.is_empty() {
            struct_ser.serialize_field("timeoutRefundTx", &self.timeout_refund_tx)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DlcMeta {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "liquidation_cet",
            "liquidationCet",
            "default_liquidation_cet",
            "defaultLiquidationCet",
            "repayment_cet",
            "repaymentCet",
            "vault_utxos",
            "vaultUtxos",
            "internal_key",
            "internalKey",
            "multisig_script",
            "multisigScript",
            "timeout_refund_script",
            "timeoutRefundScript",
            "timeout_refund_tx",
            "timeoutRefundTx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LiquidationCet,
            DefaultLiquidationCet,
            RepaymentCet,
            VaultUtxos,
            InternalKey,
            MultisigScript,
            TimeoutRefundScript,
            TimeoutRefundTx,
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
                            "liquidationCet" | "liquidation_cet" => {
                                Ok(GeneratedField::LiquidationCet)
                            }
                            "defaultLiquidationCet" | "default_liquidation_cet" => {
                                Ok(GeneratedField::DefaultLiquidationCet)
                            }
                            "repaymentCet" | "repayment_cet" => Ok(GeneratedField::RepaymentCet),
                            "vaultUtxos" | "vault_utxos" => Ok(GeneratedField::VaultUtxos),
                            "internalKey" | "internal_key" => Ok(GeneratedField::InternalKey),
                            "multisigScript" | "multisig_script" => {
                                Ok(GeneratedField::MultisigScript)
                            }
                            "timeoutRefundScript" | "timeout_refund_script" => {
                                Ok(GeneratedField::TimeoutRefundScript)
                            }
                            "timeoutRefundTx" | "timeout_refund_tx" => {
                                Ok(GeneratedField::TimeoutRefundTx)
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
            type Value = DlcMeta;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.DLCMeta")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<DlcMeta, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut liquidation_cet__ = None;
                let mut default_liquidation_cet__ = None;
                let mut repayment_cet__ = None;
                let mut vault_utxos__ = None;
                let mut internal_key__ = None;
                let mut multisig_script__ = None;
                let mut timeout_refund_script__ = None;
                let mut timeout_refund_tx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LiquidationCet => {
                            if liquidation_cet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidationCet"));
                            }
                            liquidation_cet__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultLiquidationCet => {
                            if default_liquidation_cet__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultLiquidationCet",
                                ));
                            }
                            default_liquidation_cet__ = map_.next_value()?;
                        }
                        GeneratedField::RepaymentCet => {
                            if repayment_cet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repaymentCet"));
                            }
                            repayment_cet__ = map_.next_value()?;
                        }
                        GeneratedField::VaultUtxos => {
                            if vault_utxos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultUtxos"));
                            }
                            vault_utxos__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InternalKey => {
                            if internal_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalKey"));
                            }
                            internal_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MultisigScript => {
                            if multisig_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multisigScript"));
                            }
                            multisig_script__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TimeoutRefundScript => {
                            if timeout_refund_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "timeoutRefundScript",
                                ));
                            }
                            timeout_refund_script__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TimeoutRefundTx => {
                            if timeout_refund_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeoutRefundTx"));
                            }
                            timeout_refund_tx__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DlcMeta {
                    liquidation_cet: liquidation_cet__,
                    default_liquidation_cet: default_liquidation_cet__,
                    repayment_cet: repayment_cet__,
                    vault_utxos: vault_utxos__.unwrap_or_default(),
                    internal_key: internal_key__.unwrap_or_default(),
                    multisig_script: multisig_script__.unwrap_or_default(),
                    timeout_refund_script: timeout_refund_script__.unwrap_or_default(),
                    timeout_refund_tx: timeout_refund_tx__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.DLCMeta", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DepositLog {
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
        if !self.vault_address.is_empty() {
            len += 1;
        }
        if !self.deposit_tx.is_empty() {
            len += 1;
        }
        if self.verified {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.DepositLog", len)?;
        if !self.txid.is_empty() {
            struct_ser.serialize_field("txid", &self.txid)?;
        }
        if !self.vault_address.is_empty() {
            struct_ser.serialize_field("vaultAddress", &self.vault_address)?;
        }
        if !self.deposit_tx.is_empty() {
            struct_ser.serialize_field("depositTx", &self.deposit_tx)?;
        }
        if self.verified {
            struct_ser.serialize_field("verified", &self.verified)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DepositLog {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "txid",
            "vault_address",
            "vaultAddress",
            "deposit_tx",
            "depositTx",
            "verified",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Txid,
            VaultAddress,
            DepositTx,
            Verified,
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
                            "txid" => Ok(GeneratedField::Txid),
                            "vaultAddress" | "vault_address" => Ok(GeneratedField::VaultAddress),
                            "depositTx" | "deposit_tx" => Ok(GeneratedField::DepositTx),
                            "verified" => Ok(GeneratedField::Verified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DepositLog;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.DepositLog")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<DepositLog, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut txid__ = None;
                let mut vault_address__ = None;
                let mut deposit_tx__ = None;
                let mut verified__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Txid => {
                            if txid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txid"));
                            }
                            txid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VaultAddress => {
                            if vault_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultAddress"));
                            }
                            vault_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DepositTx => {
                            if deposit_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositTx"));
                            }
                            deposit_tx__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Verified => {
                            if verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verified"));
                            }
                            verified__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DepositLog {
                    txid: txid__.unwrap_or_default(),
                    vault_address: vault_address__.unwrap_or_default(),
                    deposit_tx: deposit_tx__.unwrap_or_default(),
                    verified: verified__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.DepositLog", FIELDS, GeneratedVisitor)
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
        if !self.pools.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.pools.is_empty() {
            struct_ser.serialize_field("pools", &self.pools)?;
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
        const FIELDS: &[&str] = &["params", "pools"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            Pools,
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
                            "pools" => Ok(GeneratedField::Pools),
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
                formatter.write_str("struct side.lending.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut pools__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::Pools => {
                            if pools__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pools"));
                            }
                            pools__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    pools: pools__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.GenesisState", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for LendingPool {
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
        if self.supply.is_some() {
            len += 1;
        }
        if !self.available_amount.is_empty() {
            len += 1;
        }
        if !self.total_borrowed.is_empty() {
            len += 1;
        }
        if !self.total_reserves.is_empty() {
            len += 1;
        }
        if self.total_stokens.is_some() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.LendingPool", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.supply.as_ref() {
            struct_ser.serialize_field("supply", v)?;
        }
        if !self.available_amount.is_empty() {
            struct_ser.serialize_field("availableAmount", &self.available_amount)?;
        }
        if !self.total_borrowed.is_empty() {
            struct_ser.serialize_field("totalBorrowed", &self.total_borrowed)?;
        }
        if !self.total_reserves.is_empty() {
            struct_ser.serialize_field("totalReserves", &self.total_reserves)?;
        }
        if let Some(v) = self.total_stokens.as_ref() {
            struct_ser.serialize_field("totalStokens", v)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if self.status != 0 {
            let v = PoolStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LendingPool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "supply",
            "available_amount",
            "availableAmount",
            "total_borrowed",
            "totalBorrowed",
            "total_reserves",
            "totalReserves",
            "total_stokens",
            "totalStokens",
            "config",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Supply,
            AvailableAmount,
            TotalBorrowed,
            TotalReserves,
            TotalStokens,
            Config,
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
                            "supply" => Ok(GeneratedField::Supply),
                            "availableAmount" | "available_amount" => {
                                Ok(GeneratedField::AvailableAmount)
                            }
                            "totalBorrowed" | "total_borrowed" => Ok(GeneratedField::TotalBorrowed),
                            "totalReserves" | "total_reserves" => Ok(GeneratedField::TotalReserves),
                            "totalStokens" | "total_stokens" => Ok(GeneratedField::TotalStokens),
                            "config" => Ok(GeneratedField::Config),
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
            type Value = LendingPool;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.LendingPool")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<LendingPool, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut supply__ = None;
                let mut available_amount__ = None;
                let mut total_borrowed__ = None;
                let mut total_reserves__ = None;
                let mut total_stokens__ = None;
                let mut config__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Supply => {
                            if supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supply"));
                            }
                            supply__ = map_.next_value()?;
                        }
                        GeneratedField::AvailableAmount => {
                            if available_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("availableAmount"));
                            }
                            available_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalBorrowed => {
                            if total_borrowed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalBorrowed"));
                            }
                            total_borrowed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalReserves => {
                            if total_reserves__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalReserves"));
                            }
                            total_reserves__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalStokens => {
                            if total_stokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalStokens"));
                            }
                            total_stokens__ = map_.next_value()?;
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<PoolStatus>()? as i32);
                        }
                    }
                }
                Ok(LendingPool {
                    id: id__.unwrap_or_default(),
                    supply: supply__,
                    available_amount: available_amount__.unwrap_or_default(),
                    total_borrowed: total_borrowed__.unwrap_or_default(),
                    total_reserves: total_reserves__.unwrap_or_default(),
                    total_stokens: total_stokens__,
                    config: config__,
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.LendingPool", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for LiquidationCet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tx.is_empty() {
            len += 1;
        }
        if !self.borrower_adaptor_signatures.is_empty() {
            len += 1;
        }
        if !self.borrower_adapted_signatures.is_empty() {
            len += 1;
        }
        if !self.dcm_signatures.is_empty() {
            len += 1;
        }
        if !self.signed_tx_hex.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.LiquidationCet", len)?;
        if !self.tx.is_empty() {
            struct_ser.serialize_field("tx", &self.tx)?;
        }
        if !self.borrower_adaptor_signatures.is_empty() {
            struct_ser.serialize_field(
                "borrowerAdaptorSignatures",
                &self.borrower_adaptor_signatures,
            )?;
        }
        if !self.borrower_adapted_signatures.is_empty() {
            struct_ser.serialize_field(
                "borrowerAdaptedSignatures",
                &self.borrower_adapted_signatures,
            )?;
        }
        if !self.dcm_signatures.is_empty() {
            struct_ser.serialize_field("dcmSignatures", &self.dcm_signatures)?;
        }
        if !self.signed_tx_hex.is_empty() {
            struct_ser.serialize_field("signedTxHex", &self.signed_tx_hex)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LiquidationCet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tx",
            "borrower_adaptor_signatures",
            "borrowerAdaptorSignatures",
            "borrower_adapted_signatures",
            "borrowerAdaptedSignatures",
            "dcm_signatures",
            "dcmSignatures",
            "signed_tx_hex",
            "signedTxHex",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tx,
            BorrowerAdaptorSignatures,
            BorrowerAdaptedSignatures,
            DcmSignatures,
            SignedTxHex,
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
                            "tx" => Ok(GeneratedField::Tx),
                            "borrowerAdaptorSignatures" | "borrower_adaptor_signatures" => {
                                Ok(GeneratedField::BorrowerAdaptorSignatures)
                            }
                            "borrowerAdaptedSignatures" | "borrower_adapted_signatures" => {
                                Ok(GeneratedField::BorrowerAdaptedSignatures)
                            }
                            "dcmSignatures" | "dcm_signatures" => Ok(GeneratedField::DcmSignatures),
                            "signedTxHex" | "signed_tx_hex" => Ok(GeneratedField::SignedTxHex),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LiquidationCet;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.LiquidationCet")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<LiquidationCet, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut tx__ = None;
                let mut borrower_adaptor_signatures__ = None;
                let mut borrower_adapted_signatures__ = None;
                let mut dcm_signatures__ = None;
                let mut signed_tx_hex__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tx => {
                            if tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tx"));
                            }
                            tx__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowerAdaptorSignatures => {
                            if borrower_adaptor_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "borrowerAdaptorSignatures",
                                ));
                            }
                            borrower_adaptor_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowerAdaptedSignatures => {
                            if borrower_adapted_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "borrowerAdaptedSignatures",
                                ));
                            }
                            borrower_adapted_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DcmSignatures => {
                            if dcm_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dcmSignatures"));
                            }
                            dcm_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SignedTxHex => {
                            if signed_tx_hex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signedTxHex"));
                            }
                            signed_tx_hex__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LiquidationCet {
                    tx: tx__.unwrap_or_default(),
                    borrower_adaptor_signatures: borrower_adaptor_signatures__.unwrap_or_default(),
                    borrower_adapted_signatures: borrower_adapted_signatures__.unwrap_or_default(),
                    dcm_signatures: dcm_signatures__.unwrap_or_default(),
                    signed_tx_hex: signed_tx_hex__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.LiquidationCet", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Loan {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vault_address.is_empty() {
            len += 1;
        }
        if !self.borrower.is_empty() {
            len += 1;
        }
        if !self.borrower_pub_key.is_empty() {
            len += 1;
        }
        if !self.dcm.is_empty() {
            len += 1;
        }
        if self.maturity_time != 0 {
            len += 1;
        }
        if self.final_timeout != 0 {
            len += 1;
        }
        if !self.pool_id.is_empty() {
            len += 1;
        }
        if self.borrow_amount.is_some() {
            len += 1;
        }
        if !self.origination_fee.is_empty() {
            len += 1;
        }
        if !self.interest.is_empty() {
            len += 1;
        }
        if !self.protocol_fee.is_empty() {
            len += 1;
        }
        if self.term != 0 {
            len += 1;
        }
        if !self.liquidation_price.is_empty() {
            len += 1;
        }
        if self.liquidation_event_id != 0 {
            len += 1;
        }
        if self.default_liquidation_event_id != 0 {
            len += 1;
        }
        if self.repayment_event_id != 0 {
            len += 1;
        }
        if !self.deposit_txs.is_empty() {
            len += 1;
        }
        if !self.collateral_amount.is_empty() {
            len += 1;
        }
        if self.liquidation_id != 0 {
            len += 1;
        }
        if self.create_at.is_some() {
            len += 1;
        }
        if self.disburse_at.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.Loan", len)?;
        if !self.vault_address.is_empty() {
            struct_ser.serialize_field("vaultAddress", &self.vault_address)?;
        }
        if !self.borrower.is_empty() {
            struct_ser.serialize_field("borrower", &self.borrower)?;
        }
        if !self.borrower_pub_key.is_empty() {
            struct_ser.serialize_field("borrowerPubKey", &self.borrower_pub_key)?;
        }
        if !self.dcm.is_empty() {
            struct_ser.serialize_field("dcm", &self.dcm)?;
        }
        if self.maturity_time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "maturityTime",
                alloc::string::ToString::to_string(&self.maturity_time).as_str(),
            )?;
        }
        if self.final_timeout != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "finalTimeout",
                alloc::string::ToString::to_string(&self.final_timeout).as_str(),
            )?;
        }
        if !self.pool_id.is_empty() {
            struct_ser.serialize_field("poolId", &self.pool_id)?;
        }
        if let Some(v) = self.borrow_amount.as_ref() {
            struct_ser.serialize_field("borrowAmount", v)?;
        }
        if !self.origination_fee.is_empty() {
            struct_ser.serialize_field("originationFee", &self.origination_fee)?;
        }
        if !self.interest.is_empty() {
            struct_ser.serialize_field("interest", &self.interest)?;
        }
        if !self.protocol_fee.is_empty() {
            struct_ser.serialize_field("protocolFee", &self.protocol_fee)?;
        }
        if self.term != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "term",
                alloc::string::ToString::to_string(&self.term).as_str(),
            )?;
        }
        if !self.liquidation_price.is_empty() {
            struct_ser.serialize_field("liquidationPrice", &self.liquidation_price)?;
        }
        if self.liquidation_event_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "liquidationEventId",
                alloc::string::ToString::to_string(&self.liquidation_event_id).as_str(),
            )?;
        }
        if self.default_liquidation_event_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "defaultLiquidationEventId",
                alloc::string::ToString::to_string(&self.default_liquidation_event_id).as_str(),
            )?;
        }
        if self.repayment_event_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "repaymentEventId",
                alloc::string::ToString::to_string(&self.repayment_event_id).as_str(),
            )?;
        }
        if !self.deposit_txs.is_empty() {
            struct_ser.serialize_field("depositTxs", &self.deposit_txs)?;
        }
        if !self.collateral_amount.is_empty() {
            struct_ser.serialize_field("collateralAmount", &self.collateral_amount)?;
        }
        if self.liquidation_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "liquidationId",
                alloc::string::ToString::to_string(&self.liquidation_id).as_str(),
            )?;
        }
        if let Some(v) = self.create_at.as_ref() {
            struct_ser.serialize_field("createAt", v)?;
        }
        if let Some(v) = self.disburse_at.as_ref() {
            struct_ser.serialize_field("disburseAt", v)?;
        }
        if self.status != 0 {
            let v = LoanStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Loan {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vault_address",
            "vaultAddress",
            "borrower",
            "borrowerPubKey",
            "dcm",
            "maturity_time",
            "maturityTime",
            "final_timeout",
            "finalTimeout",
            "pool_id",
            "poolId",
            "borrow_amount",
            "borrowAmount",
            "origination_fee",
            "originationFee",
            "interest",
            "protocol_fee",
            "protocolFee",
            "term",
            "liquidation_price",
            "liquidationPrice",
            "liquidation_event_id",
            "liquidationEventId",
            "default_liquidation_event_id",
            "defaultLiquidationEventId",
            "repayment_event_id",
            "repaymentEventId",
            "deposit_txs",
            "depositTxs",
            "collateral_amount",
            "collateralAmount",
            "liquidation_id",
            "liquidationId",
            "create_at",
            "createAt",
            "disburse_at",
            "disburseAt",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VaultAddress,
            Borrower,
            BorrowerPubKey,
            Dcm,
            MaturityTime,
            FinalTimeout,
            PoolId,
            BorrowAmount,
            OriginationFee,
            Interest,
            ProtocolFee,
            Term,
            LiquidationPrice,
            LiquidationEventId,
            DefaultLiquidationEventId,
            RepaymentEventId,
            DepositTxs,
            CollateralAmount,
            LiquidationId,
            CreateAt,
            DisburseAt,
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
                            "vaultAddress" | "vault_address" => Ok(GeneratedField::VaultAddress),
                            "borrower" => Ok(GeneratedField::Borrower),
                            "borrowerPubKey" => Ok(GeneratedField::BorrowerPubKey),
                            "dcm" => Ok(GeneratedField::Dcm),
                            "maturityTime" | "maturity_time" => Ok(GeneratedField::MaturityTime),
                            "finalTimeout" | "final_timeout" => Ok(GeneratedField::FinalTimeout),
                            "poolId" | "pool_id" => Ok(GeneratedField::PoolId),
                            "borrowAmount" | "borrow_amount" => Ok(GeneratedField::BorrowAmount),
                            "originationFee" | "origination_fee" => {
                                Ok(GeneratedField::OriginationFee)
                            }
                            "interest" => Ok(GeneratedField::Interest),
                            "protocolFee" | "protocol_fee" => Ok(GeneratedField::ProtocolFee),
                            "term" => Ok(GeneratedField::Term),
                            "liquidationPrice" | "liquidation_price" => {
                                Ok(GeneratedField::LiquidationPrice)
                            }
                            "liquidationEventId" | "liquidation_event_id" => {
                                Ok(GeneratedField::LiquidationEventId)
                            }
                            "defaultLiquidationEventId" | "default_liquidation_event_id" => {
                                Ok(GeneratedField::DefaultLiquidationEventId)
                            }
                            "repaymentEventId" | "repayment_event_id" => {
                                Ok(GeneratedField::RepaymentEventId)
                            }
                            "depositTxs" | "deposit_txs" => Ok(GeneratedField::DepositTxs),
                            "collateralAmount" | "collateral_amount" => {
                                Ok(GeneratedField::CollateralAmount)
                            }
                            "liquidationId" | "liquidation_id" => Ok(GeneratedField::LiquidationId),
                            "createAt" | "create_at" => Ok(GeneratedField::CreateAt),
                            "disburseAt" | "disburse_at" => Ok(GeneratedField::DisburseAt),
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
            type Value = Loan;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.Loan")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Loan, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut vault_address__ = None;
                let mut borrower__ = None;
                let mut borrower_pub_key__ = None;
                let mut dcm__ = None;
                let mut maturity_time__ = None;
                let mut final_timeout__ = None;
                let mut pool_id__ = None;
                let mut borrow_amount__ = None;
                let mut origination_fee__ = None;
                let mut interest__ = None;
                let mut protocol_fee__ = None;
                let mut term__ = None;
                let mut liquidation_price__ = None;
                let mut liquidation_event_id__ = None;
                let mut default_liquidation_event_id__ = None;
                let mut repayment_event_id__ = None;
                let mut deposit_txs__ = None;
                let mut collateral_amount__ = None;
                let mut liquidation_id__ = None;
                let mut create_at__ = None;
                let mut disburse_at__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VaultAddress => {
                            if vault_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultAddress"));
                            }
                            vault_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Borrower => {
                            if borrower__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrower"));
                            }
                            borrower__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowerPubKey => {
                            if borrower_pub_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowerPubKey"));
                            }
                            borrower_pub_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Dcm => {
                            if dcm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dcm"));
                            }
                            dcm__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaturityTime => {
                            if maturity_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maturityTime"));
                            }
                            maturity_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::FinalTimeout => {
                            if final_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finalTimeout"));
                            }
                            final_timeout__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PoolId => {
                            if pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolId"));
                            }
                            pool_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowAmount => {
                            if borrow_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowAmount"));
                            }
                            borrow_amount__ = map_.next_value()?;
                        }
                        GeneratedField::OriginationFee => {
                            if origination_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originationFee"));
                            }
                            origination_fee__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Interest => {
                            if interest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interest"));
                            }
                            interest__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProtocolFee => {
                            if protocol_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolFee"));
                            }
                            protocol_fee__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Term => {
                            if term__.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            term__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LiquidationPrice => {
                            if liquidation_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidationPrice"));
                            }
                            liquidation_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LiquidationEventId => {
                            if liquidation_event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationEventId",
                                ));
                            }
                            liquidation_event_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DefaultLiquidationEventId => {
                            if default_liquidation_event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultLiquidationEventId",
                                ));
                            }
                            default_liquidation_event_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::RepaymentEventId => {
                            if repayment_event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repaymentEventId"));
                            }
                            repayment_event_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::DepositTxs => {
                            if deposit_txs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositTxs"));
                            }
                            deposit_txs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CollateralAmount => {
                            if collateral_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collateralAmount"));
                            }
                            collateral_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LiquidationId => {
                            if liquidation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidationId"));
                            }
                            liquidation_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CreateAt => {
                            if create_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createAt"));
                            }
                            create_at__ = map_.next_value()?;
                        }
                        GeneratedField::DisburseAt => {
                            if disburse_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disburseAt"));
                            }
                            disburse_at__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<LoanStatus>()? as i32);
                        }
                    }
                }
                Ok(Loan {
                    vault_address: vault_address__.unwrap_or_default(),
                    borrower: borrower__.unwrap_or_default(),
                    borrower_pub_key: borrower_pub_key__.unwrap_or_default(),
                    dcm: dcm__.unwrap_or_default(),
                    maturity_time: maturity_time__.unwrap_or_default(),
                    final_timeout: final_timeout__.unwrap_or_default(),
                    pool_id: pool_id__.unwrap_or_default(),
                    borrow_amount: borrow_amount__,
                    origination_fee: origination_fee__.unwrap_or_default(),
                    interest: interest__.unwrap_or_default(),
                    protocol_fee: protocol_fee__.unwrap_or_default(),
                    term: term__.unwrap_or_default(),
                    liquidation_price: liquidation_price__.unwrap_or_default(),
                    liquidation_event_id: liquidation_event_id__.unwrap_or_default(),
                    default_liquidation_event_id: default_liquidation_event_id__
                        .unwrap_or_default(),
                    repayment_event_id: repayment_event_id__.unwrap_or_default(),
                    deposit_txs: deposit_txs__.unwrap_or_default(),
                    collateral_amount: collateral_amount__.unwrap_or_default(),
                    liquidation_id: liquidation_id__.unwrap_or_default(),
                    create_at: create_at__,
                    disburse_at: disburse_at__,
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.Loan", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for LoanStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "Unspecified",
            Self::Requested => "Requested",
            Self::Open => "Open",
            Self::Rejected => "Rejected",
            Self::Cancelled => "Cancelled",
            Self::Repaid => "Repaid",
            Self::Defaulted => "Defaulted",
            Self::Liquidated => "Liquidated",
            Self::Closed => "Closed",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LoanStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Unspecified",
            "Requested",
            "Open",
            "Rejected",
            "Cancelled",
            "Repaid",
            "Defaulted",
            "Liquidated",
            "Closed",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoanStatus;

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
                    "Unspecified" => Ok(LoanStatus::Unspecified),
                    "Requested" => Ok(LoanStatus::Requested),
                    "Open" => Ok(LoanStatus::Open),
                    "Rejected" => Ok(LoanStatus::Rejected),
                    "Cancelled" => Ok(LoanStatus::Cancelled),
                    "Repaid" => Ok(LoanStatus::Repaid),
                    "Defaulted" => Ok(LoanStatus::Defaulted),
                    "Liquidated" => Ok(LoanStatus::Liquidated),
                    "Closed" => Ok(LoanStatus::Closed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgAddLiquidity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.lender.is_empty() {
            len += 1;
        }
        if !self.pool_id.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgAddLiquidity", len)?;
        if !self.lender.is_empty() {
            struct_ser.serialize_field("lender", &self.lender)?;
        }
        if !self.pool_id.is_empty() {
            struct_ser.serialize_field("poolId", &self.pool_id)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgAddLiquidity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["lender", "pool_id", "poolId", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Lender,
            PoolId,
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
                            "lender" => Ok(GeneratedField::Lender),
                            "poolId" | "pool_id" => Ok(GeneratedField::PoolId),
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
            type Value = MsgAddLiquidity;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgAddLiquidity")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgAddLiquidity, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut lender__ = None;
                let mut pool_id__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Lender => {
                            if lender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lender"));
                            }
                            lender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PoolId => {
                            if pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolId"));
                            }
                            pool_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgAddLiquidity {
                    lender: lender__.unwrap_or_default(),
                    pool_id: pool_id__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer.deserialize_struct("side.lending.MsgAddLiquidity", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgAddLiquidityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("side.lending.MsgAddLiquidityResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgAddLiquidityResponse {
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
            type Value = MsgAddLiquidityResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgAddLiquidityResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgAddLiquidityResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddLiquidityResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgAddLiquidityResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgApply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.borrower.is_empty() {
            len += 1;
        }
        if !self.borrower_pubkey.is_empty() {
            len += 1;
        }
        if self.maturity_time != 0 {
            len += 1;
        }
        if !self.pool_id.is_empty() {
            len += 1;
        }
        if self.borrow_amount.is_some() {
            len += 1;
        }
        if self.dcm_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgApply", len)?;
        if !self.borrower.is_empty() {
            struct_ser.serialize_field("borrower", &self.borrower)?;
        }
        if !self.borrower_pubkey.is_empty() {
            struct_ser.serialize_field("borrowerPubkey", &self.borrower_pubkey)?;
        }
        if self.maturity_time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "maturityTime",
                alloc::string::ToString::to_string(&self.maturity_time).as_str(),
            )?;
        }
        if !self.pool_id.is_empty() {
            struct_ser.serialize_field("poolId", &self.pool_id)?;
        }
        if let Some(v) = self.borrow_amount.as_ref() {
            struct_ser.serialize_field("borrowAmount", v)?;
        }
        if self.dcm_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "dcmId",
                alloc::string::ToString::to_string(&self.dcm_id).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgApply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "borrower",
            "borrower_pubkey",
            "borrowerPubkey",
            "maturity_time",
            "maturityTime",
            "pool_id",
            "poolId",
            "borrow_amount",
            "borrowAmount",
            "dcm_id",
            "dcmId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Borrower,
            BorrowerPubkey,
            MaturityTime,
            PoolId,
            BorrowAmount,
            DcmId,
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
                            "borrower" => Ok(GeneratedField::Borrower),
                            "borrowerPubkey" | "borrower_pubkey" => {
                                Ok(GeneratedField::BorrowerPubkey)
                            }
                            "maturityTime" | "maturity_time" => Ok(GeneratedField::MaturityTime),
                            "poolId" | "pool_id" => Ok(GeneratedField::PoolId),
                            "borrowAmount" | "borrow_amount" => Ok(GeneratedField::BorrowAmount),
                            "dcmId" | "dcm_id" => Ok(GeneratedField::DcmId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgApply;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgApply")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgApply, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut borrower__ = None;
                let mut borrower_pubkey__ = None;
                let mut maturity_time__ = None;
                let mut pool_id__ = None;
                let mut borrow_amount__ = None;
                let mut dcm_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Borrower => {
                            if borrower__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrower"));
                            }
                            borrower__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowerPubkey => {
                            if borrower_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowerPubkey"));
                            }
                            borrower_pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaturityTime => {
                            if maturity_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maturityTime"));
                            }
                            maturity_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::PoolId => {
                            if pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolId"));
                            }
                            pool_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowAmount => {
                            if borrow_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowAmount"));
                            }
                            borrow_amount__ = map_.next_value()?;
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
                    }
                }
                Ok(MsgApply {
                    borrower: borrower__.unwrap_or_default(),
                    borrower_pubkey: borrower_pubkey__.unwrap_or_default(),
                    maturity_time: maturity_time__.unwrap_or_default(),
                    pool_id: pool_id__.unwrap_or_default(),
                    borrow_amount: borrow_amount__,
                    dcm_id: dcm_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.MsgApply", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgApplyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.lending.MsgApplyResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgApplyResponse {
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
            type Value = MsgApplyResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgApplyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgApplyResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgApplyResponse {})
            }
        }
        deserializer.deserialize_struct("side.lending.MsgApplyResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgApprove {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relayer.is_empty() {
            len += 1;
        }
        if !self.vault.is_empty() {
            len += 1;
        }
        if !self.deposit_tx.is_empty() {
            len += 1;
        }
        if !self.block_hash.is_empty() {
            len += 1;
        }
        if !self.proof.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgApprove", len)?;
        if !self.relayer.is_empty() {
            struct_ser.serialize_field("relayer", &self.relayer)?;
        }
        if !self.vault.is_empty() {
            struct_ser.serialize_field("vault", &self.vault)?;
        }
        if !self.deposit_tx.is_empty() {
            struct_ser.serialize_field("depositTx", &self.deposit_tx)?;
        }
        if !self.block_hash.is_empty() {
            struct_ser.serialize_field("blockHash", &self.block_hash)?;
        }
        if !self.proof.is_empty() {
            struct_ser.serialize_field("proof", &self.proof)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgApprove {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relayer",
            "vault",
            "deposit_tx",
            "depositTx",
            "block_hash",
            "blockHash",
            "proof",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relayer,
            Vault,
            DepositTx,
            BlockHash,
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
                            "relayer" => Ok(GeneratedField::Relayer),
                            "vault" => Ok(GeneratedField::Vault),
                            "depositTx" | "deposit_tx" => Ok(GeneratedField::DepositTx),
                            "blockHash" | "block_hash" => Ok(GeneratedField::BlockHash),
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
            type Value = MsgApprove;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgApprove")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgApprove, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut relayer__ = None;
                let mut vault__ = None;
                let mut deposit_tx__ = None;
                let mut block_hash__ = None;
                let mut proof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Relayer => {
                            if relayer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayer"));
                            }
                            relayer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Vault => {
                            if vault__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vault"));
                            }
                            vault__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DepositTx => {
                            if deposit_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositTx"));
                            }
                            deposit_tx__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BlockHash => {
                            if block_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHash"));
                            }
                            block_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Proof => {
                            if proof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proof"));
                            }
                            proof__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgApprove {
                    relayer: relayer__.unwrap_or_default(),
                    vault: vault__.unwrap_or_default(),
                    deposit_tx: deposit_tx__.unwrap_or_default(),
                    block_hash: block_hash__.unwrap_or_default(),
                    proof: proof__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.MsgApprove", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgApproveResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.lending.MsgApproveResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgApproveResponse {
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
            type Value = MsgApproveResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgApproveResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgApproveResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgApproveResponse {})
            }
        }
        deserializer.deserialize_struct("side.lending.MsgApproveResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCancel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.borrower.is_empty() {
            len += 1;
        }
        if !self.loan_id.is_empty() {
            len += 1;
        }
        if !self.tx.is_empty() {
            len += 1;
        }
        if !self.signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgCancel", len)?;
        if !self.borrower.is_empty() {
            struct_ser.serialize_field("borrower", &self.borrower)?;
        }
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.tx.is_empty() {
            struct_ser.serialize_field("tx", &self.tx)?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCancel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["borrower", "loan_id", "loanId", "tx", "signatures"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Borrower,
            LoanId,
            Tx,
            Signatures,
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
                            "borrower" => Ok(GeneratedField::Borrower),
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
                            "tx" => Ok(GeneratedField::Tx),
                            "signatures" => Ok(GeneratedField::Signatures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCancel;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgCancel")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgCancel, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut borrower__ = None;
                let mut loan_id__ = None;
                let mut tx__ = None;
                let mut signatures__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Borrower => {
                            if borrower__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrower"));
                            }
                            borrower__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tx => {
                            if tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tx"));
                            }
                            tx__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgCancel {
                    borrower: borrower__.unwrap_or_default(),
                    loan_id: loan_id__.unwrap_or_default(),
                    tx: tx__.unwrap_or_default(),
                    signatures: signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.MsgCancel", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCancelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.lending.MsgCancelResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCancelResponse {
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
            type Value = MsgCancelResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgCancelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgCancelResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCancelResponse {})
            }
        }
        deserializer.deserialize_struct("side.lending.MsgCancelResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCreatePool {
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
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.lending_asset.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgCreatePool", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.lending_asset.is_empty() {
            struct_ser.serialize_field("lendingAsset", &self.lending_asset)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCreatePool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "id", "lending_asset", "lendingAsset", "config"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Id,
            LendingAsset,
            Config,
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
                            "id" => Ok(GeneratedField::Id),
                            "lendingAsset" | "lending_asset" => Ok(GeneratedField::LendingAsset),
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreatePool;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgCreatePool")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgCreatePool, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut id__ = None;
                let mut lending_asset__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LendingAsset => {
                            if lending_asset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lendingAsset"));
                            }
                            lending_asset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgCreatePool {
                    authority: authority__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    lending_asset: lending_asset__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("side.lending.MsgCreatePool", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCreatePoolResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.lending.MsgCreatePoolResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCreatePoolResponse {
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
            type Value = MsgCreatePoolResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgCreatePoolResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgCreatePoolResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCreatePoolResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgCreatePoolResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRemoveLiquidity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.lender.is_empty() {
            len += 1;
        }
        if self.stokens.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgRemoveLiquidity", len)?;
        if !self.lender.is_empty() {
            struct_ser.serialize_field("lender", &self.lender)?;
        }
        if let Some(v) = self.stokens.as_ref() {
            struct_ser.serialize_field("stokens", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRemoveLiquidity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["lender", "stokens"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Lender,
            Stokens,
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
                            "lender" => Ok(GeneratedField::Lender),
                            "stokens" => Ok(GeneratedField::Stokens),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveLiquidity;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgRemoveLiquidity")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgRemoveLiquidity, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut lender__ = None;
                let mut stokens__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Lender => {
                            if lender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lender"));
                            }
                            lender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Stokens => {
                            if stokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stokens"));
                            }
                            stokens__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgRemoveLiquidity {
                    lender: lender__.unwrap_or_default(),
                    stokens: stokens__,
                })
            }
        }
        deserializer.deserialize_struct("side.lending.MsgRemoveLiquidity", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRemoveLiquidityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("side.lending.MsgRemoveLiquidityResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRemoveLiquidityResponse {
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
            type Value = MsgRemoveLiquidityResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgRemoveLiquidityResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgRemoveLiquidityResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveLiquidityResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgRemoveLiquidityResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRepay {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.borrower.is_empty() {
            len += 1;
        }
        if !self.loan_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgRepay", len)?;
        if !self.borrower.is_empty() {
            struct_ser.serialize_field("borrower", &self.borrower)?;
        }
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRepay {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["borrower", "loan_id", "loanId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Borrower,
            LoanId,
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
                            "borrower" => Ok(GeneratedField::Borrower),
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRepay;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgRepay")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgRepay, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut borrower__ = None;
                let mut loan_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Borrower => {
                            if borrower__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrower"));
                            }
                            borrower__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRepay {
                    borrower: borrower__.unwrap_or_default(),
                    loan_id: loan_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.MsgRepay", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRepayResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.lending.MsgRepayResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRepayResponse {
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
            type Value = MsgRepayResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgRepayResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgRepayResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRepayResponse {})
            }
        }
        deserializer.deserialize_struct("side.lending.MsgRepayResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitCets {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.borrower.is_empty() {
            len += 1;
        }
        if !self.loan_id.is_empty() {
            len += 1;
        }
        if !self.deposit_tx.is_empty() {
            len += 1;
        }
        if !self.liquidation_cet.is_empty() {
            len += 1;
        }
        if !self.liquidation_adaptor_signatures.is_empty() {
            len += 1;
        }
        if !self.default_liquidation_adaptor_signatures.is_empty() {
            len += 1;
        }
        if !self.repayment_cet.is_empty() {
            len += 1;
        }
        if !self.repayment_signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgSubmitCets", len)?;
        if !self.borrower.is_empty() {
            struct_ser.serialize_field("borrower", &self.borrower)?;
        }
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.deposit_tx.is_empty() {
            struct_ser.serialize_field("depositTx", &self.deposit_tx)?;
        }
        if !self.liquidation_cet.is_empty() {
            struct_ser.serialize_field("liquidationCet", &self.liquidation_cet)?;
        }
        if !self.liquidation_adaptor_signatures.is_empty() {
            struct_ser.serialize_field(
                "liquidationAdaptorSignatures",
                &self.liquidation_adaptor_signatures,
            )?;
        }
        if !self.default_liquidation_adaptor_signatures.is_empty() {
            struct_ser.serialize_field(
                "defaultLiquidationAdaptorSignatures",
                &self.default_liquidation_adaptor_signatures,
            )?;
        }
        if !self.repayment_cet.is_empty() {
            struct_ser.serialize_field("repaymentCet", &self.repayment_cet)?;
        }
        if !self.repayment_signatures.is_empty() {
            struct_ser.serialize_field("repaymentSignatures", &self.repayment_signatures)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitCets {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "borrower",
            "loan_id",
            "loanId",
            "deposit_tx",
            "depositTx",
            "liquidation_cet",
            "liquidationCet",
            "liquidation_adaptor_signatures",
            "liquidationAdaptorSignatures",
            "default_liquidation_adaptor_signatures",
            "defaultLiquidationAdaptorSignatures",
            "repayment_cet",
            "repaymentCet",
            "repayment_signatures",
            "repaymentSignatures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Borrower,
            LoanId,
            DepositTx,
            LiquidationCet,
            LiquidationAdaptorSignatures,
            DefaultLiquidationAdaptorSignatures,
            RepaymentCet,
            RepaymentSignatures,
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
                            "borrower" => Ok(GeneratedField::Borrower),
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
                            "depositTx" | "deposit_tx" => Ok(GeneratedField::DepositTx),
                            "liquidationCet" | "liquidation_cet" => {
                                Ok(GeneratedField::LiquidationCet)
                            }
                            "liquidationAdaptorSignatures" | "liquidation_adaptor_signatures" => {
                                Ok(GeneratedField::LiquidationAdaptorSignatures)
                            }
                            "defaultLiquidationAdaptorSignatures"
                            | "default_liquidation_adaptor_signatures" => {
                                Ok(GeneratedField::DefaultLiquidationAdaptorSignatures)
                            }
                            "repaymentCet" | "repayment_cet" => Ok(GeneratedField::RepaymentCet),
                            "repaymentSignatures" | "repayment_signatures" => {
                                Ok(GeneratedField::RepaymentSignatures)
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
            type Value = MsgSubmitCets;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgSubmitCets")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitCets, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut borrower__ = None;
                let mut loan_id__ = None;
                let mut deposit_tx__ = None;
                let mut liquidation_cet__ = None;
                let mut liquidation_adaptor_signatures__ = None;
                let mut default_liquidation_adaptor_signatures__ = None;
                let mut repayment_cet__ = None;
                let mut repayment_signatures__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Borrower => {
                            if borrower__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrower"));
                            }
                            borrower__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DepositTx => {
                            if deposit_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositTx"));
                            }
                            deposit_tx__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LiquidationCet => {
                            if liquidation_cet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidationCet"));
                            }
                            liquidation_cet__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LiquidationAdaptorSignatures => {
                            if liquidation_adaptor_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationAdaptorSignatures",
                                ));
                            }
                            liquidation_adaptor_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultLiquidationAdaptorSignatures => {
                            if default_liquidation_adaptor_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultLiquidationAdaptorSignatures",
                                ));
                            }
                            default_liquidation_adaptor_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RepaymentCet => {
                            if repayment_cet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repaymentCet"));
                            }
                            repayment_cet__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RepaymentSignatures => {
                            if repayment_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "repaymentSignatures",
                                ));
                            }
                            repayment_signatures__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitCets {
                    borrower: borrower__.unwrap_or_default(),
                    loan_id: loan_id__.unwrap_or_default(),
                    deposit_tx: deposit_tx__.unwrap_or_default(),
                    liquidation_cet: liquidation_cet__.unwrap_or_default(),
                    liquidation_adaptor_signatures: liquidation_adaptor_signatures__
                        .unwrap_or_default(),
                    default_liquidation_adaptor_signatures:
                        default_liquidation_adaptor_signatures__.unwrap_or_default(),
                    repayment_cet: repayment_cet__.unwrap_or_default(),
                    repayment_signatures: repayment_signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.MsgSubmitCets", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitCetsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.lending.MsgSubmitCetsResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitCetsResponse {
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
            type Value = MsgSubmitCetsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgSubmitCetsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitCetsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitCetsResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgSubmitCetsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitPrice {
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
        if !self.price.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgSubmitPrice", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitPrice {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "price"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
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
                            "sender" => Ok(GeneratedField::Sender),
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
            type Value = MsgSubmitPrice;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgSubmitPrice")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgSubmitPrice, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut price__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitPrice {
                    sender: sender__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.MsgSubmitPrice", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitPriceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.lending.MsgSubmitPriceResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitPriceResponse {
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
            type Value = MsgSubmitPriceResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgSubmitPriceResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitPriceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitPriceResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgSubmitPriceResponse",
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
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgUpdateParams", len)?;
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
                formatter.write_str("struct side.lending.MsgUpdateParams")
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
        deserializer.deserialize_struct("side.lending.MsgUpdateParams", FIELDS, GeneratedVisitor)
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
        let struct_ser =
            serializer.serialize_struct("side.lending.MsgUpdateParamsResponse", len)?;
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
                formatter.write_str("struct side.lending.MsgUpdateParamsResponse")
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
            "side.lending.MsgUpdateParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdatePoolConfig {
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
        if !self.pool_id.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.MsgUpdatePoolConfig", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.pool_id.is_empty() {
            struct_ser.serialize_field("poolId", &self.pool_id)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdatePoolConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "pool_id", "poolId", "config"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            PoolId,
            Config,
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
                            "poolId" | "pool_id" => Ok(GeneratedField::PoolId),
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdatePoolConfig;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgUpdatePoolConfig")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgUpdatePoolConfig, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut pool_id__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PoolId => {
                            if pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolId"));
                            }
                            pool_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdatePoolConfig {
                    authority: authority__.unwrap_or_default(),
                    pool_id: pool_id__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgUpdatePoolConfig",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgUpdatePoolConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("side.lending.MsgUpdatePoolConfigResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgUpdatePoolConfigResponse {
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
            type Value = MsgUpdatePoolConfigResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgUpdatePoolConfigResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgUpdatePoolConfigResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdatePoolConfigResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgUpdatePoolConfigResponse",
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
        if self.min_loan_duration.is_some() {
            len += 1;
        }
        if self.max_loan_duration.is_some() {
            len += 1;
        }
        if self.final_timeout_duration.is_some() {
            len += 1;
        }
        if !self.origination_fee_collector.is_empty() {
            len += 1;
        }
        if !self.protocol_fee_collector.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.Params", len)?;
        if let Some(v) = self.min_loan_duration.as_ref() {
            struct_ser.serialize_field("minLoanDuration", v)?;
        }
        if let Some(v) = self.max_loan_duration.as_ref() {
            struct_ser.serialize_field("maxLoanDuration", v)?;
        }
        if let Some(v) = self.final_timeout_duration.as_ref() {
            struct_ser.serialize_field("finalTimeoutDuration", v)?;
        }
        if !self.origination_fee_collector.is_empty() {
            struct_ser
                .serialize_field("originationFeeCollector", &self.origination_fee_collector)?;
        }
        if !self.protocol_fee_collector.is_empty() {
            struct_ser.serialize_field("protocolFeeCollector", &self.protocol_fee_collector)?;
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
            "min_loan_duration",
            "minLoanDuration",
            "max_loan_duration",
            "maxLoanDuration",
            "final_timeout_duration",
            "finalTimeoutDuration",
            "origination_fee_collector",
            "originationFeeCollector",
            "protocol_fee_collector",
            "protocolFeeCollector",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinLoanDuration,
            MaxLoanDuration,
            FinalTimeoutDuration,
            OriginationFeeCollector,
            ProtocolFeeCollector,
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
                            "minLoanDuration" | "min_loan_duration" => {
                                Ok(GeneratedField::MinLoanDuration)
                            }
                            "maxLoanDuration" | "max_loan_duration" => {
                                Ok(GeneratedField::MaxLoanDuration)
                            }
                            "finalTimeoutDuration" | "final_timeout_duration" => {
                                Ok(GeneratedField::FinalTimeoutDuration)
                            }
                            "originationFeeCollector" | "origination_fee_collector" => {
                                Ok(GeneratedField::OriginationFeeCollector)
                            }
                            "protocolFeeCollector" | "protocol_fee_collector" => {
                                Ok(GeneratedField::ProtocolFeeCollector)
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
                formatter.write_str("struct side.lending.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut min_loan_duration__ = None;
                let mut max_loan_duration__ = None;
                let mut final_timeout_duration__ = None;
                let mut origination_fee_collector__ = None;
                let mut protocol_fee_collector__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinLoanDuration => {
                            if min_loan_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minLoanDuration"));
                            }
                            min_loan_duration__ = map_.next_value()?;
                        }
                        GeneratedField::MaxLoanDuration => {
                            if max_loan_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLoanDuration"));
                            }
                            max_loan_duration__ = map_.next_value()?;
                        }
                        GeneratedField::FinalTimeoutDuration => {
                            if final_timeout_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "finalTimeoutDuration",
                                ));
                            }
                            final_timeout_duration__ = map_.next_value()?;
                        }
                        GeneratedField::OriginationFeeCollector => {
                            if origination_fee_collector__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "originationFeeCollector",
                                ));
                            }
                            origination_fee_collector__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProtocolFeeCollector => {
                            if protocol_fee_collector__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "protocolFeeCollector",
                                ));
                            }
                            protocol_fee_collector__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    min_loan_duration: min_loan_duration__,
                    max_loan_duration: max_loan_duration__,
                    final_timeout_duration: final_timeout_duration__,
                    origination_fee_collector: origination_fee_collector__.unwrap_or_default(),
                    protocol_fee_collector: protocol_fee_collector__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.Params", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PoolConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.borrow_apr != 0 {
            len += 1;
        }
        if self.reserve_factor != 0 {
            len += 1;
        }
        if !self.supply_cap.is_empty() {
            len += 1;
        }
        if !self.borrow_cap.is_empty() {
            len += 1;
        }
        if !self.debt_ceiling.is_empty() {
            len += 1;
        }
        if !self.min_borrow_amount.is_empty() {
            len += 1;
        }
        if !self.origination_fee.is_empty() {
            len += 1;
        }
        if self.max_ltv != 0 {
            len += 1;
        }
        if self.liquidation_threshold != 0 {
            len += 1;
        }
        if self.paused {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.PoolConfig", len)?;
        if self.borrow_apr != 0 {
            struct_ser.serialize_field("borrowApr", &self.borrow_apr)?;
        }
        if self.reserve_factor != 0 {
            struct_ser.serialize_field("reserveFactor", &self.reserve_factor)?;
        }
        if !self.supply_cap.is_empty() {
            struct_ser.serialize_field("supplyCap", &self.supply_cap)?;
        }
        if !self.borrow_cap.is_empty() {
            struct_ser.serialize_field("borrowCap", &self.borrow_cap)?;
        }
        if !self.debt_ceiling.is_empty() {
            struct_ser.serialize_field("debtCeiling", &self.debt_ceiling)?;
        }
        if !self.min_borrow_amount.is_empty() {
            struct_ser.serialize_field("minBorrowAmount", &self.min_borrow_amount)?;
        }
        if !self.origination_fee.is_empty() {
            struct_ser.serialize_field("originationFee", &self.origination_fee)?;
        }
        if self.max_ltv != 0 {
            struct_ser.serialize_field("maxLtv", &self.max_ltv)?;
        }
        if self.liquidation_threshold != 0 {
            struct_ser.serialize_field("liquidationThreshold", &self.liquidation_threshold)?;
        }
        if self.paused {
            struct_ser.serialize_field("paused", &self.paused)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PoolConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "borrow_apr",
            "borrowApr",
            "reserve_factor",
            "reserveFactor",
            "supply_cap",
            "supplyCap",
            "borrow_cap",
            "borrowCap",
            "debt_ceiling",
            "debtCeiling",
            "min_borrow_amount",
            "minBorrowAmount",
            "origination_fee",
            "originationFee",
            "max_ltv",
            "maxLtv",
            "liquidation_threshold",
            "liquidationThreshold",
            "paused",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BorrowApr,
            ReserveFactor,
            SupplyCap,
            BorrowCap,
            DebtCeiling,
            MinBorrowAmount,
            OriginationFee,
            MaxLtv,
            LiquidationThreshold,
            Paused,
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
                            "borrowApr" | "borrow_apr" => Ok(GeneratedField::BorrowApr),
                            "reserveFactor" | "reserve_factor" => Ok(GeneratedField::ReserveFactor),
                            "supplyCap" | "supply_cap" => Ok(GeneratedField::SupplyCap),
                            "borrowCap" | "borrow_cap" => Ok(GeneratedField::BorrowCap),
                            "debtCeiling" | "debt_ceiling" => Ok(GeneratedField::DebtCeiling),
                            "minBorrowAmount" | "min_borrow_amount" => {
                                Ok(GeneratedField::MinBorrowAmount)
                            }
                            "originationFee" | "origination_fee" => {
                                Ok(GeneratedField::OriginationFee)
                            }
                            "maxLtv" | "max_ltv" => Ok(GeneratedField::MaxLtv),
                            "liquidationThreshold" | "liquidation_threshold" => {
                                Ok(GeneratedField::LiquidationThreshold)
                            }
                            "paused" => Ok(GeneratedField::Paused),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PoolConfig;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.PoolConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<PoolConfig, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut borrow_apr__ = None;
                let mut reserve_factor__ = None;
                let mut supply_cap__ = None;
                let mut borrow_cap__ = None;
                let mut debt_ceiling__ = None;
                let mut min_borrow_amount__ = None;
                let mut origination_fee__ = None;
                let mut max_ltv__ = None;
                let mut liquidation_threshold__ = None;
                let mut paused__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BorrowApr => {
                            if borrow_apr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowApr"));
                            }
                            borrow_apr__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ReserveFactor => {
                            if reserve_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reserveFactor"));
                            }
                            reserve_factor__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::SupplyCap => {
                            if supply_cap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supplyCap"));
                            }
                            supply_cap__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowCap => {
                            if borrow_cap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowCap"));
                            }
                            borrow_cap__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DebtCeiling => {
                            if debt_ceiling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debtCeiling"));
                            }
                            debt_ceiling__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MinBorrowAmount => {
                            if min_borrow_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minBorrowAmount"));
                            }
                            min_borrow_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OriginationFee => {
                            if origination_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originationFee"));
                            }
                            origination_fee__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxLtv => {
                            if max_ltv__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLtv"));
                            }
                            max_ltv__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LiquidationThreshold => {
                            if liquidation_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationThreshold",
                                ));
                            }
                            liquidation_threshold__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Paused => {
                            if paused__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paused"));
                            }
                            paused__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PoolConfig {
                    borrow_apr: borrow_apr__.unwrap_or_default(),
                    reserve_factor: reserve_factor__.unwrap_or_default(),
                    supply_cap: supply_cap__.unwrap_or_default(),
                    borrow_cap: borrow_cap__.unwrap_or_default(),
                    debt_ceiling: debt_ceiling__.unwrap_or_default(),
                    min_borrow_amount: min_borrow_amount__.unwrap_or_default(),
                    origination_fee: origination_fee__.unwrap_or_default(),
                    max_ltv: max_ltv__.unwrap_or_default(),
                    liquidation_threshold: liquidation_threshold__.unwrap_or_default(),
                    paused: paused__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.PoolConfig", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PoolStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Inactive => "INACTIVE",
            Self::Active => "ACTIVE",
            Self::Paused => "PAUSED",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PoolStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["INACTIVE", "ACTIVE", "PAUSED"];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PoolStatus;

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
                    "INACTIVE" => Ok(PoolStatus::Inactive),
                    "ACTIVE" => Ok(PoolStatus::Active),
                    "PAUSED" => Ok(PoolStatus::Paused),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryCollateralAddressRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.borrower_pubkey.is_empty() {
            len += 1;
        }
        if !self.dcm_pubkey.is_empty() {
            len += 1;
        }
        if self.maturity_time != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryCollateralAddressRequest", len)?;
        if !self.borrower_pubkey.is_empty() {
            struct_ser.serialize_field("borrowerPubkey", &self.borrower_pubkey)?;
        }
        if !self.dcm_pubkey.is_empty() {
            struct_ser.serialize_field("dcmPubkey", &self.dcm_pubkey)?;
        }
        if self.maturity_time != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "maturityTime",
                alloc::string::ToString::to_string(&self.maturity_time).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryCollateralAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "borrower_pubkey",
            "borrowerPubkey",
            "dcm_pubkey",
            "dcmPubkey",
            "maturity_time",
            "maturityTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BorrowerPubkey,
            DcmPubkey,
            MaturityTime,
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
                            "borrowerPubkey" | "borrower_pubkey" => {
                                Ok(GeneratedField::BorrowerPubkey)
                            }
                            "dcmPubkey" | "dcm_pubkey" => Ok(GeneratedField::DcmPubkey),
                            "maturityTime" | "maturity_time" => Ok(GeneratedField::MaturityTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCollateralAddressRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryCollateralAddressRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryCollateralAddressRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut borrower_pubkey__ = None;
                let mut dcm_pubkey__ = None;
                let mut maturity_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BorrowerPubkey => {
                            if borrower_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowerPubkey"));
                            }
                            borrower_pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DcmPubkey => {
                            if dcm_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dcmPubkey"));
                            }
                            dcm_pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaturityTime => {
                            if maturity_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maturityTime"));
                            }
                            maturity_time__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryCollateralAddressRequest {
                    borrower_pubkey: borrower_pubkey__.unwrap_or_default(),
                    dcm_pubkey: dcm_pubkey__.unwrap_or_default(),
                    maturity_time: maturity_time__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryCollateralAddressRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryCollateralAddressResponse {
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
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryCollateralAddressResponse", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryCollateralAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address"];

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
            type Value = QueryCollateralAddressResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryCollateralAddressResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryCollateralAddressResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryCollateralAddressResponse {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryCollateralAddressResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryCurrentInterestRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.loan_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryCurrentInterestRequest", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryCurrentInterestRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["loan_id", "loanId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoanId,
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
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCurrentInterestRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryCurrentInterestRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryCurrentInterestRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut loan_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryCurrentInterestRequest {
                    loan_id: loan_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryCurrentInterestRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryCurrentInterestResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.interest.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryCurrentInterestResponse", len)?;
        if let Some(v) = self.interest.as_ref() {
            struct_ser.serialize_field("interest", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryCurrentInterestResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["interest"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Interest,
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
                            "interest" => Ok(GeneratedField::Interest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCurrentInterestResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryCurrentInterestResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryCurrentInterestResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut interest__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Interest => {
                            if interest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interest"));
                            }
                            interest__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryCurrentInterestResponse {
                    interest: interest__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryCurrentInterestResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLiquidationEventRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pool_id.is_empty() {
            len += 1;
        }
        if !self.collateral_amount.is_empty() {
            len += 1;
        }
        if !self.borrow_amount.is_empty() {
            len += 1;
        }
        if self.term != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLiquidationEventRequest", len)?;
        if !self.pool_id.is_empty() {
            struct_ser.serialize_field("poolId", &self.pool_id)?;
        }
        if !self.collateral_amount.is_empty() {
            struct_ser.serialize_field("collateralAmount", &self.collateral_amount)?;
        }
        if !self.borrow_amount.is_empty() {
            struct_ser.serialize_field("borrowAmount", &self.borrow_amount)?;
        }
        if self.term != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "term",
                alloc::string::ToString::to_string(&self.term).as_str(),
            )?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLiquidationEventRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pool_id",
            "poolId",
            "collateral_amount",
            "collateralAmount",
            "borrow_amount",
            "borrowAmount",
            "term",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PoolId,
            CollateralAmount,
            BorrowAmount,
            Term,
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
                            "poolId" | "pool_id" => Ok(GeneratedField::PoolId),
                            "collateralAmount" | "collateral_amount" => {
                                Ok(GeneratedField::CollateralAmount)
                            }
                            "borrowAmount" | "borrow_amount" => Ok(GeneratedField::BorrowAmount),
                            "term" => Ok(GeneratedField::Term),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLiquidationEventRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLiquidationEventRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLiquidationEventRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pool_id__ = None;
                let mut collateral_amount__ = None;
                let mut borrow_amount__ = None;
                let mut term__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PoolId => {
                            if pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolId"));
                            }
                            pool_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CollateralAmount => {
                            if collateral_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collateralAmount"));
                            }
                            collateral_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowAmount => {
                            if borrow_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowAmount"));
                            }
                            borrow_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Term => {
                            if term__.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            term__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(QueryLiquidationEventRequest {
                    pool_id: pool_id__.unwrap_or_default(),
                    collateral_amount: collateral_amount__.unwrap_or_default(),
                    borrow_amount: borrow_amount__.unwrap_or_default(),
                    term: term__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLiquidationEventRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLiquidationEventResponse {
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
        if !self.oracle_pubkey.is_empty() {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        if !self.price.is_empty() {
            len += 1;
        }
        if !self.signature_point.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLiquidationEventResponse", len)?;
        if self.event_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventId",
                alloc::string::ToString::to_string(&self.event_id).as_str(),
            )?;
        }
        if !self.oracle_pubkey.is_empty() {
            struct_ser.serialize_field("oraclePubkey", &self.oracle_pubkey)?;
        }
        if !self.nonce.is_empty() {
            struct_ser.serialize_field("nonce", &self.nonce)?;
        }
        if !self.price.is_empty() {
            struct_ser.serialize_field("price", &self.price)?;
        }
        if !self.signature_point.is_empty() {
            struct_ser.serialize_field("signaturePoint", &self.signature_point)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLiquidationEventResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_id",
            "eventId",
            "oracle_pubkey",
            "oraclePubkey",
            "nonce",
            "price",
            "signature_point",
            "signaturePoint",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventId,
            OraclePubkey,
            Nonce,
            Price,
            SignaturePoint,
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
                            "oraclePubkey" | "oracle_pubkey" => Ok(GeneratedField::OraclePubkey),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "price" => Ok(GeneratedField::Price),
                            "signaturePoint" | "signature_point" => {
                                Ok(GeneratedField::SignaturePoint)
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
            type Value = QueryLiquidationEventResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLiquidationEventResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLiquidationEventResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_id__ = None;
                let mut oracle_pubkey__ = None;
                let mut nonce__ = None;
                let mut price__ = None;
                let mut signature_point__ = None;
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
                        GeneratedField::OraclePubkey => {
                            if oracle_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oraclePubkey"));
                            }
                            oracle_pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Price => {
                            if price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SignaturePoint => {
                            if signature_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signaturePoint"));
                            }
                            signature_point__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLiquidationEventResponse {
                    event_id: event_id__.unwrap_or_default(),
                    oracle_pubkey: oracle_pubkey__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
                    signature_point: signature_point__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLiquidationEventResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoanCancellationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.loan_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLoanCancellationRequest", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLoanCancellationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["loan_id", "loanId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoanId,
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
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLoanCancellationRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoanCancellationRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLoanCancellationRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut loan_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLoanCancellationRequest {
                    loan_id: loan_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLoanCancellationRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoanCancellationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cancellation.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLoanCancellationResponse", len)?;
        if let Some(v) = self.cancellation.as_ref() {
            struct_ser.serialize_field("cancellation", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLoanCancellationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["cancellation"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cancellation,
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
                            "cancellation" => Ok(GeneratedField::Cancellation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLoanCancellationResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoanCancellationResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLoanCancellationResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut cancellation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cancellation => {
                            if cancellation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancellation"));
                            }
                            cancellation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLoanCancellationResponse {
                    cancellation: cancellation__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLoanCancellationResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoanCetInfosRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.loan_id.is_empty() {
            len += 1;
        }
        if !self.collateral_amount.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLoanCetInfosRequest", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.collateral_amount.is_empty() {
            struct_ser.serialize_field("collateralAmount", &self.collateral_amount)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLoanCetInfosRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["loan_id", "loanId", "collateral_amount", "collateralAmount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoanId,
            CollateralAmount,
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
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
                            "collateralAmount" | "collateral_amount" => {
                                Ok(GeneratedField::CollateralAmount)
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
            type Value = QueryLoanCetInfosRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoanCetInfosRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLoanCetInfosRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut loan_id__ = None;
                let mut collateral_amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CollateralAmount => {
                            if collateral_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collateralAmount"));
                            }
                            collateral_amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLoanCetInfosRequest {
                    loan_id: loan_id__.unwrap_or_default(),
                    collateral_amount: collateral_amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLoanCetInfosRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoanCetInfosResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.liquidation_cet_info.is_some() {
            len += 1;
        }
        if self.default_liquidation_cet_info.is_some() {
            len += 1;
        }
        if self.repayment_cet_info.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLoanCetInfosResponse", len)?;
        if let Some(v) = self.liquidation_cet_info.as_ref() {
            struct_ser.serialize_field("liquidationCetInfo", v)?;
        }
        if let Some(v) = self.default_liquidation_cet_info.as_ref() {
            struct_ser.serialize_field("defaultLiquidationCetInfo", v)?;
        }
        if let Some(v) = self.repayment_cet_info.as_ref() {
            struct_ser.serialize_field("repaymentCetInfo", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLoanCetInfosResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "liquidation_cet_info",
            "liquidationCetInfo",
            "default_liquidation_cet_info",
            "defaultLiquidationCetInfo",
            "repayment_cet_info",
            "repaymentCetInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LiquidationCetInfo,
            DefaultLiquidationCetInfo,
            RepaymentCetInfo,
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
                            "liquidationCetInfo" | "liquidation_cet_info" => {
                                Ok(GeneratedField::LiquidationCetInfo)
                            }
                            "defaultLiquidationCetInfo" | "default_liquidation_cet_info" => {
                                Ok(GeneratedField::DefaultLiquidationCetInfo)
                            }
                            "repaymentCetInfo" | "repayment_cet_info" => {
                                Ok(GeneratedField::RepaymentCetInfo)
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
            type Value = QueryLoanCetInfosResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoanCetInfosResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLoanCetInfosResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut liquidation_cet_info__ = None;
                let mut default_liquidation_cet_info__ = None;
                let mut repayment_cet_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LiquidationCetInfo => {
                            if liquidation_cet_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationCetInfo",
                                ));
                            }
                            liquidation_cet_info__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultLiquidationCetInfo => {
                            if default_liquidation_cet_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultLiquidationCetInfo",
                                ));
                            }
                            default_liquidation_cet_info__ = map_.next_value()?;
                        }
                        GeneratedField::RepaymentCetInfo => {
                            if repayment_cet_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repaymentCetInfo"));
                            }
                            repayment_cet_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLoanCetInfosResponse {
                    liquidation_cet_info: liquidation_cet_info__,
                    default_liquidation_cet_info: default_liquidation_cet_info__,
                    repayment_cet_info: repayment_cet_info__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLoanCetInfosResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoanDlcMetaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.loan_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLoanDlcMetaRequest", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLoanDlcMetaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["loan_id", "loanId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoanId,
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
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLoanDlcMetaRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoanDlcMetaRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLoanDlcMetaRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut loan_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLoanDlcMetaRequest {
                    loan_id: loan_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLoanDlcMetaRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoanDlcMetaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dlc_meta.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLoanDlcMetaResponse", len)?;
        if let Some(v) = self.dlc_meta.as_ref() {
            struct_ser.serialize_field("dlcMeta", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLoanDlcMetaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["dlc_meta", "dlcMeta"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DlcMeta,
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
                            "dlcMeta" | "dlc_meta" => Ok(GeneratedField::DlcMeta),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLoanDlcMetaResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoanDlcMetaResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLoanDlcMetaResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut dlc_meta__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DlcMeta => {
                            if dlc_meta__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dlcMeta"));
                            }
                            dlc_meta__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLoanDlcMetaResponse {
                    dlc_meta: dlc_meta__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLoanDlcMetaResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoanRequest {
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
        let mut struct_ser = serializer.serialize_struct("side.lending.QueryLoanRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLoanRequest {
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
            type Value = QueryLoanRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoanRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryLoanRequest, V::Error>
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
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLoanRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.QueryLoanRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoanResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.loan.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.QueryLoanResponse", len)?;
        if let Some(v) = self.loan.as_ref() {
            struct_ser.serialize_field("loan", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLoanResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["loan"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Loan,
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
                            "loan" => Ok(GeneratedField::Loan),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLoanResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoanResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryLoanResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut loan__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Loan => {
                            if loan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loan"));
                            }
                            loan__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLoanResponse { loan: loan__ })
            }
        }
        deserializer.deserialize_struct("side.lending.QueryLoanResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoansByAddressRequest {
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
        if self.status != 0 {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLoansByAddressRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.status != 0 {
            let v = LoanStatus::try_from(self.status).map_err(|_| {
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
impl<'de> serde::Deserialize<'de> for QueryLoansByAddressRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "status", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
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
            type Value = QueryLoansByAddressRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoansByAddressRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLoansByAddressRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut status__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                            status__ = Some(map_.next_value::<LoanStatus>()? as i32);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLoansByAddressRequest {
                    address: address__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLoansByAddressRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoansByAddressResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.loans.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLoansByAddressResponse", len)?;
        if !self.loans.is_empty() {
            struct_ser.serialize_field("loans", &self.loans)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLoansByAddressResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["loans", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Loans,
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
                            "loans" => Ok(GeneratedField::Loans),
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
            type Value = QueryLoansByAddressResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoansByAddressResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLoansByAddressResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut loans__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Loans => {
                            if loans__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loans"));
                            }
                            loans__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLoansByAddressResponse {
                    loans: loans__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLoansByAddressResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoansRequest {
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
        let mut struct_ser = serializer.serialize_struct("side.lending.QueryLoansRequest", len)?;
        if self.status != 0 {
            let v = LoanStatus::try_from(self.status).map_err(|_| {
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
impl<'de> serde::Deserialize<'de> for QueryLoansRequest {
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
            type Value = QueryLoansRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoansRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryLoansRequest, V::Error>
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
                            status__ = Some(map_.next_value::<LoanStatus>()? as i32);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLoansRequest {
                    status: status__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.lending.QueryLoansRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoansResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.loans.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.QueryLoansResponse", len)?;
        if !self.loans.is_empty() {
            struct_ser.serialize_field("loans", &self.loans)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLoansResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["loans", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Loans,
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
                            "loans" => Ok(GeneratedField::Loans),
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
            type Value = QueryLoansResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoansResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryLoansResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut loans__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Loans => {
                            if loans__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loans"));
                            }
                            loans__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLoansResponse {
                    loans: loans__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.lending.QueryLoansResponse", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("side.lending.QueryParamsRequest", len)?;
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
                formatter.write_str("struct side.lending.QueryParamsRequest")
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
        deserializer.deserialize_struct("side.lending.QueryParamsRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryParamsResponse", len)?;
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
                formatter.write_str("struct side.lending.QueryParamsResponse")
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
        deserializer.deserialize_struct(
            "side.lending.QueryParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPoolExchangeRateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pool_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryPoolExchangeRateRequest", len)?;
        if !self.pool_id.is_empty() {
            struct_ser.serialize_field("poolId", &self.pool_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPoolExchangeRateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pool_id", "poolId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PoolId,
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
                            "poolId" | "pool_id" => Ok(GeneratedField::PoolId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPoolExchangeRateRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryPoolExchangeRateRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryPoolExchangeRateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pool_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PoolId => {
                            if pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolId"));
                            }
                            pool_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPoolExchangeRateRequest {
                    pool_id: pool_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryPoolExchangeRateRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPoolExchangeRateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.exchange_rate.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryPoolExchangeRateResponse", len)?;
        if !self.exchange_rate.is_empty() {
            struct_ser.serialize_field("exchangeRate", &self.exchange_rate)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPoolExchangeRateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["exchange_rate", "exchangeRate"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExchangeRate,
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
                            "exchangeRate" | "exchange_rate" => Ok(GeneratedField::ExchangeRate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPoolExchangeRateResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryPoolExchangeRateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryPoolExchangeRateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut exchange_rate__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExchangeRate => {
                            if exchange_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exchangeRate"));
                            }
                            exchange_rate__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPoolExchangeRateResponse {
                    exchange_rate: exchange_rate__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryPoolExchangeRateResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPoolRequest {
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
        let mut struct_ser = serializer.serialize_struct("side.lending.QueryPoolRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPoolRequest {
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
            type Value = QueryPoolRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryPoolRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryPoolRequest, V::Error>
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
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPoolRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.QueryPoolRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPoolResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pool.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.QueryPoolResponse", len)?;
        if let Some(v) = self.pool.as_ref() {
            struct_ser.serialize_field("pool", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPoolResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pool"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pool,
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
                            "pool" => Ok(GeneratedField::Pool),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPoolResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryPoolResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryPoolResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pool__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pool => {
                            if pool__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pool"));
                            }
                            pool__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryPoolResponse { pool: pool__ })
            }
        }
        deserializer.deserialize_struct("side.lending.QueryPoolResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPoolsRequest {
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
        let mut struct_ser = serializer.serialize_struct("side.lending.QueryPoolsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPoolsRequest {
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
            type Value = QueryPoolsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryPoolsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryPoolsRequest, V::Error>
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
                Ok(QueryPoolsRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.lending.QueryPoolsRequest", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryPoolsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pools.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.QueryPoolsResponse", len)?;
        if !self.pools.is_empty() {
            struct_ser.serialize_field("pools", &self.pools)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryPoolsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pools", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pools,
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
                            "pools" => Ok(GeneratedField::Pools),
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
            type Value = QueryPoolsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryPoolsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryPoolsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pools__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pools => {
                            if pools__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pools"));
                            }
                            pools__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryPoolsResponse {
                    pools: pools__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("side.lending.QueryPoolsResponse", FIELDS, GeneratedVisitor)
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
        if !self.pair.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.QueryPriceRequest", len)?;
        if !self.pair.is_empty() {
            struct_ser.serialize_field("pair", &self.pair)?;
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
        const FIELDS: &[&str] = &["pair"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pair,
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
                            "pair" => Ok(GeneratedField::Pair),
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
                formatter.write_str("struct side.lending.QueryPriceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryPriceRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pair__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pair => {
                            if pair__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pair"));
                            }
                            pair__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPriceRequest {
                    pair: pair__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.QueryPriceRequest", FIELDS, GeneratedVisitor)
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
        if !self.price.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.QueryPriceResponse", len)?;
        if !self.price.is_empty() {
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
                formatter.write_str("struct side.lending.QueryPriceResponse")
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
                            price__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryPriceResponse {
                    price: price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.QueryPriceResponse", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryRepaymentRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.loan_id.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryRepaymentRequest", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryRepaymentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["loan_id", "loanId"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoanId,
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
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRepaymentRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryRepaymentRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryRepaymentRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut loan_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryRepaymentRequest {
                    loan_id: loan_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryRepaymentRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryRepaymentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.repayment.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryRepaymentResponse", len)?;
        if let Some(v) = self.repayment.as_ref() {
            struct_ser.serialize_field("repayment", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryRepaymentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["repayment"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Repayment,
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
                            "repayment" => Ok(GeneratedField::Repayment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRepaymentResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryRepaymentResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryRepaymentResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut repayment__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Repayment => {
                            if repayment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repayment"));
                            }
                            repayment__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryRepaymentResponse {
                    repayment: repayment__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryRepaymentResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Repayment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.loan_id.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if self.create_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.Repayment", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if let Some(v) = self.create_at.as_ref() {
            struct_ser.serialize_field("createAt", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Repayment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["loan_id", "loanId", "amount", "create_at", "createAt"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoanId,
            Amount,
            CreateAt,
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
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
                            "amount" => Ok(GeneratedField::Amount),
                            "createAt" | "create_at" => Ok(GeneratedField::CreateAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Repayment;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.Repayment")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Repayment, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut loan_id__ = None;
                let mut amount__ = None;
                let mut create_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                        GeneratedField::CreateAt => {
                            if create_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createAt"));
                            }
                            create_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Repayment {
                    loan_id: loan_id__.unwrap_or_default(),
                    amount: amount__,
                    create_at: create_at__,
                })
            }
        }
        deserializer.deserialize_struct("side.lending.Repayment", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for RepaymentCet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tx.is_empty() {
            len += 1;
        }
        if !self.dcm_adaptor_signatures.is_empty() {
            len += 1;
        }
        if !self.dcm_adapted_signatures.is_empty() {
            len += 1;
        }
        if !self.borrower_signatures.is_empty() {
            len += 1;
        }
        if !self.signed_tx_hex.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.RepaymentCet", len)?;
        if !self.tx.is_empty() {
            struct_ser.serialize_field("tx", &self.tx)?;
        }
        if !self.dcm_adaptor_signatures.is_empty() {
            struct_ser.serialize_field("dcmAdaptorSignatures", &self.dcm_adaptor_signatures)?;
        }
        if !self.dcm_adapted_signatures.is_empty() {
            struct_ser.serialize_field("dcmAdaptedSignatures", &self.dcm_adapted_signatures)?;
        }
        if !self.borrower_signatures.is_empty() {
            struct_ser.serialize_field("borrowerSignatures", &self.borrower_signatures)?;
        }
        if !self.signed_tx_hex.is_empty() {
            struct_ser.serialize_field("signedTxHex", &self.signed_tx_hex)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RepaymentCet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tx",
            "dcm_adaptor_signatures",
            "dcmAdaptorSignatures",
            "dcm_adapted_signatures",
            "dcmAdaptedSignatures",
            "borrower_signatures",
            "borrowerSignatures",
            "signed_tx_hex",
            "signedTxHex",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tx,
            DcmAdaptorSignatures,
            DcmAdaptedSignatures,
            BorrowerSignatures,
            SignedTxHex,
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
                            "tx" => Ok(GeneratedField::Tx),
                            "dcmAdaptorSignatures" | "dcm_adaptor_signatures" => {
                                Ok(GeneratedField::DcmAdaptorSignatures)
                            }
                            "dcmAdaptedSignatures" | "dcm_adapted_signatures" => {
                                Ok(GeneratedField::DcmAdaptedSignatures)
                            }
                            "borrowerSignatures" | "borrower_signatures" => {
                                Ok(GeneratedField::BorrowerSignatures)
                            }
                            "signedTxHex" | "signed_tx_hex" => Ok(GeneratedField::SignedTxHex),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RepaymentCet;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.RepaymentCet")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<RepaymentCet, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut tx__ = None;
                let mut dcm_adaptor_signatures__ = None;
                let mut dcm_adapted_signatures__ = None;
                let mut borrower_signatures__ = None;
                let mut signed_tx_hex__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tx => {
                            if tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tx"));
                            }
                            tx__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DcmAdaptorSignatures => {
                            if dcm_adaptor_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "dcmAdaptorSignatures",
                                ));
                            }
                            dcm_adaptor_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DcmAdaptedSignatures => {
                            if dcm_adapted_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "dcmAdaptedSignatures",
                                ));
                            }
                            dcm_adapted_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowerSignatures => {
                            if borrower_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "borrowerSignatures",
                                ));
                            }
                            borrower_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SignedTxHex => {
                            if signed_tx_hex__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signedTxHex"));
                            }
                            signed_tx_hex__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RepaymentCet {
                    tx: tx__.unwrap_or_default(),
                    dcm_adaptor_signatures: dcm_adaptor_signatures__.unwrap_or_default(),
                    dcm_adapted_signatures: dcm_adapted_signatures__.unwrap_or_default(),
                    borrower_signatures: borrower_signatures__.unwrap_or_default(),
                    signed_tx_hex: signed_tx_hex__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.RepaymentCet", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SigningIntent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Repayment => "SIGNING_INTENT_REPAYMENT",
            Self::Liquidation => "SIGNING_INTENT_LIQUIDATION",
            Self::DefaultLiquidation => "SIGNING_INTENT_DEFAULT_LIQUIDATION",
            Self::Cancellation => "SIGNING_INTENT_CANCELLATION",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SigningIntent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SIGNING_INTENT_REPAYMENT",
            "SIGNING_INTENT_LIQUIDATION",
            "SIGNING_INTENT_DEFAULT_LIQUIDATION",
            "SIGNING_INTENT_CANCELLATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SigningIntent;

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
                    "SIGNING_INTENT_REPAYMENT" => Ok(SigningIntent::Repayment),
                    "SIGNING_INTENT_LIQUIDATION" => Ok(SigningIntent::Liquidation),
                    "SIGNING_INTENT_DEFAULT_LIQUIDATION" => Ok(SigningIntent::DefaultLiquidation),
                    "SIGNING_INTENT_CANCELLATION" => Ok(SigningIntent::Cancellation),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
