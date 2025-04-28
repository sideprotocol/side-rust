// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for AssetType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Bitcoin => "ASSET_TYPE_BITCOIN",
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
        const FIELDS: &[&str] = &["ASSET_TYPE_BITCOIN"];

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
                    "ASSET_TYPE_BITCOIN" => Ok(AssetType::Bitcoin),
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
        if !self.liquidations.is_empty() {
            len += 1;
        }
        if !self.liquidation_records.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.liquidation.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.liquidations.is_empty() {
            struct_ser.serialize_field("liquidations", &self.liquidations)?;
        }
        if !self.liquidation_records.is_empty() {
            struct_ser.serialize_field("liquidationRecords", &self.liquidation_records)?;
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
        const FIELDS: &[&str] = &["params", "liquidations", "liquidationRecords"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            Liquidations,
            LiquidationRecords,
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
                            "liquidations" => Ok(GeneratedField::Liquidations),
                            "liquidationRecords" => Ok(GeneratedField::LiquidationRecords),
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
                formatter.write_str("struct side.liquidation.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut liquidations__ = None;
                let mut liquidation_records__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::Liquidations => {
                            if liquidations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidations"));
                            }
                            liquidations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LiquidationRecords => {
                            if liquidation_records__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationRecords",
                                ));
                            }
                            liquidation_records__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    liquidations: liquidations__.unwrap_or_default(),
                    liquidation_records: liquidation_records__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.liquidation.GenesisState", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for Liquidation {
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
        if !self.loan_id.is_empty() {
            len += 1;
        }
        if !self.debtor.is_empty() {
            len += 1;
        }
        if !self.dcm.is_empty() {
            len += 1;
        }
        if self.collateral_amount.is_some() {
            len += 1;
        }
        if self.actual_collateral_amount.is_some() {
            len += 1;
        }
        if self.debt_amount.is_some() {
            len += 1;
        }
        if !self.liquidated_price.is_empty() {
            len += 1;
        }
        if self.liquidated_time.is_some() {
            len += 1;
        }
        if self.liquidated_collateral_amount.is_some() {
            len += 1;
        }
        if self.liquidated_debt_amount.is_some() {
            len += 1;
        }
        if self.liquidation_bonus_amount.is_some() {
            len += 1;
        }
        if self.protocol_liquidation_fee.is_some() {
            len += 1;
        }
        if self.unliquidated_collateral_amount.is_some() {
            len += 1;
        }
        if !self.liquidation_cet.is_empty() {
            len += 1;
        }
        if !self.settlement_tx.is_empty() {
            len += 1;
        }
        if !self.settlement_tx_id.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.liquidation.Liquidation", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.debtor.is_empty() {
            struct_ser.serialize_field("debtor", &self.debtor)?;
        }
        if !self.dcm.is_empty() {
            struct_ser.serialize_field("dcm", &self.dcm)?;
        }
        if let Some(v) = self.collateral_amount.as_ref() {
            struct_ser.serialize_field("collateralAmount", v)?;
        }
        if let Some(v) = self.actual_collateral_amount.as_ref() {
            struct_ser.serialize_field("actualCollateralAmount", v)?;
        }
        if let Some(v) = self.debt_amount.as_ref() {
            struct_ser.serialize_field("debtAmount", v)?;
        }
        if !self.liquidated_price.is_empty() {
            struct_ser.serialize_field("liquidatedPrice", &self.liquidated_price)?;
        }
        if let Some(v) = self.liquidated_time.as_ref() {
            struct_ser.serialize_field("liquidatedTime", v)?;
        }
        if let Some(v) = self.liquidated_collateral_amount.as_ref() {
            struct_ser.serialize_field("liquidatedCollateralAmount", v)?;
        }
        if let Some(v) = self.liquidated_debt_amount.as_ref() {
            struct_ser.serialize_field("liquidatedDebtAmount", v)?;
        }
        if let Some(v) = self.liquidation_bonus_amount.as_ref() {
            struct_ser.serialize_field("liquidationBonusAmount", v)?;
        }
        if let Some(v) = self.protocol_liquidation_fee.as_ref() {
            struct_ser.serialize_field("protocolLiquidationFee", v)?;
        }
        if let Some(v) = self.unliquidated_collateral_amount.as_ref() {
            struct_ser.serialize_field("unliquidatedCollateralAmount", v)?;
        }
        if !self.liquidation_cet.is_empty() {
            struct_ser.serialize_field("liquidationCet", &self.liquidation_cet)?;
        }
        if !self.settlement_tx.is_empty() {
            struct_ser.serialize_field("settlementTx", &self.settlement_tx)?;
        }
        if !self.settlement_tx_id.is_empty() {
            struct_ser.serialize_field("settlementTxId", &self.settlement_tx_id)?;
        }
        if self.status != 0 {
            let v = LiquidationStatus::try_from(self.status).map_err(|_| {
                serde::ser::Error::custom(alloc::format!("Invalid variant {}", self.status))
            })?;
            struct_ser.serialize_field("status", &v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Liquidation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "loan_id",
            "loanId",
            "debtor",
            "dcm",
            "collateral_amount",
            "collateralAmount",
            "actual_collateral_amount",
            "actualCollateralAmount",
            "debt_amount",
            "debtAmount",
            "liquidated_price",
            "liquidatedPrice",
            "liquidated_time",
            "liquidatedTime",
            "liquidated_collateral_amount",
            "liquidatedCollateralAmount",
            "liquidated_debt_amount",
            "liquidatedDebtAmount",
            "liquidation_bonus_amount",
            "liquidationBonusAmount",
            "protocol_liquidation_fee",
            "protocolLiquidationFee",
            "unliquidated_collateral_amount",
            "unliquidatedCollateralAmount",
            "liquidation_cet",
            "liquidationCet",
            "settlement_tx",
            "settlementTx",
            "settlement_tx_id",
            "settlementTxId",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            LoanId,
            Debtor,
            Dcm,
            CollateralAmount,
            ActualCollateralAmount,
            DebtAmount,
            LiquidatedPrice,
            LiquidatedTime,
            LiquidatedCollateralAmount,
            LiquidatedDebtAmount,
            LiquidationBonusAmount,
            ProtocolLiquidationFee,
            UnliquidatedCollateralAmount,
            LiquidationCet,
            SettlementTx,
            SettlementTxId,
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
                            "loanId" | "loan_id" => Ok(GeneratedField::LoanId),
                            "debtor" => Ok(GeneratedField::Debtor),
                            "dcm" => Ok(GeneratedField::Dcm),
                            "collateralAmount" | "collateral_amount" => {
                                Ok(GeneratedField::CollateralAmount)
                            }
                            "actualCollateralAmount" | "actual_collateral_amount" => {
                                Ok(GeneratedField::ActualCollateralAmount)
                            }
                            "debtAmount" | "debt_amount" => Ok(GeneratedField::DebtAmount),
                            "liquidatedPrice" | "liquidated_price" => {
                                Ok(GeneratedField::LiquidatedPrice)
                            }
                            "liquidatedTime" | "liquidated_time" => {
                                Ok(GeneratedField::LiquidatedTime)
                            }
                            "liquidatedCollateralAmount" | "liquidated_collateral_amount" => {
                                Ok(GeneratedField::LiquidatedCollateralAmount)
                            }
                            "liquidatedDebtAmount" | "liquidated_debt_amount" => {
                                Ok(GeneratedField::LiquidatedDebtAmount)
                            }
                            "liquidationBonusAmount" | "liquidation_bonus_amount" => {
                                Ok(GeneratedField::LiquidationBonusAmount)
                            }
                            "protocolLiquidationFee" | "protocol_liquidation_fee" => {
                                Ok(GeneratedField::ProtocolLiquidationFee)
                            }
                            "unliquidatedCollateralAmount" | "unliquidated_collateral_amount" => {
                                Ok(GeneratedField::UnliquidatedCollateralAmount)
                            }
                            "liquidationCet" | "liquidation_cet" => {
                                Ok(GeneratedField::LiquidationCet)
                            }
                            "settlementTx" | "settlement_tx" => Ok(GeneratedField::SettlementTx),
                            "settlementTxId" | "settlement_tx_id" => {
                                Ok(GeneratedField::SettlementTxId)
                            }
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
            type Value = Liquidation;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.liquidation.Liquidation")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Liquidation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut loan_id__ = None;
                let mut debtor__ = None;
                let mut dcm__ = None;
                let mut collateral_amount__ = None;
                let mut actual_collateral_amount__ = None;
                let mut debt_amount__ = None;
                let mut liquidated_price__ = None;
                let mut liquidated_time__ = None;
                let mut liquidated_collateral_amount__ = None;
                let mut liquidated_debt_amount__ = None;
                let mut liquidation_bonus_amount__ = None;
                let mut protocol_liquidation_fee__ = None;
                let mut unliquidated_collateral_amount__ = None;
                let mut liquidation_cet__ = None;
                let mut settlement_tx__ = None;
                let mut settlement_tx_id__ = None;
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
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Debtor => {
                            if debtor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debtor"));
                            }
                            debtor__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Dcm => {
                            if dcm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dcm"));
                            }
                            dcm__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CollateralAmount => {
                            if collateral_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collateralAmount"));
                            }
                            collateral_amount__ = map_.next_value()?;
                        }
                        GeneratedField::ActualCollateralAmount => {
                            if actual_collateral_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "actualCollateralAmount",
                                ));
                            }
                            actual_collateral_amount__ = map_.next_value()?;
                        }
                        GeneratedField::DebtAmount => {
                            if debt_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debtAmount"));
                            }
                            debt_amount__ = map_.next_value()?;
                        }
                        GeneratedField::LiquidatedPrice => {
                            if liquidated_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidatedPrice"));
                            }
                            liquidated_price__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LiquidatedTime => {
                            if liquidated_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidatedTime"));
                            }
                            liquidated_time__ = map_.next_value()?;
                        }
                        GeneratedField::LiquidatedCollateralAmount => {
                            if liquidated_collateral_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidatedCollateralAmount",
                                ));
                            }
                            liquidated_collateral_amount__ = map_.next_value()?;
                        }
                        GeneratedField::LiquidatedDebtAmount => {
                            if liquidated_debt_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidatedDebtAmount",
                                ));
                            }
                            liquidated_debt_amount__ = map_.next_value()?;
                        }
                        GeneratedField::LiquidationBonusAmount => {
                            if liquidation_bonus_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationBonusAmount",
                                ));
                            }
                            liquidation_bonus_amount__ = map_.next_value()?;
                        }
                        GeneratedField::ProtocolLiquidationFee => {
                            if protocol_liquidation_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "protocolLiquidationFee",
                                ));
                            }
                            protocol_liquidation_fee__ = map_.next_value()?;
                        }
                        GeneratedField::UnliquidatedCollateralAmount => {
                            if unliquidated_collateral_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "unliquidatedCollateralAmount",
                                ));
                            }
                            unliquidated_collateral_amount__ = map_.next_value()?;
                        }
                        GeneratedField::LiquidationCet => {
                            if liquidation_cet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidationCet"));
                            }
                            liquidation_cet__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SettlementTx => {
                            if settlement_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settlementTx"));
                            }
                            settlement_tx__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SettlementTxId => {
                            if settlement_tx_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settlementTxId"));
                            }
                            settlement_tx_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<LiquidationStatus>()? as i32);
                        }
                    }
                }
                Ok(Liquidation {
                    id: id__.unwrap_or_default(),
                    loan_id: loan_id__.unwrap_or_default(),
                    debtor: debtor__.unwrap_or_default(),
                    dcm: dcm__.unwrap_or_default(),
                    collateral_amount: collateral_amount__,
                    actual_collateral_amount: actual_collateral_amount__,
                    debt_amount: debt_amount__,
                    liquidated_price: liquidated_price__.unwrap_or_default(),
                    liquidated_time: liquidated_time__,
                    liquidated_collateral_amount: liquidated_collateral_amount__,
                    liquidated_debt_amount: liquidated_debt_amount__,
                    liquidation_bonus_amount: liquidation_bonus_amount__,
                    protocol_liquidation_fee: protocol_liquidation_fee__,
                    unliquidated_collateral_amount: unliquidated_collateral_amount__,
                    liquidation_cet: liquidation_cet__.unwrap_or_default(),
                    settlement_tx: settlement_tx__.unwrap_or_default(),
                    settlement_tx_id: settlement_tx_id__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.liquidation.Liquidation", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for LiquidationRecord {
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
        if self.liquidation_id != 0 {
            len += 1;
        }
        if !self.liquidator.is_empty() {
            len += 1;
        }
        if self.debt_amount.is_some() {
            len += 1;
        }
        if self.collateral_amount.is_some() {
            len += 1;
        }
        if self.bonus_amount.is_some() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.liquidation.LiquidationRecord", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        if self.liquidation_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "liquidationId",
                alloc::string::ToString::to_string(&self.liquidation_id).as_str(),
            )?;
        }
        if !self.liquidator.is_empty() {
            struct_ser.serialize_field("liquidator", &self.liquidator)?;
        }
        if let Some(v) = self.debt_amount.as_ref() {
            struct_ser.serialize_field("debtAmount", v)?;
        }
        if let Some(v) = self.collateral_amount.as_ref() {
            struct_ser.serialize_field("collateralAmount", v)?;
        }
        if let Some(v) = self.bonus_amount.as_ref() {
            struct_ser.serialize_field("bonusAmount", v)?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LiquidationRecord {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "liquidation_id",
            "liquidationId",
            "liquidator",
            "debt_amount",
            "debtAmount",
            "collateral_amount",
            "collateralAmount",
            "bonus_amount",
            "bonusAmount",
            "time",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            LiquidationId,
            Liquidator,
            DebtAmount,
            CollateralAmount,
            BonusAmount,
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
                            "liquidationId" | "liquidation_id" => Ok(GeneratedField::LiquidationId),
                            "liquidator" => Ok(GeneratedField::Liquidator),
                            "debtAmount" | "debt_amount" => Ok(GeneratedField::DebtAmount),
                            "collateralAmount" | "collateral_amount" => {
                                Ok(GeneratedField::CollateralAmount)
                            }
                            "bonusAmount" | "bonus_amount" => Ok(GeneratedField::BonusAmount),
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
            type Value = LiquidationRecord;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.liquidation.LiquidationRecord")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<LiquidationRecord, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut liquidation_id__ = None;
                let mut liquidator__ = None;
                let mut debt_amount__ = None;
                let mut collateral_amount__ = None;
                let mut bonus_amount__ = None;
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
                        GeneratedField::LiquidationId => {
                            if liquidation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidationId"));
                            }
                            liquidation_id__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Liquidator => {
                            if liquidator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidator"));
                            }
                            liquidator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DebtAmount => {
                            if debt_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debtAmount"));
                            }
                            debt_amount__ = map_.next_value()?;
                        }
                        GeneratedField::CollateralAmount => {
                            if collateral_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collateralAmount"));
                            }
                            collateral_amount__ = map_.next_value()?;
                        }
                        GeneratedField::BonusAmount => {
                            if bonus_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bonusAmount"));
                            }
                            bonus_amount__ = map_.next_value()?;
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LiquidationRecord {
                    id: id__.unwrap_or_default(),
                    liquidation_id: liquidation_id__.unwrap_or_default(),
                    liquidator: liquidator__.unwrap_or_default(),
                    debt_amount: debt_amount__,
                    collateral_amount: collateral_amount__,
                    bonus_amount: bonus_amount__,
                    time: time__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.liquidation.LiquidationRecord",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for LiquidationStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "LIQUIDATION_STATUS_UNSPECIFIED",
            Self::Liquidating => "LIQUIDATION_STATUS_LIQUIDATING",
            Self::Liquidated => "LIQUIDATION_STATUS_LIQUIDATED",
            Self::Settling => "LIQUIDATION_STATUS_SETTLING",
            Self::Settled => "LIQUIDATION_STATUS_SETTLED",
        };
        serializer.serialize_str(variant)
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LiquidationStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LIQUIDATION_STATUS_UNSPECIFIED",
            "LIQUIDATION_STATUS_LIQUIDATING",
            "LIQUIDATION_STATUS_LIQUIDATED",
            "LIQUIDATION_STATUS_SETTLING",
            "LIQUIDATION_STATUS_SETTLED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LiquidationStatus;

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
                    "LIQUIDATION_STATUS_UNSPECIFIED" => Ok(LiquidationStatus::Unspecified),
                    "LIQUIDATION_STATUS_LIQUIDATING" => Ok(LiquidationStatus::Liquidating),
                    "LIQUIDATION_STATUS_LIQUIDATED" => Ok(LiquidationStatus::Liquidated),
                    "LIQUIDATION_STATUS_SETTLING" => Ok(LiquidationStatus::Settling),
                    "LIQUIDATION_STATUS_SETTLED" => Ok(LiquidationStatus::Settled),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgLiquidate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.liquidator.is_empty() {
            len += 1;
        }
        if self.liquidation_id != 0 {
            len += 1;
        }
        if self.debt_amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.liquidation.MsgLiquidate", len)?;
        if !self.liquidator.is_empty() {
            struct_ser.serialize_field("liquidator", &self.liquidator)?;
        }
        if self.liquidation_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "liquidationId",
                alloc::string::ToString::to_string(&self.liquidation_id).as_str(),
            )?;
        }
        if let Some(v) = self.debt_amount.as_ref() {
            struct_ser.serialize_field("debtAmount", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgLiquidate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "liquidator",
            "liquidation_id",
            "liquidationId",
            "debt_amount",
            "debtAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Liquidator,
            LiquidationId,
            DebtAmount,
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
                            "liquidator" => Ok(GeneratedField::Liquidator),
                            "liquidationId" | "liquidation_id" => Ok(GeneratedField::LiquidationId),
                            "debtAmount" | "debt_amount" => Ok(GeneratedField::DebtAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgLiquidate;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.liquidation.MsgLiquidate")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgLiquidate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut liquidator__ = None;
                let mut liquidation_id__ = None;
                let mut debt_amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Liquidator => {
                            if liquidator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidator"));
                            }
                            liquidator__ = Some(map_.next_value()?);
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
                        GeneratedField::DebtAmount => {
                            if debt_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debtAmount"));
                            }
                            debt_amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgLiquidate {
                    liquidator: liquidator__.unwrap_or_default(),
                    liquidation_id: liquidation_id__.unwrap_or_default(),
                    debt_amount: debt_amount__,
                })
            }
        }
        deserializer.deserialize_struct("side.liquidation.MsgLiquidate", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgLiquidateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser =
            serializer.serialize_struct("side.liquidation.MsgLiquidateResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgLiquidateResponse {
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
            type Value = MsgLiquidateResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.liquidation.MsgLiquidateResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<MsgLiquidateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgLiquidateResponse {})
            }
        }
        deserializer.deserialize_struct(
            "side.liquidation.MsgLiquidateResponse",
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
        let mut struct_ser =
            serializer.serialize_struct("side.liquidation.MsgUpdateParams", len)?;
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
                formatter.write_str("struct side.liquidation.MsgUpdateParams")
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
        deserializer.deserialize_struct(
            "side.liquidation.MsgUpdateParams",
            FIELDS,
            GeneratedVisitor,
        )
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
            serializer.serialize_struct("side.liquidation.MsgUpdateParamsResponse", len)?;
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
                formatter.write_str("struct side.liquidation.MsgUpdateParamsResponse")
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
            "side.liquidation.MsgUpdateParamsResponse",
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
        if self.min_liquidation_factor != 0 {
            len += 1;
        }
        if self.liquidation_bonus_factor != 0 {
            len += 1;
        }
        if self.protocol_liquidation_fee_factor != 0 {
            len += 1;
        }
        if !self.protocol_liquidation_fee_collector.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.liquidation.Params", len)?;
        if self.min_liquidation_factor != 0 {
            struct_ser.serialize_field("minLiquidationFactor", &self.min_liquidation_factor)?;
        }
        if self.liquidation_bonus_factor != 0 {
            struct_ser.serialize_field("liquidationBonusFactor", &self.liquidation_bonus_factor)?;
        }
        if self.protocol_liquidation_fee_factor != 0 {
            struct_ser.serialize_field(
                "protocolLiquidationFeeFactor",
                &self.protocol_liquidation_fee_factor,
            )?;
        }
        if !self.protocol_liquidation_fee_collector.is_empty() {
            struct_ser.serialize_field(
                "protocolLiquidationFeeCollector",
                &self.protocol_liquidation_fee_collector,
            )?;
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
            "min_liquidation_factor",
            "minLiquidationFactor",
            "liquidation_bonus_factor",
            "liquidationBonusFactor",
            "protocol_liquidation_fee_factor",
            "protocolLiquidationFeeFactor",
            "protocol_liquidation_fee_collector",
            "protocolLiquidationFeeCollector",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinLiquidationFactor,
            LiquidationBonusFactor,
            ProtocolLiquidationFeeFactor,
            ProtocolLiquidationFeeCollector,
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
                            "minLiquidationFactor" | "min_liquidation_factor" => {
                                Ok(GeneratedField::MinLiquidationFactor)
                            }
                            "liquidationBonusFactor" | "liquidation_bonus_factor" => {
                                Ok(GeneratedField::LiquidationBonusFactor)
                            }
                            "protocolLiquidationFeeFactor" | "protocol_liquidation_fee_factor" => {
                                Ok(GeneratedField::ProtocolLiquidationFeeFactor)
                            }
                            "protocolLiquidationFeeCollector"
                            | "protocol_liquidation_fee_collector" => {
                                Ok(GeneratedField::ProtocolLiquidationFeeCollector)
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
                formatter.write_str("struct side.liquidation.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut min_liquidation_factor__ = None;
                let mut liquidation_bonus_factor__ = None;
                let mut protocol_liquidation_fee_factor__ = None;
                let mut protocol_liquidation_fee_collector__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinLiquidationFactor => {
                            if min_liquidation_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "minLiquidationFactor",
                                ));
                            }
                            min_liquidation_factor__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LiquidationBonusFactor => {
                            if liquidation_bonus_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationBonusFactor",
                                ));
                            }
                            liquidation_bonus_factor__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ProtocolLiquidationFeeFactor => {
                            if protocol_liquidation_fee_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "protocolLiquidationFeeFactor",
                                ));
                            }
                            protocol_liquidation_fee_factor__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ProtocolLiquidationFeeCollector => {
                            if protocol_liquidation_fee_collector__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "protocolLiquidationFeeCollector",
                                ));
                            }
                            protocol_liquidation_fee_collector__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    min_liquidation_factor: min_liquidation_factor__.unwrap_or_default(),
                    liquidation_bonus_factor: liquidation_bonus_factor__.unwrap_or_default(),
                    protocol_liquidation_fee_factor: protocol_liquidation_fee_factor__
                        .unwrap_or_default(),
                    protocol_liquidation_fee_collector: protocol_liquidation_fee_collector__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.liquidation.Params", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLiquidationRecordRequest {
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
            serializer.serialize_struct("side.liquidation.QueryLiquidationRecordRequest", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLiquidationRecordRequest {
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
            type Value = QueryLiquidationRecordRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.liquidation.QueryLiquidationRecordRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLiquidationRecordRequest, V::Error>
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
                Ok(QueryLiquidationRecordRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.liquidation.QueryLiquidationRecordRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLiquidationRecordResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.liquidation_record.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.liquidation.QueryLiquidationRecordResponse", len)?;
        if let Some(v) = self.liquidation_record.as_ref() {
            struct_ser.serialize_field("liquidationRecord", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLiquidationRecordResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["liquidation_record", "liquidationRecord"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LiquidationRecord,
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
                            "liquidationRecord" | "liquidation_record" => {
                                Ok(GeneratedField::LiquidationRecord)
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
            type Value = QueryLiquidationRecordResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.liquidation.QueryLiquidationRecordResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLiquidationRecordResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut liquidation_record__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LiquidationRecord => {
                            if liquidation_record__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidationRecord"));
                            }
                            liquidation_record__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLiquidationRecordResponse {
                    liquidation_record: liquidation_record__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.liquidation.QueryLiquidationRecordResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLiquidationRecordsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.liquidation_id != 0 {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.liquidation.QueryLiquidationRecordsRequest", len)?;
        if self.liquidation_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "liquidationId",
                alloc::string::ToString::to_string(&self.liquidation_id).as_str(),
            )?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLiquidationRecordsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["liquidation_id", "liquidationId", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LiquidationId,
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
                            "liquidationId" | "liquidation_id" => Ok(GeneratedField::LiquidationId),
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
            type Value = QueryLiquidationRecordsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.liquidation.QueryLiquidationRecordsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLiquidationRecordsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut liquidation_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LiquidationId => {
                            if liquidation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidationId"));
                            }
                            liquidation_id__ = Some(
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
                Ok(QueryLiquidationRecordsRequest {
                    liquidation_id: liquidation_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.liquidation.QueryLiquidationRecordsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLiquidationRecordsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.liquidation_records.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.liquidation.QueryLiquidationRecordsResponse", len)?;
        if !self.liquidation_records.is_empty() {
            struct_ser.serialize_field("liquidationRecords", &self.liquidation_records)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLiquidationRecordsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["liquidation_records", "liquidationRecords", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LiquidationRecords,
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
                            "liquidationRecords" | "liquidation_records" => {
                                Ok(GeneratedField::LiquidationRecords)
                            }
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
            type Value = QueryLiquidationRecordsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.liquidation.QueryLiquidationRecordsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLiquidationRecordsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut liquidation_records__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LiquidationRecords => {
                            if liquidation_records__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationRecords",
                                ));
                            }
                            liquidation_records__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLiquidationRecordsResponse {
                    liquidation_records: liquidation_records__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.liquidation.QueryLiquidationRecordsResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLiquidationRequest {
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
            serializer.serialize_struct("side.liquidation.QueryLiquidationRequest", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser
                .serialize_field("id", alloc::string::ToString::to_string(&self.id).as_str())?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLiquidationRequest {
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
            type Value = QueryLiquidationRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.liquidation.QueryLiquidationRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLiquidationRequest, V::Error>
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
                Ok(QueryLiquidationRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.liquidation.QueryLiquidationRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLiquidationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.liquidation.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.liquidation.QueryLiquidationResponse", len)?;
        if let Some(v) = self.liquidation.as_ref() {
            struct_ser.serialize_field("liquidation", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLiquidationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["liquidation"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Liquidation,
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
                            "liquidation" => Ok(GeneratedField::Liquidation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLiquidationResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.liquidation.QueryLiquidationResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLiquidationResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut liquidation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Liquidation => {
                            if liquidation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidation"));
                            }
                            liquidation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLiquidationResponse {
                    liquidation: liquidation__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.liquidation.QueryLiquidationResponse",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLiquidationsRequest {
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
        let mut struct_ser =
            serializer.serialize_struct("side.liquidation.QueryLiquidationsRequest", len)?;
        if self.status != 0 {
            let v = LiquidationStatus::try_from(self.status).map_err(|_| {
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
impl<'de> serde::Deserialize<'de> for QueryLiquidationsRequest {
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
            type Value = QueryLiquidationsRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.liquidation.QueryLiquidationsRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLiquidationsRequest, V::Error>
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
                            status__ = Some(map_.next_value::<LiquidationStatus>()? as i32);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLiquidationsRequest {
                    status: status__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.liquidation.QueryLiquidationsRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLiquidationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.liquidations.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.liquidation.QueryLiquidationsResponse", len)?;
        if !self.liquidations.is_empty() {
            struct_ser.serialize_field("liquidations", &self.liquidations)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLiquidationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["liquidations", "pagination"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Liquidations,
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
                            "liquidations" => Ok(GeneratedField::Liquidations),
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
            type Value = QueryLiquidationsResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.liquidation.QueryLiquidationsResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLiquidationsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut liquidations__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Liquidations => {
                            if liquidations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("liquidations"));
                            }
                            liquidations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLiquidationsResponse {
                    liquidations: liquidations__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct(
            "side.liquidation.QueryLiquidationsResponse",
            FIELDS,
            GeneratedVisitor,
        )
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
        let struct_ser = serializer.serialize_struct("side.liquidation.QueryParamsRequest", len)?;
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
                formatter.write_str("struct side.liquidation.QueryParamsRequest")
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
        deserializer.deserialize_struct(
            "side.liquidation.QueryParamsRequest",
            FIELDS,
            GeneratedVisitor,
        )
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
            serializer.serialize_struct("side.liquidation.QueryParamsResponse", len)?;
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
                formatter.write_str("struct side.liquidation.QueryParamsResponse")
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
            "side.liquidation.QueryParamsResponse",
            FIELDS,
            GeneratedVisitor,
        )
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
            Self::Default => "SIGNING_INTENT_DEFAULT",
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
        const FIELDS: &[&str] = &["SIGNING_INTENT_DEFAULT"];

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
                    "SIGNING_INTENT_DEFAULT" => Ok(SigningIntent::Default),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
