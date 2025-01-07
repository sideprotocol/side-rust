// @generated
#[cfg(feature = "serde")]
impl serde::Serialize for Cet {
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
        if !self.cets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.CET", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.cets.is_empty() {
            struct_ser.serialize_field("cets", &self.cets)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Cet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["loan_id", "loanId", "cets"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoanId,
            Cets,
        }
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
                            "cets" => Ok(GeneratedField::Cets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Cet;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.CET")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<Cet, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut loan_id__ = None;
                let mut cets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoanId => {
                            if loan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanId"));
                            }
                            loan_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cets => {
                            if cets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cets"));
                            }
                            cets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Cet {
                    loan_id: loan_id__.unwrap_or_default(),
                    cets: cets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.CET", FIELDS, GeneratedVisitor)
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
        if !self.total_shares.is_empty() {
            len += 1;
        }
        if !self.borrowed_amount.is_empty() {
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
        if !self.total_shares.is_empty() {
            struct_ser.serialize_field("totalShares", &self.total_shares)?;
        }
        if !self.borrowed_amount.is_empty() {
            struct_ser.serialize_field("borrowedAmount", &self.borrowed_amount)?;
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
            "total_shares",
            "totalShares",
            "borrowed_amount",
            "borrowedAmount",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Supply,
            TotalShares,
            BorrowedAmount,
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
                            "totalShares" | "total_shares" => Ok(GeneratedField::TotalShares),
                            "borrowedAmount" | "borrowed_amount" => {
                                Ok(GeneratedField::BorrowedAmount)
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
                let mut total_shares__ = None;
                let mut borrowed_amount__ = None;
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
                        GeneratedField::TotalShares => {
                            if total_shares__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalShares"));
                            }
                            total_shares__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowedAmount => {
                            if borrowed_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowedAmount"));
                            }
                            borrowed_amount__ = Some(map_.next_value()?);
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
                    total_shares: total_shares__.unwrap_or_default(),
                    borrowed_amount: borrowed_amount__.unwrap_or_default(),
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
        if !self.agency.is_empty() {
            len += 1;
        }
        if !self.hash_loan_secret.is_empty() {
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
        if !self.fees.is_empty() {
            len += 1;
        }
        if !self.interests.is_empty() {
            len += 1;
        }
        if !self.term.is_empty() {
            len += 1;
        }
        if !self.event_id.is_empty() {
            len += 1;
        }
        if !self.attestation_id.is_empty() {
            len += 1;
        }
        if !self.deposit_txs.is_empty() {
            len += 1;
        }
        if !self.collateral_amount.is_empty() {
            len += 1;
        }
        if !self.loan_secret.is_empty() {
            len += 1;
        }
        if self.create_at.is_some() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.pool_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.Loan", len)?;
        if !self.vault_address.is_empty() {
            struct_ser.serialize_field("vaultAddress", &self.vault_address)?;
        }
        if !self.borrower.is_empty() {
            struct_ser.serialize_field("borrower", &self.borrower)?;
        }
        if !self.agency.is_empty() {
            struct_ser.serialize_field("agency", &self.agency)?;
        }
        if !self.hash_loan_secret.is_empty() {
            struct_ser.serialize_field("hashLoanSecret", &self.hash_loan_secret)?;
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
        if !self.fees.is_empty() {
            struct_ser.serialize_field("fees", &self.fees)?;
        }
        if !self.interests.is_empty() {
            struct_ser.serialize_field("interests", &self.interests)?;
        }
        if !self.term.is_empty() {
            struct_ser.serialize_field("term", &self.term)?;
        }
        if !self.event_id.is_empty() {
            struct_ser.serialize_field("eventId", &self.event_id)?;
        }
        if !self.attestation_id.is_empty() {
            struct_ser.serialize_field("attestationId", &self.attestation_id)?;
        }
        if !self.deposit_txs.is_empty() {
            struct_ser.serialize_field("depositTxs", &self.deposit_txs)?;
        }
        if !self.collateral_amount.is_empty() {
            struct_ser.serialize_field("collateralAmount", &self.collateral_amount)?;
        }
        if !self.loan_secret.is_empty() {
            struct_ser.serialize_field("loanSecret", &self.loan_secret)?;
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
        if !self.pool_id.is_empty() {
            struct_ser.serialize_field("poolId", &self.pool_id)?;
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
            "agency",
            "hash_loan_secret",
            "hashLoanSecret",
            "maturity_time",
            "maturityTime",
            "final_timeout",
            "finalTimeout",
            "borrow_amount",
            "borrowAmount",
            "fees",
            "interests",
            "term",
            "event_id",
            "eventId",
            "attestation_id",
            "attestationId",
            "deposit_txs",
            "depositTxs",
            "collateral_amount",
            "collateralAmount",
            "loan_secret",
            "loanSecret",
            "create_at",
            "createAt",
            "status",
            "pool_id",
            "poolId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VaultAddress,
            Borrower,
            Agency,
            HashLoanSecret,
            MaturityTime,
            FinalTimeout,
            BorrowAmount,
            Fees,
            Interests,
            Term,
            EventId,
            AttestationId,
            DepositTxs,
            CollateralAmount,
            LoanSecret,
            CreateAt,
            Status,
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
                            "vaultAddress" | "vault_address" => Ok(GeneratedField::VaultAddress),
                            "borrower" => Ok(GeneratedField::Borrower),
                            "agency" => Ok(GeneratedField::Agency),
                            "hashLoanSecret" | "hash_loan_secret" => {
                                Ok(GeneratedField::HashLoanSecret)
                            }
                            "maturityTime" | "maturity_time" => Ok(GeneratedField::MaturityTime),
                            "finalTimeout" | "final_timeout" => Ok(GeneratedField::FinalTimeout),
                            "borrowAmount" | "borrow_amount" => Ok(GeneratedField::BorrowAmount),
                            "fees" => Ok(GeneratedField::Fees),
                            "interests" => Ok(GeneratedField::Interests),
                            "term" => Ok(GeneratedField::Term),
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            "attestationId" | "attestation_id" => Ok(GeneratedField::AttestationId),
                            "depositTxs" | "deposit_txs" => Ok(GeneratedField::DepositTxs),
                            "collateralAmount" | "collateral_amount" => {
                                Ok(GeneratedField::CollateralAmount)
                            }
                            "loanSecret" | "loan_secret" => Ok(GeneratedField::LoanSecret),
                            "createAt" | "create_at" => Ok(GeneratedField::CreateAt),
                            "status" => Ok(GeneratedField::Status),
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
                let mut agency__ = None;
                let mut hash_loan_secret__ = None;
                let mut maturity_time__ = None;
                let mut final_timeout__ = None;
                let mut borrow_amount__ = None;
                let mut fees__ = None;
                let mut interests__ = None;
                let mut term__ = None;
                let mut event_id__ = None;
                let mut attestation_id__ = None;
                let mut deposit_txs__ = None;
                let mut collateral_amount__ = None;
                let mut loan_secret__ = None;
                let mut create_at__ = None;
                let mut status__ = None;
                let mut pool_id__ = None;
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
                        GeneratedField::Agency => {
                            if agency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agency"));
                            }
                            agency__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HashLoanSecret => {
                            if hash_loan_secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashLoanSecret"));
                            }
                            hash_loan_secret__ = Some(map_.next_value()?);
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
                        GeneratedField::Fees => {
                            if fees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fees"));
                            }
                            fees__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Interests => {
                            if interests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interests"));
                            }
                            interests__ = Some(map_.next_value()?);
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
                            event_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttestationId => {
                            if attestation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestationId"));
                            }
                            attestation_id__ = Some(map_.next_value()?);
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
                        GeneratedField::LoanSecret => {
                            if loan_secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanSecret"));
                            }
                            loan_secret__ = Some(map_.next_value()?);
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
                        GeneratedField::PoolId => {
                            if pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolId"));
                            }
                            pool_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Loan {
                    vault_address: vault_address__.unwrap_or_default(),
                    borrower: borrower__.unwrap_or_default(),
                    agency: agency__.unwrap_or_default(),
                    hash_loan_secret: hash_loan_secret__.unwrap_or_default(),
                    maturity_time: maturity_time__.unwrap_or_default(),
                    final_timeout: final_timeout__.unwrap_or_default(),
                    borrow_amount: borrow_amount__,
                    fees: fees__.unwrap_or_default(),
                    interests: interests__.unwrap_or_default(),
                    term: term__.unwrap_or_default(),
                    event_id: event_id__.unwrap_or_default(),
                    attestation_id: attestation_id__.unwrap_or_default(),
                    deposit_txs: deposit_txs__.unwrap_or_default(),
                    collateral_amount: collateral_amount__.unwrap_or_default(),
                    loan_secret: loan_secret__.unwrap_or_default(),
                    create_at: create_at__,
                    status: status__.unwrap_or_default(),
                    pool_id: pool_id__.unwrap_or_default(),
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
            Self::Apply => "Apply",
            Self::Approve => "Approve",
            Self::Disburse => "Disburse",
            Self::Repay => "Repay",
            Self::Default => "Default",
            Self::Liquidate => "Liquidate",
            Self::Close => "Close",
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
            "Apply",
            "Approve",
            "Disburse",
            "Repay",
            "Default",
            "Liquidate",
            "Close",
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
                    "Apply" => Ok(LoanStatus::Apply),
                    "Approve" => Ok(LoanStatus::Approve),
                    "Disburse" => Ok(LoanStatus::Disburse),
                    "Repay" => Ok(LoanStatus::Repay),
                    "Default" => Ok(LoanStatus::Default),
                    "Liquidate" => Ok(LoanStatus::Liquidate),
                    "Close" => Ok(LoanStatus::Close),
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
        if !self.pool_id.is_empty() {
            len += 1;
        }
        if !self.lender.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgAddLiquidity", len)?;
        if !self.pool_id.is_empty() {
            struct_ser.serialize_field("poolId", &self.pool_id)?;
        }
        if !self.lender.is_empty() {
            struct_ser.serialize_field("lender", &self.lender)?;
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
        const FIELDS: &[&str] = &["pool_id", "poolId", "lender", "amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PoolId,
            Lender,
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
                            "poolId" | "pool_id" => Ok(GeneratedField::PoolId),
                            "lender" => Ok(GeneratedField::Lender),
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
                let mut pool_id__ = None;
                let mut lender__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PoolId => {
                            if pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolId"));
                            }
                            pool_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Lender => {
                            if lender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lender"));
                            }
                            lender__ = Some(map_.next_value()?);
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
                    pool_id: pool_id__.unwrap_or_default(),
                    lender: lender__.unwrap_or_default(),
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
        let mut len = 0;
        if self.shares.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.MsgAddLiquidityResponse", len)?;
        if let Some(v) = self.shares.as_ref() {
            struct_ser.serialize_field("shares", v)?;
        }
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
        const FIELDS: &[&str] = &["shares"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                let mut shares__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Shares => {
                            if shares__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shares"));
                            }
                            shares__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgAddLiquidityResponse { shares: shares__ })
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
        if !self.loan_secret_hash.is_empty() {
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
        if !self.event_id.is_empty() {
            len += 1;
        }
        if !self.cets.is_empty() {
            len += 1;
        }
        if !self.deposit_tx.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgApply", len)?;
        if !self.borrower.is_empty() {
            struct_ser.serialize_field("borrower", &self.borrower)?;
        }
        if !self.borrower_pubkey.is_empty() {
            struct_ser.serialize_field("borrowerPubkey", &self.borrower_pubkey)?;
        }
        if !self.loan_secret_hash.is_empty() {
            struct_ser.serialize_field("loanSecretHash", &self.loan_secret_hash)?;
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
        if !self.event_id.is_empty() {
            struct_ser.serialize_field("eventId", &self.event_id)?;
        }
        if !self.cets.is_empty() {
            struct_ser.serialize_field("cets", &self.cets)?;
        }
        if !self.deposit_tx.is_empty() {
            struct_ser.serialize_field("depositTx", &self.deposit_tx)?;
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
            "loan_secret_hash",
            "loanSecretHash",
            "maturity_time",
            "maturityTime",
            "final_timeout",
            "finalTimeout",
            "pool_id",
            "poolId",
            "borrow_amount",
            "borrowAmount",
            "event_id",
            "eventId",
            "cets",
            "deposit_tx",
            "depositTx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Borrower,
            BorrowerPubkey,
            LoanSecretHash,
            MaturityTime,
            FinalTimeout,
            PoolId,
            BorrowAmount,
            EventId,
            Cets,
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
                            "borrower" => Ok(GeneratedField::Borrower),
                            "borrowerPubkey" | "borrower_pubkey" => {
                                Ok(GeneratedField::BorrowerPubkey)
                            }
                            "loanSecretHash" | "loan_secret_hash" => {
                                Ok(GeneratedField::LoanSecretHash)
                            }
                            "maturityTime" | "maturity_time" => Ok(GeneratedField::MaturityTime),
                            "finalTimeout" | "final_timeout" => Ok(GeneratedField::FinalTimeout),
                            "poolId" | "pool_id" => Ok(GeneratedField::PoolId),
                            "borrowAmount" | "borrow_amount" => Ok(GeneratedField::BorrowAmount),
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            "cets" => Ok(GeneratedField::Cets),
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
                let mut loan_secret_hash__ = None;
                let mut maturity_time__ = None;
                let mut final_timeout__ = None;
                let mut pool_id__ = None;
                let mut borrow_amount__ = None;
                let mut event_id__ = None;
                let mut cets__ = None;
                let mut deposit_tx__ = None;
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
                        GeneratedField::LoanSecretHash => {
                            if loan_secret_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanSecretHash"));
                            }
                            loan_secret_hash__ = Some(map_.next_value()?);
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
                        GeneratedField::EventId => {
                            if event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventId"));
                            }
                            event_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cets => {
                            if cets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cets"));
                            }
                            cets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DepositTx => {
                            if deposit_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositTx"));
                            }
                            deposit_tx__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgApply {
                    borrower: borrower__.unwrap_or_default(),
                    borrower_pubkey: borrower_pubkey__.unwrap_or_default(),
                    loan_secret_hash: loan_secret_hash__.unwrap_or_default(),
                    maturity_time: maturity_time__.unwrap_or_default(),
                    final_timeout: final_timeout__.unwrap_or_default(),
                    pool_id: pool_id__.unwrap_or_default(),
                    borrow_amount: borrow_amount__,
                    event_id: event_id__.unwrap_or_default(),
                    cets: cets__.unwrap_or_default(),
                    deposit_tx: deposit_tx__.unwrap_or_default(),
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
        let mut len = 0;
        if !self.vault_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgApplyResponse", len)?;
        if !self.vault_address.is_empty() {
            struct_ser.serialize_field("vaultAddress", &self.vault_address)?;
        }
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
        const FIELDS: &[&str] = &["vault_address", "vaultAddress"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VaultAddress,
        }
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
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
                let mut vault_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VaultAddress => {
                            if vault_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaultAddress"));
                            }
                            vault_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgApplyResponse {
                    vault_address: vault_address__.unwrap_or_default(),
                })
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
        if self.height != 0 {
            len += 1;
        }
        if !self.poof.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgApprove", len)?;
        if !self.relayer.is_empty() {
            struct_ser.serialize_field("relayer", &self.relayer)?;
        }
        if !self.deposit_tx_id.is_empty() {
            struct_ser.serialize_field("depositTxId", &self.deposit_tx_id)?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field(
                "height",
                alloc::string::ToString::to_string(&self.height).as_str(),
            )?;
        }
        if !self.poof.is_empty() {
            struct_ser.serialize_field("poof", &self.poof)?;
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
        const FIELDS: &[&str] = &["relayer", "deposit_tx_id", "depositTxId", "height", "poof"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relayer,
            DepositTxId,
            Height,
            Poof,
        }
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
                            "height" => Ok(GeneratedField::Height),
                            "poof" => Ok(GeneratedField::Poof),
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
                let mut height__ = None;
                let mut poof__ = None;
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
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = Some(
                                map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Poof => {
                            if poof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poof"));
                            }
                            poof__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgApprove {
                    relayer: relayer__.unwrap_or_default(),
                    deposit_tx_id: deposit_tx_id__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    poof: poof__.unwrap_or_default(),
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
        if !self.creator.is_empty() {
            len += 1;
        }
        if !self.pool_id.is_empty() {
            len += 1;
        }
        if !self.lending_asset.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgCreatePool", len)?;
        if !self.creator.is_empty() {
            struct_ser.serialize_field("creator", &self.creator)?;
        }
        if !self.pool_id.is_empty() {
            struct_ser.serialize_field("poolId", &self.pool_id)?;
        }
        if !self.lending_asset.is_empty() {
            struct_ser.serialize_field("lendingAsset", &self.lending_asset)?;
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
        const FIELDS: &[&str] = &[
            "creator",
            "pool_id",
            "poolId",
            "lending_asset",
            "lendingAsset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Creator,
            PoolId,
            LendingAsset,
        }
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
                            "creator" => Ok(GeneratedField::Creator),
                            "poolId" | "pool_id" => Ok(GeneratedField::PoolId),
                            "lendingAsset" | "lending_asset" => Ok(GeneratedField::LendingAsset),
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
                let mut creator__ = None;
                let mut pool_id__ = None;
                let mut lending_asset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Creator => {
                            if creator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creator"));
                            }
                            creator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PoolId => {
                            if pool_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolId"));
                            }
                            pool_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LendingAsset => {
                            if lending_asset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lendingAsset"));
                            }
                            lending_asset__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgCreatePool {
                    creator: creator__.unwrap_or_default(),
                    pool_id: pool_id__.unwrap_or_default(),
                    lending_asset: lending_asset__.unwrap_or_default(),
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
impl serde::Serialize for MsgRedeem {
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
        if !self.loan_secret.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.MsgRedeem", len)?;
        if !self.borrower.is_empty() {
            struct_ser.serialize_field("borrower", &self.borrower)?;
        }
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        if !self.loan_secret.is_empty() {
            struct_ser.serialize_field("loanSecret", &self.loan_secret)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRedeem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["borrower", "loan_id", "loanId", "loan_secret", "loanSecret"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Borrower,
            LoanId,
            LoanSecret,
        }
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
                            "loanSecret" | "loan_secret" => Ok(GeneratedField::LoanSecret),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRedeem;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgRedeem")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgRedeem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut borrower__ = None;
                let mut loan_id__ = None;
                let mut loan_secret__ = None;
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
                        GeneratedField::LoanSecret => {
                            if loan_secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loanSecret"));
                            }
                            loan_secret__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRedeem {
                    borrower: borrower__.unwrap_or_default(),
                    loan_id: loan_id__.unwrap_or_default(),
                    loan_secret: loan_secret__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.MsgRedeem", FIELDS, GeneratedVisitor)
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for MsgRedeemResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("side.lending.MsgRedeemResponse", len)?;
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MsgRedeemResponse {
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
            type Value = MsgRedeemResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.MsgRedeemResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> core::result::Result<MsgRedeemResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRedeemResponse {})
            }
        }
        deserializer.deserialize_struct("side.lending.MsgRedeemResponse", FIELDS, GeneratedVisitor)
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
        let mut len = 0;
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.MsgRemoveLiquidityResponse", len)?;
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
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
        const FIELDS: &[&str] = &["amount"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgRemoveLiquidityResponse { amount: amount__ })
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
impl serde::Serialize for Params {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.supply_rate_permille.is_empty() {
            len += 1;
        }
        if !self.borrow_rate_permille.is_empty() {
            len += 1;
        }
        if !self.fee_recipient.is_empty() {
            len += 1;
        }
        if !self.pool_creators.is_empty() {
            len += 1;
        }
        if !self.min_initial_ltv_percent.is_empty() {
            len += 1;
        }
        if !self.liquidation_threshold_percent.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("side.lending.Params", len)?;
        if !self.supply_rate_permille.is_empty() {
            struct_ser.serialize_field("supplyRatePermille", &self.supply_rate_permille)?;
        }
        if !self.borrow_rate_permille.is_empty() {
            struct_ser.serialize_field("borrowRatePermille", &self.borrow_rate_permille)?;
        }
        if !self.fee_recipient.is_empty() {
            struct_ser.serialize_field("feeRecipient", &self.fee_recipient)?;
        }
        if !self.pool_creators.is_empty() {
            struct_ser.serialize_field("poolCreators", &self.pool_creators)?;
        }
        if !self.min_initial_ltv_percent.is_empty() {
            struct_ser.serialize_field("minInitialLtvPercent", &self.min_initial_ltv_percent)?;
        }
        if !self.liquidation_threshold_percent.is_empty() {
            struct_ser.serialize_field(
                "liquidationThresholdPercent",
                &self.liquidation_threshold_percent,
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
            "supply_rate_permille",
            "supplyRatePermille",
            "borrow_rate_permille",
            "borrowRatePermille",
            "fee_recipient",
            "feeRecipient",
            "pool_creators",
            "poolCreators",
            "min_initial_ltv_percent",
            "minInitialLtvPercent",
            "liquidation_threshold_percent",
            "liquidationThresholdPercent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SupplyRatePermille,
            BorrowRatePermille,
            FeeRecipient,
            PoolCreators,
            MinInitialLtvPercent,
            LiquidationThresholdPercent,
        }
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
                            "supplyRatePermille" | "supply_rate_permille" => {
                                Ok(GeneratedField::SupplyRatePermille)
                            }
                            "borrowRatePermille" | "borrow_rate_permille" => {
                                Ok(GeneratedField::BorrowRatePermille)
                            }
                            "feeRecipient" | "fee_recipient" => Ok(GeneratedField::FeeRecipient),
                            "poolCreators" | "pool_creators" => Ok(GeneratedField::PoolCreators),
                            "minInitialLtvPercent" | "min_initial_ltv_percent" => {
                                Ok(GeneratedField::MinInitialLtvPercent)
                            }
                            "liquidationThresholdPercent" | "liquidation_threshold_percent" => {
                                Ok(GeneratedField::LiquidationThresholdPercent)
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
                let mut supply_rate_permille__ = None;
                let mut borrow_rate_permille__ = None;
                let mut fee_recipient__ = None;
                let mut pool_creators__ = None;
                let mut min_initial_ltv_percent__ = None;
                let mut liquidation_threshold_percent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SupplyRatePermille => {
                            if supply_rate_permille__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "supplyRatePermille",
                                ));
                            }
                            supply_rate_permille__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowRatePermille => {
                            if borrow_rate_permille__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "borrowRatePermille",
                                ));
                            }
                            borrow_rate_permille__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeeRecipient => {
                            if fee_recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeRecipient"));
                            }
                            fee_recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PoolCreators => {
                            if pool_creators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolCreators"));
                            }
                            pool_creators__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MinInitialLtvPercent => {
                            if min_initial_ltv_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "minInitialLtvPercent",
                                ));
                            }
                            min_initial_ltv_percent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LiquidationThresholdPercent => {
                            if liquidation_threshold_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "liquidationThresholdPercent",
                                ));
                            }
                            liquidation_threshold_percent__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    supply_rate_permille: supply_rate_permille__.unwrap_or_default(),
                    borrow_rate_permille: borrow_rate_permille__.unwrap_or_default(),
                    fee_recipient: fee_recipient__.unwrap_or_default(),
                    pool_creators: pool_creators__.unwrap_or_default(),
                    min_initial_ltv_percent: min_initial_ltv_percent__.unwrap_or_default(),
                    liquidation_threshold_percent: liquidation_threshold_percent__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("side.lending.Params", FIELDS, GeneratedVisitor)
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
            Self::Active => "ACTIVE",
            Self::Inactive => "INACTIVE",
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
        const FIELDS: &[&str] = &["ACTIVE", "INACTIVE"];

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
                    "ACTIVE" => Ok(PoolStatus::Active),
                    "INACTIVE" => Ok(PoolStatus::Inactive),
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
        if !self.hash_of_loan_secret.is_empty() {
            len += 1;
        }
        if self.maturity_time != 0 {
            len += 1;
        }
        if self.final_timeout != 0 {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryCollateralAddressRequest", len)?;
        if !self.borrower_pubkey.is_empty() {
            struct_ser.serialize_field("borrowerPubkey", &self.borrower_pubkey)?;
        }
        if !self.hash_of_loan_secret.is_empty() {
            struct_ser.serialize_field("hashOfLoanSecret", &self.hash_of_loan_secret)?;
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
            "hash_of_loan_secret",
            "hashOfLoanSecret",
            "maturity_time",
            "maturityTime",
            "final_timeout",
            "finalTimeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BorrowerPubkey,
            HashOfLoanSecret,
            MaturityTime,
            FinalTimeout,
        }
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
                            "hashOfLoanSecret" | "hash_of_loan_secret" => {
                                Ok(GeneratedField::HashOfLoanSecret)
                            }
                            "maturityTime" | "maturity_time" => Ok(GeneratedField::MaturityTime),
                            "finalTimeout" | "final_timeout" => Ok(GeneratedField::FinalTimeout),
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
                let mut hash_of_loan_secret__ = None;
                let mut maturity_time__ = None;
                let mut final_timeout__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BorrowerPubkey => {
                            if borrower_pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowerPubkey"));
                            }
                            borrower_pubkey__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HashOfLoanSecret => {
                            if hash_of_loan_secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashOfLoanSecret"));
                            }
                            hash_of_loan_secret__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(QueryCollateralAddressRequest {
                    borrower_pubkey: borrower_pubkey__.unwrap_or_default(),
                    hash_of_loan_secret: hash_of_loan_secret__.unwrap_or_default(),
                    maturity_time: maturity_time__.unwrap_or_default(),
                    final_timeout: final_timeout__.unwrap_or_default(),
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
impl serde::Serialize for QueryLiquidationEventRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.borrow_amount.is_some() {
            len += 1;
        }
        if self.collateral_acmount.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLiquidationEventRequest", len)?;
        if let Some(v) = self.borrow_amount.as_ref() {
            struct_ser.serialize_field("borrowAmount", v)?;
        }
        if let Some(v) = self.collateral_acmount.as_ref() {
            struct_ser.serialize_field("collateralAcmount", v)?;
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
            "borrow_amount",
            "borrowAmount",
            "collateral_acmount",
            "collateralAcmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BorrowAmount,
            CollateralAcmount,
        }
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
                            "borrowAmount" | "borrow_amount" => Ok(GeneratedField::BorrowAmount),
                            "collateralAcmount" | "collateral_acmount" => {
                                Ok(GeneratedField::CollateralAcmount)
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
                let mut borrow_amount__ = None;
                let mut collateral_acmount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BorrowAmount => {
                            if borrow_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowAmount"));
                            }
                            borrow_amount__ = map_.next_value()?;
                        }
                        GeneratedField::CollateralAcmount => {
                            if collateral_acmount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collateralAcmount"));
                            }
                            collateral_acmount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLiquidationEventRequest {
                    borrow_amount: borrow_amount__,
                    collateral_acmount: collateral_acmount__,
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
        if !self.event_id.is_empty() {
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
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLiquidationEventResponse", len)?;
        if !self.event_id.is_empty() {
            struct_ser.serialize_field("eventId", &self.event_id)?;
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventId,
            OraclePubkey,
            Nonce,
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
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            "oraclePubkey" | "oracle_pubkey" => Ok(GeneratedField::OraclePubkey),
                            "nonce" => Ok(GeneratedField::Nonce),
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
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EventId => {
                            if event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventId"));
                            }
                            event_id__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(QueryLiquidationEventResponse {
                    event_id: event_id__.unwrap_or_default(),
                    oracle_pubkey: oracle_pubkey__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    price: price__.unwrap_or_default(),
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
impl serde::Serialize for QueryLoanCetRequest {
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
            serializer.serialize_struct("side.lending.QueryLoanCETRequest", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLoanCetRequest {
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
            type Value = QueryLoanCetRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoanCETRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLoanCetRequest, V::Error>
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
                Ok(QueryLoanCetRequest {
                    loan_id: loan_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLoanCETRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryLoanCetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cet.is_some() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryLoanCETResponse", len)?;
        if let Some(v) = self.cet.as_ref() {
            struct_ser.serialize_field("cet", v)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryLoanCetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["cet"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cet,
        }
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
                            "cet" => Ok(GeneratedField::Cet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLoanCetResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryLoanCETResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryLoanCetResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut cet__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cet => {
                            if cet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cet"));
                            }
                            cet__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryLoanCetResponse { cet: cet__ })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryLoanCETResponse",
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
impl serde::Serialize for QueryRepaymentTxRequest {
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
            serializer.serialize_struct("side.lending.QueryRepaymentTxRequest", len)?;
        if !self.loan_id.is_empty() {
            struct_ser.serialize_field("loanId", &self.loan_id)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryRepaymentTxRequest {
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
            type Value = QueryRepaymentTxRequest;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryRepaymentTxRequest")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryRepaymentTxRequest, V::Error>
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
                Ok(QueryRepaymentTxRequest {
                    loan_id: loan_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryRepaymentTxRequest",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for QueryRepaymentTxResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.claim_tx.is_empty() {
            len += 1;
        }
        let mut struct_ser =
            serializer.serialize_struct("side.lending.QueryRepaymentTxResponse", len)?;
        if !self.claim_tx.is_empty() {
            struct_ser.serialize_field("claimTx", &self.claim_tx)?;
        }
        struct_ser.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QueryRepaymentTxResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["claim_tx", "claimTx"];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClaimTx,
        }
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
                            "claimTx" | "claim_tx" => Ok(GeneratedField::ClaimTx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRepaymentTxResponse;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("struct side.lending.QueryRepaymentTxResponse")
            }

            fn visit_map<V>(
                self,
                mut map_: V,
            ) -> core::result::Result<QueryRepaymentTxResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut claim_tx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClaimTx => {
                            if claim_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimTx"));
                            }
                            claim_tx__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryRepaymentTxResponse {
                    claim_tx: claim_tx__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct(
            "side.lending.QueryRepaymentTxResponse",
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
        if !self.repay_adaptor_point.is_empty() {
            len += 1;
        }
        if !self.borrower_signature.is_empty() {
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
        if !self.repay_adaptor_point.is_empty() {
            struct_ser.serialize_field("repayAdaptorPoint", &self.repay_adaptor_point)?;
        }
        if !self.borrower_signature.is_empty() {
            struct_ser.serialize_field("borrowerSignature", &self.borrower_signature)?;
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
            "repay_adaptor_point",
            "repayAdaptorPoint",
            "borrower_signature",
            "borrowerSignature",
            "create_at",
            "createAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoanId,
            Txid,
            Tx,
            RepayAdaptorPoint,
            BorrowerSignature,
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
                            "repayAdaptorPoint" | "repay_adaptor_point" => {
                                Ok(GeneratedField::RepayAdaptorPoint)
                            }
                            "borrowerSignature" | "borrower_signature" => {
                                Ok(GeneratedField::BorrowerSignature)
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
                let mut repay_adaptor_point__ = None;
                let mut borrower_signature__ = None;
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
                        GeneratedField::RepayAdaptorPoint => {
                            if repay_adaptor_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repayAdaptorPoint"));
                            }
                            repay_adaptor_point__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BorrowerSignature => {
                            if borrower_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("borrowerSignature"));
                            }
                            borrower_signature__ = Some(map_.next_value()?);
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
                    repay_adaptor_point: repay_adaptor_point__.unwrap_or_default(),
                    borrower_signature: borrower_signature__.unwrap_or_default(),
                    create_at: create_at__,
                })
            }
        }
        deserializer.deserialize_struct("side.lending.Repayment", FIELDS, GeneratedVisitor)
    }
}
