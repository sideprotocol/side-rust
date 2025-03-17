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
        if !self.dca_signatures.is_empty() {
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
        if !self.dca_signatures.is_empty() {
            struct_ser.serialize_field("dcaSignatures", &self.dca_signatures)?;
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
            "dca_signatures",
            "dcaSignatures",
            "create_at",
            "createAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoanId,
            Txid,
            Tx,
            Signatures,
            DcaSignatures,
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
                            "dcaSignatures" | "dca_signatures" => Ok(GeneratedField::DcaSignatures),
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
                let mut dca_signatures__ = None;
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
                        GeneratedField::DcaSignatures => {
                            if dca_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dcaSignatures"));
                            }
                            dca_signatures__ = Some(map_.next_value()?);
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
                    dca_signatures: dca_signatures__.unwrap_or_default(),
                    create_at: create_at__,
                })
            }
        }
        deserializer.deserialize_struct("side.lending.Cancellation", FIELDS, GeneratedVisitor)
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
        if !self.liquidation_cet.is_empty() {
            len += 1;
        }
        if !self.signed_liquidation_cet_hex.is_empty() {
            len += 1;
        }
        if !self.liquidation_adaptor_signatures.is_empty() {
            len += 1;
        }
        if !self.liquidation_adapted_signatures.is_empty() {
            len += 1;
        }
        if !self.liquidation_agency_signatures.is_empty() {
            len += 1;
        }
        if self.vault_utxo.is_some() {
            len += 1;
        }
        if !self.internal_key.is_empty() {
            len += 1;
        }
        if !self.liquidation_cet_script.is_empty() {
            len += 1;
        }
        if !self.repayment_script.is_empty() {
            len += 1;
        }
        if !self.forced_repayment_script.is_empty() {
            len += 1;
        }
        if !self.timeout_refund_script.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.DLCMeta", len)?;
        if !self.liquidation_cet.is_empty() {
            struct_ser.serialize_field("liquidationCet", &self.liquidation_cet)?;
        }
        if !self.signed_liquidation_cet_hex.is_empty() {
            struct_ser
                .serialize_field("signedLiquidationCetHex", &self.signed_liquidation_cet_hex)?;
        }
        if !self.liquidation_adaptor_signatures.is_empty() {
            struct_ser.serialize_field(
                "liquidationAdaptorSignatures",
                &self.liquidation_adaptor_signatures,
            )?;
        }
        if !self.liquidation_adapted_signatures.is_empty() {
            struct_ser.serialize_field(
                "liquidationAdaptedSignatures",
                &self.liquidation_adapted_signatures,
            )?;
        }
        if !self.liquidation_agency_signatures.is_empty() {
            struct_ser.serialize_field(
                "liquidationAgencySignatures",
                &self.liquidation_agency_signatures,
            )?;
        }
        if let Some(v) = self.vault_utxo.as_ref() {
            struct_ser.serialize_field("vaultUtxo", v)?;
        }
        if !self.internal_key.is_empty() {
            struct_ser.serialize_field("internalKey", &self.internal_key)?;
        }
        if !self.liquidation_cet_script.is_empty() {
            struct_ser.serialize_field("liquidationCetScript", &self.liquidation_cet_script)?;
        }
        if !self.repayment_script.is_empty() {
            struct_ser.serialize_field("repaymentScript", &self.repayment_script)?;
        }
        if !self.forced_repayment_script.is_empty() {
            struct_ser.serialize_field("forcedRepaymentScript", &self.forced_repayment_script)?;
        }
        if !self.timeout_refund_script.is_empty() {
            struct_ser.serialize_field("timeoutRefundScript", &self.timeout_refund_script)?;
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
            "signed_liquidation_cet_hex",
            "signedLiquidationCetHex",
            "liquidation_adaptor_signatures",
            "liquidationAdaptorSignatures",
            "liquidation_adapted_signatures",
            "liquidationAdaptedSignatures",
            "liquidation_agency_signatures",
            "liquidationAgencySignatures",
            "vault_utxo",
            "vaultUtxo",
            "internal_key",
            "internalKey",
            "liquidation_cet_script",
            "liquidationCetScript",
            "repayment_script",
            "repaymentScript",
            "forced_repayment_script",
            "forcedRepaymentScript",
            "timeout_refund_script",
            "timeoutRefundScript",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LiquidationCet,
            SignedLiquidationCetHex,
            LiquidationAdaptorSignatures,
            LiquidationAdaptedSignatures,
            LiquidationAgencySignatures,
            VaultUtxo,
            InternalKey,
            LiquidationCetScript,
            RepaymentScript,
            ForcedRepaymentScript,
            TimeoutRefundScript,
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
                            "signedLiquidationCetHex" | "signed_liquidation_cet_hex" => {
                                Ok(GeneratedField::SignedLiquidationCetHex)
                            }
                            "liquidationAdaptorSignatures" | "liquidation_adaptor_signatures" => {
                                Ok(GeneratedField::LiquidationAdaptorSignatures)
                            }
                            "liquidationAdaptedSignatures" | "liquidation_adapted_signatures" => {
                                Ok(GeneratedField::LiquidationAdaptedSignatures)
                            }
                            "liquidationAgencySignatures" | "liquidation_agency_signatures" => {
                                Ok(GeneratedField::LiquidationAgencySignatures)
                            }
                            "vaultUtxo" | "vault_utxo" => Ok(GeneratedField::VaultUtxo),
                            "internalKey" | "internal_key" => Ok(GeneratedField::InternalKey),
                            "liquidationCetScript" | "liquidation_cet_script" => {
                                Ok(GeneratedField::LiquidationCetScript)
                            }
                            "repaymentScript" | "repayment_script" => {
                                Ok(GeneratedField::RepaymentScript)
                            }
                            "forcedRepaymentScript" | "forced_repayment_script" => {
                                Ok(GeneratedField::ForcedRepaymentScript)
                            }
                            "timeoutRefundScript" | "timeout_refund_script" => {
                                Ok(GeneratedField::TimeoutRefundScript)
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
                let mut signed_liquidation_cet_hex__ = None;
                let mut liquidation_adaptor_signatures__ = None;
                let mut liquidation_adapted_signatures__ = None;
                let mut liquidation_agency_signatures__ = None;
                let mut vault_utxo__ = None;
                let mut internal_key__ = None;
                let mut liquidation_cet_script__ = None;
                let mut repayment_script__ = None;
                let mut forced_repayment_script__ = None;
                let mut timeout_refund_script__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LiquidationCet => {
                            if liquidation_cet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidationCet"));
                            }
                            liquidation_cet__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SignedLiquidationCetHex => {
                            if signed_liquidation_cet_hex__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "signedLiquidationCetHex",
                                ));
                            }
                            signed_liquidation_cet_hex__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LiquidationAdaptorSignatures => {
                            if liquidation_adaptor_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationAdaptorSignatures",
                                ));
                            }
                            liquidation_adaptor_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LiquidationAdaptedSignatures => {
                            if liquidation_adapted_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationAdaptedSignatures",
                                ));
                            }
                            liquidation_adapted_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LiquidationAgencySignatures => {
                            if liquidation_agency_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationAgencySignatures",
                                ));
                            }
                            liquidation_agency_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VaultUtxo => {
                            if vault_utxo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultUtxo"));
                            }
                            vault_utxo__ = map_.next_value()?;
                        }
                        GeneratedField::InternalKey => {
                            if internal_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalKey"));
                            }
                            internal_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LiquidationCetScript => {
                            if liquidation_cet_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationCetScript",
                                ));
                            }
                            liquidation_cet_script__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RepaymentScript => {
                            if repayment_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repaymentScript"));
                            }
                            repayment_script__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ForcedRepaymentScript => {
                            if forced_repayment_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "forcedRepaymentScript",
                                ));
                            }
                            forced_repayment_script__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TimeoutRefundScript => {
                            if timeout_refund_script__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "timeoutRefundScript",
                                ));
                            }
                            timeout_refund_script__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DlcMeta {
                    liquidation_cet: liquidation_cet__.unwrap_or_default(),
                    signed_liquidation_cet_hex: signed_liquidation_cet_hex__.unwrap_or_default(),
                    liquidation_adaptor_signatures: liquidation_adaptor_signatures__
                        .unwrap_or_default(),
                    liquidation_adapted_signatures: liquidation_adapted_signatures__
                        .unwrap_or_default(),
                    liquidation_agency_signatures: liquidation_agency_signatures__
                        .unwrap_or_default(),
                    vault_utxo: vault_utxo__,
                    internal_key: internal_key__.unwrap_or_default(),
                    liquidation_cet_script: liquidation_cet_script__.unwrap_or_default(),
                    repayment_script: repayment_script__.unwrap_or_default(),
                    forced_repayment_script: forced_repayment_script__.unwrap_or_default(),
                    timeout_refund_script: timeout_refund_script__.unwrap_or_default(),
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Txid,
            VaultAddress,
            DepositTx,
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
                    }
                }
                Ok(DepositLog {
                    txid: txid__.unwrap_or_default(),
                    vault_address: vault_address__.unwrap_or_default(),
                    deposit_tx: deposit_tx__.unwrap_or_default(),
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
        if !self.borrowed_amount.is_empty() {
            len += 1;
        }
        if !self.total_shares.is_empty() {
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
        if !self.borrowed_amount.is_empty() {
            struct_ser.serialize_field("borrowedAmount", &self.borrowed_amount)?;
        }
        if !self.total_shares.is_empty() {
            struct_ser.serialize_field("totalShares", &self.total_shares)?;
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
            "borrowed_amount",
            "borrowedAmount",
            "total_shares",
            "totalShares",
            "config",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Supply,
            AvailableAmount,
            BorrowedAmount,
            TotalShares,
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
                            "borrowedAmount" | "borrowed_amount" => {
                                Ok(GeneratedField::BorrowedAmount)
                            }
                            "totalShares" | "total_shares" => Ok(GeneratedField::TotalShares),
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
                let mut borrowed_amount__ = None;
                let mut total_shares__ = None;
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
                        GeneratedField::BorrowedAmount => {
                            if borrowed_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowedAmount"));
                            }
                            borrowed_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalShares => {
                            if total_shares__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalShares"));
                            }
                            total_shares__ = Some(map_.next_value()?);
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
                    borrowed_amount: borrowed_amount__.unwrap_or_default(),
                    total_shares: total_shares__.unwrap_or_default(),
                    config: config__,
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.LendingPool", FIELDS, GeneratedVisitor)
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
        if !self.agency.is_empty() {
            len += 1;
        }
        if self.maturity_time != 0 {
            len += 1;
        }
        if self.final_timeout != 0 {
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
        if !self.term.is_empty() {
            len += 1;
        }
        if self.event_id != 0 {
            len += 1;
        }
        if self.attestation_id != 0 {
            len += 1;
        }
        if !self.deposit_txs.is_empty() {
            len += 1;
        }
        if !self.collateral_amount.is_empty() {
            len += 1;
        }
        if !self.pool_id.is_empty() {
            len += 1;
        }
        if self.auction_id != 0 {
            len += 1;
        }
        if self.create_at.is_some() {
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
        if !self.agency.is_empty() {
            struct_ser.serialize_field("agency", &self.agency)?;
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
        if !self.term.is_empty() {
            struct_ser.serialize_field("term", &self.term)?;
        }
        if self.event_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventId",
                alloc::string::ToString::to_string(&self.event_id).as_str(),
            )?;
        }
        if self.attestation_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "attestationId",
                alloc::string::ToString::to_string(&self.attestation_id).as_str(),
            )?;
        }
        if !self.deposit_txs.is_empty() {
            struct_ser.serialize_field("depositTxs", &self.deposit_txs)?;
        }
        if !self.collateral_amount.is_empty() {
            struct_ser.serialize_field("collateralAmount", &self.collateral_amount)?;
        }
        if !self.pool_id.is_empty() {
            struct_ser.serialize_field("poolId", &self.pool_id)?;
        }
        if self.auction_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "auctionId",
                alloc::string::ToString::to_string(&self.auction_id).as_str(),
            )?;
        }
        if let Some(v) = self.create_at.as_ref() {
            struct_ser.serialize_field("createAt", v)?;
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
            "agency",
            "maturity_time",
            "maturityTime",
            "final_timeout",
            "finalTimeout",
            "borrow_amount",
            "borrowAmount",
            "origination_fee",
            "originationFee",
            "interest",
            "protocol_fee",
            "protocolFee",
            "term",
            "event_id",
            "eventId",
            "attestation_id",
            "attestationId",
            "deposit_txs",
            "depositTxs",
            "collateral_amount",
            "collateralAmount",
            "pool_id",
            "poolId",
            "auction_id",
            "auctionId",
            "create_at",
            "createAt",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VaultAddress,
            Borrower,
            BorrowerPubKey,
            Agency,
            MaturityTime,
            FinalTimeout,
            BorrowAmount,
            OriginationFee,
            Interest,
            ProtocolFee,
            Term,
            EventId,
            AttestationId,
            DepositTxs,
            CollateralAmount,
            PoolId,
            AuctionId,
            CreateAt,
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
                            "agency" => Ok(GeneratedField::Agency),
                            "maturityTime" | "maturity_time" => Ok(GeneratedField::MaturityTime),
                            "finalTimeout" | "final_timeout" => Ok(GeneratedField::FinalTimeout),
                            "borrowAmount" | "borrow_amount" => Ok(GeneratedField::BorrowAmount),
                            "originationFee" | "origination_fee" => {
                                Ok(GeneratedField::OriginationFee)
                            }
                            "interest" => Ok(GeneratedField::Interest),
                            "protocolFee" | "protocol_fee" => Ok(GeneratedField::ProtocolFee),
                            "term" => Ok(GeneratedField::Term),
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            "attestationId" | "attestation_id" => Ok(GeneratedField::AttestationId),
                            "depositTxs" | "deposit_txs" => Ok(GeneratedField::DepositTxs),
                            "collateralAmount" | "collateral_amount" => {
                                Ok(GeneratedField::CollateralAmount)
                            }
                            "poolId" | "pool_id" => Ok(GeneratedField::PoolId),
                            "auctionId" | "auction_id" => Ok(GeneratedField::AuctionId),
                            "createAt" | "create_at" => Ok(GeneratedField::CreateAt),
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
                let mut agency__ = None;
                let mut maturity_time__ = None;
                let mut final_timeout__ = None;
                let mut borrow_amount__ = None;
                let mut origination_fee__ = None;
                let mut interest__ = None;
                let mut protocol_fee__ = None;
                let mut term__ = None;
                let mut event_id__ = None;
                let mut attestation_id__ = None;
                let mut deposit_txs__ = None;
                let mut collateral_amount__ = None;
                let mut pool_id__ = None;
                let mut auction_id__ = None;
                let mut create_at__ = None;
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
                        GeneratedField::Agency => {
                            if agency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agency"));
                            }
                            agency__ = Some(map_.next_value()?);
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
                            term__ = Some(map_.next_value()?);
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
                        GeneratedField::AttestationId => {
                            if attestation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestationId"));
                            }
                            attestation_id__ = Some(
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
                        GeneratedField::PoolId => {
                            if pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolId"));
                            }
                            pool_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuctionId => {
                            if auction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auctionId"));
                            }
                            auction_id__ = Some(
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
                    agency: agency__.unwrap_or_default(),
                    maturity_time: maturity_time__.unwrap_or_default(),
                    final_timeout: final_timeout__.unwrap_or_default(),
                    borrow_amount: borrow_amount__,
                    origination_fee: origination_fee__.unwrap_or_default(),
                    interest: interest__.unwrap_or_default(),
                    protocol_fee: protocol_fee__.unwrap_or_default(),
                    term: term__.unwrap_or_default(),
                    event_id: event_id__.unwrap_or_default(),
                    attestation_id: attestation_id__.unwrap_or_default(),
                    deposit_txs: deposit_txs__.unwrap_or_default(),
                    collateral_amount: collateral_amount__.unwrap_or_default(),
                    pool_id: pool_id__.unwrap_or_default(),
                    auction_id: auction_id__.unwrap_or_default(),
                    create_at: create_at__,
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
        if self.agency_id != 0 {
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
        if self.agency_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "agencyId",
                alloc::string::ToString::to_string(&self.agency_id).as_str(),
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
            "agency_id",
            "agencyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Borrower,
            BorrowerPubkey,
            MaturityTime,
            PoolId,
            BorrowAmount,
            AgencyId,
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
                            "agencyId" | "agency_id" => Ok(GeneratedField::AgencyId),
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
                let mut agency_id__ = None;
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
                        GeneratedField::AgencyId => {
                            if agency_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agencyId"));
                            }
                            agency_id__ = Some(
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
                    agency_id: agency_id__.unwrap_or_default(),
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
        if !self.deposit_tx_id.is_empty() {
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
        if !self.deposit_tx_id.is_empty() {
            struct_ser.serialize_field("depositTxId", &self.deposit_tx_id)?;
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
            "deposit_tx_id",
            "depositTxId",
            "block_hash",
            "blockHash",
            "proof",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relayer,
            DepositTxId,
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
                            "depositTxId" | "deposit_tx_id" => Ok(GeneratedField::DepositTxId),
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
                let mut deposit_tx_id__ = None;
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
                        GeneratedField::DepositTxId => {
                            if deposit_tx_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositTxId"));
                            }
                            deposit_tx_id__ = Some(map_.next_value()?);
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
                    deposit_tx_id: deposit_tx_id__.unwrap_or_default(),
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
impl serde::Serialize for MsgClose {
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
        if !self.loan_id.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgClose", len)?;
        if !self.relayer.is_empty() {
            struct_ser.serialize_field("relayer", &self.relayer)?;
        }
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", &self.signature)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgClose {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["relayer", "loan_id", "loanId", "signature"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relayer,
            LoanId,
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
                            "relayer" => Ok(GeneratedField::Relayer),
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
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
            type Value = MsgClose;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgClose")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgClose, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut relayer__ = None;
                let mut loan_id__ = None;
                let mut signature__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Relayer => {
                            if relayer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayer"));
                            }
                            relayer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgClose {
                    relayer: relayer__.unwrap_or_default(),
                    loan_id: loan_id__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.MsgClose", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgCloseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.lending.MsgCloseResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgCloseResponse {
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
            type Value = MsgCloseResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgCloseResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgCloseResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCloseResponse {})
            }
        }
        deserializer.deserialize_struct("side.lending.MsgCloseResponse", FIELDS, GeneratedVisitor)
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
        if self.shares.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgRemoveLiquidity", len)?;
        if !self.lender.is_empty() {
            struct_ser.serialize_field("lender", &self.lender)?;
        }
        if let Some(v) = self.shares.as_ref() {
            struct_ser.serialize_field("shares", v)?;
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
        const FIELDS: &[&str] = &["lender", "shares"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Lender,
            Shares,
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
                            "shares" => Ok(GeneratedField::Shares),
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
                let mut shares__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Lender => {
                            if lender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lender"));
                            }
                            lender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Shares => {
                            if shares__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shares"));
                            }
                            shares__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgRemoveLiquidity {
                    lender: lender__.unwrap_or_default(),
                    shares: shares__,
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
        if !self.adaptor_point.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgRepay", len)?;
        if !self.borrower.is_empty() {
            struct_ser.serialize_field("borrower", &self.borrower)?;
        }
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.adaptor_point.is_empty() {
            struct_ser.serialize_field("adaptorPoint", &self.adaptor_point)?;
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
        const FIELDS: &[&str] = &[
            "borrower",
            "loan_id",
            "loanId",
            "adaptor_point",
            "adaptorPoint",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Borrower,
            LoanId,
            AdaptorPoint,
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
                            "adaptorPoint" | "adaptor_point" => Ok(GeneratedField::AdaptorPoint),
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
                let mut adaptor_point__ = None;
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
                        GeneratedField::AdaptorPoint => {
                            if adaptor_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adaptorPoint"));
                            }
                            adaptor_point__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRepay {
                    borrower: borrower__.unwrap_or_default(),
                    loan_id: loan_id__.unwrap_or_default(),
                    adaptor_point: adaptor_point__.unwrap_or_default(),
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
impl serde::Serialize for MsgSubmitCancellationSignatures {
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
        if !self.loan_id.is_empty() {
            len += 1;
        }
        if !self.signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.MsgSubmitCancellationSignatures", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitCancellationSignatures {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "loan_id", "loanId", "signatures"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            LoanId,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
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
            type Value = MsgSubmitCancellationSignatures;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgSubmitCancellationSignatures")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitCancellationSignatures, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut loan_id__ = None;
                let mut signatures__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitCancellationSignatures {
                    sender: sender__.unwrap_or_default(),
                    loan_id: loan_id__.unwrap_or_default(),
                    signatures: signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgSubmitCancellationSignatures",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitCancellationSignaturesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("side.lending.MsgSubmitCancellationSignaturesResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitCancellationSignaturesResponse {
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
            type Value = MsgSubmitCancellationSignaturesResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgSubmitCancellationSignaturesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitCancellationSignaturesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitCancellationSignaturesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgSubmitCancellationSignaturesResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitLiquidationCet {
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
        if self.event_id != 0 {
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
        let mut struct_ser =
            serializer.serialize_struct("side.lending.MsgSubmitLiquidationCet", len)?;
        if !self.borrower.is_empty() {
            struct_ser.serialize_field("borrower", &self.borrower)?;
        }
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if self.event_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "eventId",
                alloc::string::ToString::to_string(&self.event_id).as_str(),
            )?;
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
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitLiquidationCet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "borrower",
            "loan_id",
            "loanId",
            "event_id",
            "eventId",
            "deposit_tx",
            "depositTx",
            "liquidation_cet",
            "liquidationCet",
            "liquidation_adaptor_signatures",
            "liquidationAdaptorSignatures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Borrower,
            LoanId,
            EventId,
            DepositTx,
            LiquidationCet,
            LiquidationAdaptorSignatures,
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
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            "depositTx" | "deposit_tx" => Ok(GeneratedField::DepositTx),
                            "liquidationCet" | "liquidation_cet" => {
                                Ok(GeneratedField::LiquidationCet)
                            }
                            "liquidationAdaptorSignatures" | "liquidation_adaptor_signatures" => {
                                Ok(GeneratedField::LiquidationAdaptorSignatures)
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
            type Value = MsgSubmitLiquidationCet;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgSubmitLiquidationCet")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitLiquidationCet, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut borrower__ = None;
                let mut loan_id__ = None;
                let mut event_id__ = None;
                let mut deposit_tx__ = None;
                let mut liquidation_cet__ = None;
                let mut liquidation_adaptor_signatures__ = None;
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
                        GeneratedField::EventId => {
                            if event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventId"));
                            }
                            event_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
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
                    }
                }
                Ok(MsgSubmitLiquidationCet {
                    borrower: borrower__.unwrap_or_default(),
                    loan_id: loan_id__.unwrap_or_default(),
                    event_id: event_id__.unwrap_or_default(),
                    deposit_tx: deposit_tx__.unwrap_or_default(),
                    liquidation_cet: liquidation_cet__.unwrap_or_default(),
                    liquidation_adaptor_signatures: liquidation_adaptor_signatures__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgSubmitLiquidationCet",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitLiquidationCetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("side.lending.MsgSubmitLiquidationCetResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitLiquidationCetResponse {
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
            type Value = MsgSubmitLiquidationCetResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgSubmitLiquidationCetResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitLiquidationCetResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitLiquidationCetResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgSubmitLiquidationCetResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitLiquidationCetSignatures {
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
        if !self.loan_id.is_empty() {
            len += 1;
        }
        if !self.signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.MsgSubmitLiquidationCetSignatures", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitLiquidationCetSignatures {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["sender", "loan_id", "loanId", "signatures"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            LoanId,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
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
            type Value = MsgSubmitLiquidationCetSignatures;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgSubmitLiquidationCetSignatures")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitLiquidationCetSignatures, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut loan_id__ = None;
                let mut signatures__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitLiquidationCetSignatures {
                    sender: sender__.unwrap_or_default(),
                    loan_id: loan_id__.unwrap_or_default(),
                    signatures: signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgSubmitLiquidationCetSignatures",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitLiquidationCetSignaturesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "side.lending.MsgSubmitLiquidationCetSignaturesResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitLiquidationCetSignaturesResponse {
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
            type Value = MsgSubmitLiquidationCetSignaturesResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgSubmitLiquidationCetSignaturesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitLiquidationCetSignaturesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitLiquidationCetSignaturesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgSubmitLiquidationCetSignaturesResponse",
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
impl serde::Serialize for MsgSubmitRepaymentAdaptorSignatures {
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
        if !self.loan_id.is_empty() {
            len += 1;
        }
        if !self.adaptor_signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.MsgSubmitRepaymentAdaptorSignatures", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.adaptor_signatures.is_empty() {
            struct_ser.serialize_field("adaptorSignatures", &self.adaptor_signatures)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitRepaymentAdaptorSignatures {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "loan_id",
            "loanId",
            "adaptor_signatures",
            "adaptorSignatures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            LoanId,
            AdaptorSignatures,
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
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
                            "adaptorSignatures" | "adaptor_signatures" => {
                                Ok(GeneratedField::AdaptorSignatures)
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
            type Value = MsgSubmitRepaymentAdaptorSignatures;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgSubmitRepaymentAdaptorSignatures")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitRepaymentAdaptorSignatures, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut loan_id__ = None;
                let mut adaptor_signatures__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AdaptorSignatures => {
                            if adaptor_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adaptorSignatures"));
                            }
                            adaptor_signatures__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSubmitRepaymentAdaptorSignatures {
                    sender: sender__.unwrap_or_default(),
                    loan_id: loan_id__.unwrap_or_default(),
                    adaptor_signatures: adaptor_signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgSubmitRepaymentAdaptorSignatures",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgSubmitRepaymentAdaptorSignaturesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct(
            "side.lending.MsgSubmitRepaymentAdaptorSignaturesResponse",
            len,
        )?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgSubmitRepaymentAdaptorSignaturesResponse {
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
            type Value = MsgSubmitRepaymentAdaptorSignaturesResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter
                    .write_str("struct side.lending.MsgSubmitRepaymentAdaptorSignaturesResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgSubmitRepaymentAdaptorSignaturesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitRepaymentAdaptorSignaturesResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.lending.MsgSubmitRepaymentAdaptorSignaturesResponse",
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
        if !self.origination_fee_collector.is_empty() {
            len += 1;
        }
        if !self.protocol_fee_collector.is_empty() {
            len += 1;
        }
        if self.final_timeout_duration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.Params", len)?;
        if !self.origination_fee_collector.is_empty() {
            struct_ser
                .serialize_field("originationFeeCollector", &self.origination_fee_collector)?;
        }
        if !self.protocol_fee_collector.is_empty() {
            struct_ser.serialize_field("protocolFeeCollector", &self.protocol_fee_collector)?;
        }
        if let Some(v) = self.final_timeout_duration.as_ref() {
            struct_ser.serialize_field("finalTimeoutDuration", v)?;
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
            "origination_fee_collector",
            "originationFeeCollector",
            "protocol_fee_collector",
            "protocolFeeCollector",
            "final_timeout_duration",
            "finalTimeoutDuration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OriginationFeeCollector,
            ProtocolFeeCollector,
            FinalTimeoutDuration,
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
                            "originationFeeCollector" | "origination_fee_collector" => {
                                Ok(GeneratedField::OriginationFeeCollector)
                            }
                            "protocolFeeCollector" | "protocol_fee_collector" => {
                                Ok(GeneratedField::ProtocolFeeCollector)
                            }
                            "finalTimeoutDuration" | "final_timeout_duration" => {
                                Ok(GeneratedField::FinalTimeoutDuration)
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
                let mut origination_fee_collector__ = None;
                let mut protocol_fee_collector__ = None;
                let mut final_timeout_duration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                        GeneratedField::FinalTimeoutDuration => {
                            if final_timeout_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "finalTimeoutDuration",
                                ));
                            }
                            final_timeout_duration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Params {
                    origination_fee_collector: origination_fee_collector__.unwrap_or_default(),
                    protocol_fee_collector: protocol_fee_collector__.unwrap_or_default(),
                    final_timeout_duration: final_timeout_duration__,
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
        if self.supply_rate != 0 {
            len += 1;
        }
        if self.borrow_rate != 0 {
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
        if !self.origination_fee.is_empty() {
            len += 1;
        }
        if self.ltv != 0 {
            len += 1;
        }
        if self.liquidation_threshold != 0 {
            len += 1;
        }
        if self.liquidation_penalty != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.PoolConfig", len)?;
        if self.supply_rate != 0 {
            struct_ser.serialize_field("supplyRate", &self.supply_rate)?;
        }
        if self.borrow_rate != 0 {
            struct_ser.serialize_field("borrowRate", &self.borrow_rate)?;
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
        if !self.origination_fee.is_empty() {
            struct_ser.serialize_field("originationFee", &self.origination_fee)?;
        }
        if self.ltv != 0 {
            struct_ser.serialize_field("ltv", &self.ltv)?;
        }
        if self.liquidation_threshold != 0 {
            struct_ser.serialize_field("liquidationThreshold", &self.liquidation_threshold)?;
        }
        if self.liquidation_penalty != 0 {
            struct_ser.serialize_field("liquidationPenalty", &self.liquidation_penalty)?;
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
            "supply_rate",
            "supplyRate",
            "borrow_rate",
            "borrowRate",
            "reserve_factor",
            "reserveFactor",
            "supply_cap",
            "supplyCap",
            "borrow_cap",
            "borrowCap",
            "debt_ceiling",
            "debtCeiling",
            "origination_fee",
            "originationFee",
            "ltv",
            "liquidation_threshold",
            "liquidationThreshold",
            "liquidation_penalty",
            "liquidationPenalty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SupplyRate,
            BorrowRate,
            ReserveFactor,
            SupplyCap,
            BorrowCap,
            DebtCeiling,
            OriginationFee,
            Ltv,
            LiquidationThreshold,
            LiquidationPenalty,
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
                            "supplyRate" | "supply_rate" => Ok(GeneratedField::SupplyRate),
                            "borrowRate" | "borrow_rate" => Ok(GeneratedField::BorrowRate),
                            "reserveFactor" | "reserve_factor" => Ok(GeneratedField::ReserveFactor),
                            "supplyCap" | "supply_cap" => Ok(GeneratedField::SupplyCap),
                            "borrowCap" | "borrow_cap" => Ok(GeneratedField::BorrowCap),
                            "debtCeiling" | "debt_ceiling" => Ok(GeneratedField::DebtCeiling),
                            "originationFee" | "origination_fee" => {
                                Ok(GeneratedField::OriginationFee)
                            }
                            "ltv" => Ok(GeneratedField::Ltv),
                            "liquidationThreshold" | "liquidation_threshold" => {
                                Ok(GeneratedField::LiquidationThreshold)
                            }
                            "liquidationPenalty" | "liquidation_penalty" => {
                                Ok(GeneratedField::LiquidationPenalty)
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
            type Value = PoolConfig;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.PoolConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<PoolConfig, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut supply_rate__ = None;
                let mut borrow_rate__ = None;
                let mut reserve_factor__ = None;
                let mut supply_cap__ = None;
                let mut borrow_cap__ = None;
                let mut debt_ceiling__ = None;
                let mut origination_fee__ = None;
                let mut ltv__ = None;
                let mut liquidation_threshold__ = None;
                let mut liquidation_penalty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SupplyRate => {
                            if supply_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supplyRate"));
                            }
                            supply_rate__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BorrowRate => {
                            if borrow_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowRate"));
                            }
                            borrow_rate__ = Some(
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
                        GeneratedField::OriginationFee => {
                            if origination_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originationFee"));
                            }
                            origination_fee__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ltv => {
                            if ltv__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ltv"));
                            }
                            ltv__ = Some(
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
                        GeneratedField::LiquidationPenalty => {
                            if liquidation_penalty__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationPenalty",
                                ));
                            }
                            liquidation_penalty__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                    }
                }
                Ok(PoolConfig {
                    supply_rate: supply_rate__.unwrap_or_default(),
                    borrow_rate: borrow_rate__.unwrap_or_default(),
                    reserve_factor: reserve_factor__.unwrap_or_default(),
                    supply_cap: supply_cap__.unwrap_or_default(),
                    borrow_cap: borrow_cap__.unwrap_or_default(),
                    debt_ceiling: debt_ceiling__.unwrap_or_default(),
                    origination_fee: origination_fee__.unwrap_or_default(),
                    ltv: ltv__.unwrap_or_default(),
                    liquidation_threshold: liquidation_threshold__.unwrap_or_default(),
                    liquidation_penalty: liquidation_penalty__.unwrap_or_default(),
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
        const FIELDS: &[&str] = &["INACTIVE", "ACTIVE"];

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
        if !self.agency_pubkey.is_empty() {
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
        if !self.agency_pubkey.is_empty() {
            struct_ser.serialize_field("agencyPubkey", &self.agency_pubkey)?;
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
            "agency_pubkey",
            "agencyPubkey",
            "maturity_time",
            "maturityTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BorrowerPubkey,
            AgencyPubkey,
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
                            "agencyPubkey" | "agency_pubkey" => Ok(GeneratedField::AgencyPubkey),
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
                let mut agency_pubkey__ = None;
                let mut maturity_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BorrowerPubkey => {
                            if borrower_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowerPubkey"));
                            }
                            borrower_pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AgencyPubkey => {
                            if agency_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agencyPubkey"));
                            }
                            agency_pubkey__ = Some(map_.next_value()?);
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
                    agency_pubkey: agency_pubkey__.unwrap_or_default(),
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
impl serde::Serialize for QueryLiquidationCetRequest {
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
        if !self.borrower_pubkey.is_empty() {
            len += 1;
        }
        if !self.agency_pubkey.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLiquidationCetRequest", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.borrower_pubkey.is_empty() {
            struct_ser.serialize_field("borrowerPubkey", &self.borrower_pubkey)?;
        }
        if !self.agency_pubkey.is_empty() {
            struct_ser.serialize_field("agencyPubkey", &self.agency_pubkey)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLiquidationCetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "loan_id",
            "loanId",
            "borrower_pubkey",
            "borrowerPubkey",
            "agency_pubkey",
            "agencyPubkey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoanId,
            BorrowerPubkey,
            AgencyPubkey,
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
                            "borrowerPubkey" | "borrower_pubkey" => {
                                Ok(GeneratedField::BorrowerPubkey)
                            }
                            "agencyPubkey" | "agency_pubkey" => Ok(GeneratedField::AgencyPubkey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLiquidationCetRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLiquidationCetRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLiquidationCetRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut loan_id__ = None;
                let mut borrower_pubkey__ = None;
                let mut agency_pubkey__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowerPubkey => {
                            if borrower_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowerPubkey"));
                            }
                            borrower_pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AgencyPubkey => {
                            if agency_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agencyPubkey"));
                            }
                            agency_pubkey__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLiquidationCetRequest {
                    loan_id: loan_id__.unwrap_or_default(),
                    borrower_pubkey: borrower_pubkey__.unwrap_or_default(),
                    agency_pubkey: agency_pubkey__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLiquidationCetRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLiquidationCetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.script.is_empty() {
            len += 1;
        }
        if !self.sig_hashes.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLiquidationCetResponse", len)?;
        if !self.script.is_empty() {
            struct_ser.serialize_field("script", &self.script)?;
        }
        if !self.sig_hashes.is_empty() {
            struct_ser.serialize_field("sigHashes", &self.sig_hashes)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLiquidationCetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["script", "sig_hashes", "sigHashes"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Script,
            SigHashes,
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
                            "script" => Ok(GeneratedField::Script),
                            "sigHashes" | "sig_hashes" => Ok(GeneratedField::SigHashes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLiquidationCetResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLiquidationCetResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLiquidationCetResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut script__ = None;
                let mut sig_hashes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Script => {
                            if script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            script__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SigHashes => {
                            if sig_hashes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sigHashes"));
                            }
                            sig_hashes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryLiquidationCetResponse {
                    script: script__.unwrap_or_default(),
                    sig_hashes: sig_hashes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLiquidationCetResponse",
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PoolId,
            CollateralAmount,
            BorrowAmount,
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
                    }
                }
                Ok(QueryLiquidationEventRequest {
                    pool_id: pool_id__.unwrap_or_default(),
                    collateral_amount: collateral_amount__.unwrap_or_default(),
                    borrow_amount: borrow_amount__.unwrap_or_default(),
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
        if !self.loan_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.QueryLoanRequest", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
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
            type Value = QueryLoanRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoanRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<QueryLoanRequest, V::Error>
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
                Ok(QueryLoanRequest {
                    loan_id: loan_id__.unwrap_or_default(),
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
        if !self.txid.is_empty() {
            len += 1;
        }
        if !self.tx.is_empty() {
            len += 1;
        }
        if !self.adaptor_point.is_empty() {
            len += 1;
        }
        if !self.dca_adaptor_signatures.is_empty() {
            len += 1;
        }
        if !self.dca_adapted_signature.is_empty() {
            len += 1;
        }
        if self.create_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.Repayment", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.txid.is_empty() {
            struct_ser.serialize_field("txid", &self.txid)?;
        }
        if !self.tx.is_empty() {
            struct_ser.serialize_field("tx", &self.tx)?;
        }
        if !self.adaptor_point.is_empty() {
            struct_ser.serialize_field("adaptorPoint", &self.adaptor_point)?;
        }
        if !self.dca_adaptor_signatures.is_empty() {
            struct_ser.serialize_field("dcaAdaptorSignatures", &self.dca_adaptor_signatures)?;
        }
        if !self.dca_adapted_signature.is_empty() {
            struct_ser.serialize_field("dcaAdaptedSignature", &self.dca_adapted_signature)?;
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
        const FIELDS: &[&str] = &[
            "loan_id",
            "loanId",
            "txid",
            "tx",
            "adaptor_point",
            "adaptorPoint",
            "dca_adaptor_signatures",
            "dcaAdaptorSignatures",
            "dca_adapted_signature",
            "dcaAdaptedSignature",
            "create_at",
            "createAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoanId,
            Txid,
            Tx,
            AdaptorPoint,
            DcaAdaptorSignatures,
            DcaAdaptedSignature,
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
                            "adaptorPoint" | "adaptor_point" => Ok(GeneratedField::AdaptorPoint),
                            "dcaAdaptorSignatures" | "dca_adaptor_signatures" => {
                                Ok(GeneratedField::DcaAdaptorSignatures)
                            }
                            "dcaAdaptedSignature" | "dca_adapted_signature" => {
                                Ok(GeneratedField::DcaAdaptedSignature)
                            }
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
                let mut txid__ = None;
                let mut tx__ = None;
                let mut adaptor_point__ = None;
                let mut dca_adaptor_signatures__ = None;
                let mut dca_adapted_signature__ = None;
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
                        GeneratedField::AdaptorPoint => {
                            if adaptor_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adaptorPoint"));
                            }
                            adaptor_point__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DcaAdaptorSignatures => {
                            if dca_adaptor_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "dcaAdaptorSignatures",
                                ));
                            }
                            dca_adaptor_signatures__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DcaAdaptedSignature => {
                            if dca_adapted_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "dcaAdaptedSignature",
                                ));
                            }
                            dca_adapted_signature__ = Some(map_.next_value()?);
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
                    txid: txid__.unwrap_or_default(),
                    tx: tx__.unwrap_or_default(),
                    adaptor_point: adaptor_point__.unwrap_or_default(),
                    dca_adaptor_signatures: dca_adaptor_signatures__.unwrap_or_default(),
                    dca_adapted_signature: dca_adapted_signature__.unwrap_or_default(),
                    create_at: create_at__,
                })
            }
        }
        deserializer.deserialize_struct("side.lending.Repayment", FIELDS, GeneratedVisitor)
    }
}
