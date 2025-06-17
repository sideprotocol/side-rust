// @generated
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn query_params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/side.btcbridge.Query/QueryParams");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Query", "QueryParams"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_fee_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFeeRateRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryFeeRateResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/side.btcbridge.Query/QueryFeeRate");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Query", "QueryFeeRate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_withdrawal_network_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWithdrawalNetworkFeeRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryWithdrawalNetworkFeeResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QueryWithdrawalNetworkFee",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryWithdrawalNetworkFee",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_withdraw_requests_by_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWithdrawRequestsByAddressRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryWithdrawRequestsByAddressResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QueryWithdrawRequestsByAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryWithdrawRequestsByAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_withdraw_requests_by_tx_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWithdrawRequestsByTxHashRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryWithdrawRequestsByTxHashResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QueryWithdrawRequestsByTxHash",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryWithdrawRequestsByTxHash",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_pending_btc_withdraw_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPendingBtcWithdrawRequestsRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryPendingBtcWithdrawRequestsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QueryPendingBtcWithdrawRequests",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryPendingBtcWithdrawRequests",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_signing_request(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySigningRequestRequest>,
        ) -> core::result::Result<tonic::Response<super::QuerySigningRequestResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/side.btcbridge.Query/QuerySigningRequest");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QuerySigningRequest",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_signing_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySigningRequestsRequest>,
        ) -> core::result::Result<tonic::Response<super::QuerySigningRequestsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/side.btcbridge.Query/QuerySigningRequests");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QuerySigningRequests",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_signing_requests_by_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySigningRequestsByAddressRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QuerySigningRequestsByAddressResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QuerySigningRequestsByAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QuerySigningRequestsByAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_signing_request_by_tx_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySigningRequestByTxHashRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QuerySigningRequestByTxHashResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QuerySigningRequestByTxHash",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QuerySigningRequestByTxHash",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_pending_signing_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPendingSigningRequestsRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryPendingSigningRequestsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QueryPendingSigningRequests",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryPendingSigningRequests",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_utx_os(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUtxOsRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryUtxOsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/side.btcbridge.Query/QueryUTXOs");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Query", "QueryUTXOs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_utx_os_by_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUtxOsByAddressRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryUtxOsByAddressResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/side.btcbridge.Query/QueryUTXOsByAddress");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryUTXOsByAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_utxo_count_and_balances_by_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUtxoCountAndBalancesByAddressRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryUtxoCountAndBalancesByAddressResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QueryUTXOCountAndBalancesByAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryUTXOCountAndBalancesByAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_dkg_request(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDkgRequestRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryDkgRequestResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/side.btcbridge.Query/QueryDKGRequest");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Query", "QueryDKGRequest"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_dkg_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDkgRequestsRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryDkgRequestsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/side.btcbridge.Query/QueryDKGRequests");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Query", "QueryDKGRequests"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_all_dkg_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllDkgRequestsRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryAllDkgRequestsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/side.btcbridge.Query/QueryAllDKGRequests");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryAllDKGRequests",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_dkg_completion_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDkgCompletionRequestsRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryDkgCompletionRequestsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QueryDKGCompletionRequests",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryDKGCompletionRequests",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_refreshing_request(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRefreshingRequestRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryRefreshingRequestResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QueryRefreshingRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryRefreshingRequest",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_refreshing_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRefreshingRequestsRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryRefreshingRequestsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QueryRefreshingRequests",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryRefreshingRequests",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_refreshing_completions(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRefreshingCompletionsRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryRefreshingCompletionsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QueryRefreshingCompletions",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryRefreshingCompletions",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_ibc_deposit_script(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryIbcDepositScriptRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryIbcDepositScriptResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/side.btcbridge.Query/QueryIBCDepositScript");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryIBCDepositScript",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_rate_limit(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRateLimitRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryRateLimitResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/side.btcbridge.Query/QueryRateLimit");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Query", "QueryRateLimit"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query_rate_limit_by_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRateLimitByAddressRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryRateLimitByAddressResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Query/QueryRateLimitByAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Query",
                "QueryRateLimitByAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        async fn query_params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        async fn query_fee_rate(
            &self,
            request: tonic::Request<super::QueryFeeRateRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryFeeRateResponse>, tonic::Status>;
        async fn query_withdrawal_network_fee(
            &self,
            request: tonic::Request<super::QueryWithdrawalNetworkFeeRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryWithdrawalNetworkFeeResponse>,
            tonic::Status,
        >;
        async fn query_withdraw_requests_by_address(
            &self,
            request: tonic::Request<super::QueryWithdrawRequestsByAddressRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryWithdrawRequestsByAddressResponse>,
            tonic::Status,
        >;
        async fn query_withdraw_requests_by_tx_hash(
            &self,
            request: tonic::Request<super::QueryWithdrawRequestsByTxHashRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryWithdrawRequestsByTxHashResponse>,
            tonic::Status,
        >;
        async fn query_pending_btc_withdraw_requests(
            &self,
            request: tonic::Request<super::QueryPendingBtcWithdrawRequestsRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryPendingBtcWithdrawRequestsResponse>,
            tonic::Status,
        >;
        async fn query_signing_request(
            &self,
            request: tonic::Request<super::QuerySigningRequestRequest>,
        ) -> core::result::Result<tonic::Response<super::QuerySigningRequestResponse>, tonic::Status>;
        async fn query_signing_requests(
            &self,
            request: tonic::Request<super::QuerySigningRequestsRequest>,
        ) -> core::result::Result<tonic::Response<super::QuerySigningRequestsResponse>, tonic::Status>;
        async fn query_signing_requests_by_address(
            &self,
            request: tonic::Request<super::QuerySigningRequestsByAddressRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QuerySigningRequestsByAddressResponse>,
            tonic::Status,
        >;
        async fn query_signing_request_by_tx_hash(
            &self,
            request: tonic::Request<super::QuerySigningRequestByTxHashRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QuerySigningRequestByTxHashResponse>,
            tonic::Status,
        >;
        async fn query_pending_signing_requests(
            &self,
            request: tonic::Request<super::QueryPendingSigningRequestsRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryPendingSigningRequestsResponse>,
            tonic::Status,
        >;
        async fn query_utx_os(
            &self,
            request: tonic::Request<super::QueryUtxOsRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryUtxOsResponse>, tonic::Status>;
        async fn query_utx_os_by_address(
            &self,
            request: tonic::Request<super::QueryUtxOsByAddressRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryUtxOsByAddressResponse>, tonic::Status>;
        async fn query_utxo_count_and_balances_by_address(
            &self,
            request: tonic::Request<super::QueryUtxoCountAndBalancesByAddressRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryUtxoCountAndBalancesByAddressResponse>,
            tonic::Status,
        >;
        async fn query_dkg_request(
            &self,
            request: tonic::Request<super::QueryDkgRequestRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryDkgRequestResponse>, tonic::Status>;
        async fn query_dkg_requests(
            &self,
            request: tonic::Request<super::QueryDkgRequestsRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryDkgRequestsResponse>, tonic::Status>;
        async fn query_all_dkg_requests(
            &self,
            request: tonic::Request<super::QueryAllDkgRequestsRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryAllDkgRequestsResponse>, tonic::Status>;
        async fn query_dkg_completion_requests(
            &self,
            request: tonic::Request<super::QueryDkgCompletionRequestsRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryDkgCompletionRequestsResponse>,
            tonic::Status,
        >;
        async fn query_refreshing_request(
            &self,
            request: tonic::Request<super::QueryRefreshingRequestRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryRefreshingRequestResponse>,
            tonic::Status,
        >;
        async fn query_refreshing_requests(
            &self,
            request: tonic::Request<super::QueryRefreshingRequestsRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryRefreshingRequestsResponse>,
            tonic::Status,
        >;
        async fn query_refreshing_completions(
            &self,
            request: tonic::Request<super::QueryRefreshingCompletionsRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryRefreshingCompletionsResponse>,
            tonic::Status,
        >;
        async fn query_ibc_deposit_script(
            &self,
            request: tonic::Request<super::QueryIbcDepositScriptRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryIbcDepositScriptResponse>,
            tonic::Status,
        >;
        async fn query_rate_limit(
            &self,
            request: tonic::Request<super::QueryRateLimitRequest>,
        ) -> core::result::Result<tonic::Response<super::QueryRateLimitResponse>, tonic::Status>;
        async fn query_rate_limit_by_address(
            &self,
            request: tonic::Request<super::QueryRateLimitByAddressRequest>,
        ) -> core::result::Result<
            tonic::Response<super::QueryRateLimitByAddressResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<core::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/side.btcbridge.Query/QueryParams" => {
                    #[allow(non_camel_case_types)]
                    struct QueryParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest> for QueryParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).query_params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryFeeRate" => {
                    #[allow(non_camel_case_types)]
                    struct QueryFeeRateSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryFeeRateRequest> for QueryFeeRateSvc<T> {
                        type Response = super::QueryFeeRateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryFeeRateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).query_fee_rate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryFeeRateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryWithdrawalNetworkFee" => {
                    #[allow(non_camel_case_types)]
                    struct QueryWithdrawalNetworkFeeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryWithdrawalNetworkFeeRequest>
                        for QueryWithdrawalNetworkFeeSvc<T>
                    {
                        type Response = super::QueryWithdrawalNetworkFeeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryWithdrawalNetworkFeeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).query_withdrawal_network_fee(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryWithdrawalNetworkFeeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryWithdrawRequestsByAddress" => {
                    #[allow(non_camel_case_types)]
                    struct QueryWithdrawRequestsByAddressSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryWithdrawRequestsByAddressRequest>
                        for QueryWithdrawRequestsByAddressSvc<T>
                    {
                        type Response = super::QueryWithdrawRequestsByAddressResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryWithdrawRequestsByAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).query_withdraw_requests_by_address(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryWithdrawRequestsByAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryWithdrawRequestsByTxHash" => {
                    #[allow(non_camel_case_types)]
                    struct QueryWithdrawRequestsByTxHashSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryWithdrawRequestsByTxHashRequest>
                        for QueryWithdrawRequestsByTxHashSvc<T>
                    {
                        type Response = super::QueryWithdrawRequestsByTxHashResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryWithdrawRequestsByTxHashRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).query_withdraw_requests_by_tx_hash(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryWithdrawRequestsByTxHashSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryPendingBtcWithdrawRequests" => {
                    #[allow(non_camel_case_types)]
                    struct QueryPendingBtcWithdrawRequestsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryPendingBtcWithdrawRequestsRequest>
                        for QueryPendingBtcWithdrawRequestsSvc<T>
                    {
                        type Response = super::QueryPendingBtcWithdrawRequestsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPendingBtcWithdrawRequestsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).query_pending_btc_withdraw_requests(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryPendingBtcWithdrawRequestsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QuerySigningRequest" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySigningRequestSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QuerySigningRequestRequest>
                        for QuerySigningRequestSvc<T>
                    {
                        type Response = super::QuerySigningRequestResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySigningRequestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).query_signing_request(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QuerySigningRequestSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QuerySigningRequests" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySigningRequestsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QuerySigningRequestsRequest>
                        for QuerySigningRequestsSvc<T>
                    {
                        type Response = super::QuerySigningRequestsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySigningRequestsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).query_signing_requests(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QuerySigningRequestsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QuerySigningRequestsByAddress" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySigningRequestsByAddressSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QuerySigningRequestsByAddressRequest>
                        for QuerySigningRequestsByAddressSvc<T>
                    {
                        type Response = super::QuerySigningRequestsByAddressResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySigningRequestsByAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).query_signing_requests_by_address(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QuerySigningRequestsByAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QuerySigningRequestByTxHash" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySigningRequestByTxHashSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QuerySigningRequestByTxHashRequest>
                        for QuerySigningRequestByTxHashSvc<T>
                    {
                        type Response = super::QuerySigningRequestByTxHashResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySigningRequestByTxHashRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).query_signing_request_by_tx_hash(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QuerySigningRequestByTxHashSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryPendingSigningRequests" => {
                    #[allow(non_camel_case_types)]
                    struct QueryPendingSigningRequestsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryPendingSigningRequestsRequest>
                        for QueryPendingSigningRequestsSvc<T>
                    {
                        type Response = super::QueryPendingSigningRequestsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryPendingSigningRequestsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).query_pending_signing_requests(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryPendingSigningRequestsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryUTXOs" => {
                    #[allow(non_camel_case_types)]
                    struct QueryUTXOsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryUtxOsRequest> for QueryUTXOsSvc<T> {
                        type Response = super::QueryUtxOsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryUtxOsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).query_utx_os(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryUTXOsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryUTXOsByAddress" => {
                    #[allow(non_camel_case_types)]
                    struct QueryUTXOsByAddressSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryUtxOsByAddressRequest>
                        for QueryUTXOsByAddressSvc<T>
                    {
                        type Response = super::QueryUtxOsByAddressResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryUtxOsByAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).query_utx_os_by_address(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryUTXOsByAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryUTXOCountAndBalancesByAddress" => {
                    #[allow(non_camel_case_types)]
                    struct QueryUTXOCountAndBalancesByAddressSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryUtxoCountAndBalancesByAddressRequest,
                        > for QueryUTXOCountAndBalancesByAddressSvc<T>
                    {
                        type Response = super::QueryUtxoCountAndBalancesByAddressResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryUtxoCountAndBalancesByAddressRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .query_utxo_count_and_balances_by_address(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryUTXOCountAndBalancesByAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryDKGRequest" => {
                    #[allow(non_camel_case_types)]
                    struct QueryDKGRequestSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDkgRequestRequest>
                        for QueryDKGRequestSvc<T>
                    {
                        type Response = super::QueryDkgRequestResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDkgRequestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).query_dkg_request(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryDKGRequestSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryDKGRequests" => {
                    #[allow(non_camel_case_types)]
                    struct QueryDKGRequestsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDkgRequestsRequest>
                        for QueryDKGRequestsSvc<T>
                    {
                        type Response = super::QueryDkgRequestsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDkgRequestsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).query_dkg_requests(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryDKGRequestsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryAllDKGRequests" => {
                    #[allow(non_camel_case_types)]
                    struct QueryAllDKGRequestsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllDkgRequestsRequest>
                        for QueryAllDKGRequestsSvc<T>
                    {
                        type Response = super::QueryAllDkgRequestsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllDkgRequestsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).query_all_dkg_requests(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryAllDKGRequestsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryDKGCompletionRequests" => {
                    #[allow(non_camel_case_types)]
                    struct QueryDKGCompletionRequestsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryDkgCompletionRequestsRequest>
                        for QueryDKGCompletionRequestsSvc<T>
                    {
                        type Response = super::QueryDkgCompletionRequestsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDkgCompletionRequestsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).query_dkg_completion_requests(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryDKGCompletionRequestsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryRefreshingRequest" => {
                    #[allow(non_camel_case_types)]
                    struct QueryRefreshingRequestSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryRefreshingRequestRequest>
                        for QueryRefreshingRequestSvc<T>
                    {
                        type Response = super::QueryRefreshingRequestResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRefreshingRequestRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).query_refreshing_request(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryRefreshingRequestSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryRefreshingRequests" => {
                    #[allow(non_camel_case_types)]
                    struct QueryRefreshingRequestsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryRefreshingRequestsRequest>
                        for QueryRefreshingRequestsSvc<T>
                    {
                        type Response = super::QueryRefreshingRequestsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRefreshingRequestsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).query_refreshing_requests(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryRefreshingRequestsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryRefreshingCompletions" => {
                    #[allow(non_camel_case_types)]
                    struct QueryRefreshingCompletionsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryRefreshingCompletionsRequest>
                        for QueryRefreshingCompletionsSvc<T>
                    {
                        type Response = super::QueryRefreshingCompletionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRefreshingCompletionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).query_refreshing_completions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryRefreshingCompletionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryIBCDepositScript" => {
                    #[allow(non_camel_case_types)]
                    struct QueryIBCDepositScriptSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryIbcDepositScriptRequest>
                        for QueryIBCDepositScriptSvc<T>
                    {
                        type Response = super::QueryIbcDepositScriptResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryIbcDepositScriptRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).query_ibc_deposit_script(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryIBCDepositScriptSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryRateLimit" => {
                    #[allow(non_camel_case_types)]
                    struct QueryRateLimitSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryRateLimitRequest> for QueryRateLimitSvc<T> {
                        type Response = super::QueryRateLimitResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRateLimitRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).query_rate_limit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryRateLimitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Query/QueryRateLimitByAddress" => {
                    #[allow(non_camel_case_types)]
                    struct QueryRateLimitByAddressSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryRateLimitByAddressRequest>
                        for QueryRateLimitByAddressSvc<T>
                    {
                        type Response = super::QueryRateLimitByAddressResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRateLimitByAddressRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).query_rate_limit_by_address(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryRateLimitByAddressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: core::fmt::Debug> core::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "side.btcbridge.Query";
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn submit_deposit_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubmitDepositTransaction>,
        ) -> core::result::Result<
            tonic::Response<super::MsgSubmitDepositTransactionResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Msg/SubmitDepositTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Msg",
                "SubmitDepositTransaction",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn submit_withdraw_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubmitWithdrawTransaction>,
        ) -> core::result::Result<
            tonic::Response<super::MsgSubmitWithdrawTransactionResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Msg/SubmitWithdrawTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Msg",
                "SubmitWithdrawTransaction",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn submit_fee_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubmitFeeRate>,
        ) -> core::result::Result<tonic::Response<super::MsgSubmitFeeRateResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/side.btcbridge.Msg/SubmitFeeRate");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Msg", "SubmitFeeRate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_trusted_non_btc_relayers(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateTrustedNonBtcRelayers>,
        ) -> core::result::Result<
            tonic::Response<super::MsgUpdateTrustedNonBtcRelayersResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Msg/UpdateTrustedNonBtcRelayers",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Msg",
                "UpdateTrustedNonBtcRelayers",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_trusted_fee_providers(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateTrustedFeeProviders>,
        ) -> core::result::Result<
            tonic::Response<super::MsgUpdateTrustedFeeProvidersResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/side.btcbridge.Msg/UpdateTrustedFeeProviders",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "side.btcbridge.Msg",
                "UpdateTrustedFeeProviders",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn withdraw_to_bitcoin(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawToBitcoin>,
        ) -> core::result::Result<tonic::Response<super::MsgWithdrawToBitcoinResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/side.btcbridge.Msg/WithdrawToBitcoin");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Msg", "WithdrawToBitcoin"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn submit_signatures(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubmitSignatures>,
        ) -> core::result::Result<tonic::Response<super::MsgSubmitSignaturesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/side.btcbridge.Msg/SubmitSignatures");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Msg", "SubmitSignatures"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn consolidate_vaults(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgConsolidateVaults>,
        ) -> core::result::Result<tonic::Response<super::MsgConsolidateVaultsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/side.btcbridge.Msg/ConsolidateVaults");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Msg", "ConsolidateVaults"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn initiate_dkg(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgInitiateDkg>,
        ) -> core::result::Result<tonic::Response<super::MsgInitiateDkgResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/side.btcbridge.Msg/InitiateDKG");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Msg", "InitiateDKG"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn complete_dkg(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCompleteDkg>,
        ) -> core::result::Result<tonic::Response<super::MsgCompleteDkgResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/side.btcbridge.Msg/CompleteDKG");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Msg", "CompleteDKG"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn refresh(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRefresh>,
        ) -> core::result::Result<tonic::Response<super::MsgRefreshResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/side.btcbridge.Msg/Refresh");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Msg", "Refresh"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn complete_refreshing(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCompleteRefreshing>,
        ) -> core::result::Result<
            tonic::Response<super::MsgCompleteRefreshingResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/side.btcbridge.Msg/CompleteRefreshing");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Msg", "CompleteRefreshing"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn transfer_vault(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTransferVault>,
        ) -> core::result::Result<tonic::Response<super::MsgTransferVaultResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/side.btcbridge.Msg/TransferVault");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Msg", "TransferVault"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_params(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateParams>,
        ) -> core::result::Result<tonic::Response<super::MsgUpdateParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    alloc::format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/side.btcbridge.Msg/UpdateParams");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("side.btcbridge.Msg", "UpdateParams"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        async fn submit_deposit_transaction(
            &self,
            request: tonic::Request<super::MsgSubmitDepositTransaction>,
        ) -> core::result::Result<
            tonic::Response<super::MsgSubmitDepositTransactionResponse>,
            tonic::Status,
        >;
        async fn submit_withdraw_transaction(
            &self,
            request: tonic::Request<super::MsgSubmitWithdrawTransaction>,
        ) -> core::result::Result<
            tonic::Response<super::MsgSubmitWithdrawTransactionResponse>,
            tonic::Status,
        >;
        async fn submit_fee_rate(
            &self,
            request: tonic::Request<super::MsgSubmitFeeRate>,
        ) -> core::result::Result<tonic::Response<super::MsgSubmitFeeRateResponse>, tonic::Status>;
        async fn update_trusted_non_btc_relayers(
            &self,
            request: tonic::Request<super::MsgUpdateTrustedNonBtcRelayers>,
        ) -> core::result::Result<
            tonic::Response<super::MsgUpdateTrustedNonBtcRelayersResponse>,
            tonic::Status,
        >;
        async fn update_trusted_fee_providers(
            &self,
            request: tonic::Request<super::MsgUpdateTrustedFeeProviders>,
        ) -> core::result::Result<
            tonic::Response<super::MsgUpdateTrustedFeeProvidersResponse>,
            tonic::Status,
        >;
        async fn withdraw_to_bitcoin(
            &self,
            request: tonic::Request<super::MsgWithdrawToBitcoin>,
        ) -> core::result::Result<tonic::Response<super::MsgWithdrawToBitcoinResponse>, tonic::Status>;
        async fn submit_signatures(
            &self,
            request: tonic::Request<super::MsgSubmitSignatures>,
        ) -> core::result::Result<tonic::Response<super::MsgSubmitSignaturesResponse>, tonic::Status>;
        async fn consolidate_vaults(
            &self,
            request: tonic::Request<super::MsgConsolidateVaults>,
        ) -> core::result::Result<tonic::Response<super::MsgConsolidateVaultsResponse>, tonic::Status>;
        async fn initiate_dkg(
            &self,
            request: tonic::Request<super::MsgInitiateDkg>,
        ) -> core::result::Result<tonic::Response<super::MsgInitiateDkgResponse>, tonic::Status>;
        async fn complete_dkg(
            &self,
            request: tonic::Request<super::MsgCompleteDkg>,
        ) -> core::result::Result<tonic::Response<super::MsgCompleteDkgResponse>, tonic::Status>;
        async fn refresh(
            &self,
            request: tonic::Request<super::MsgRefresh>,
        ) -> core::result::Result<tonic::Response<super::MsgRefreshResponse>, tonic::Status>;
        async fn complete_refreshing(
            &self,
            request: tonic::Request<super::MsgCompleteRefreshing>,
        ) -> core::result::Result<
            tonic::Response<super::MsgCompleteRefreshingResponse>,
            tonic::Status,
        >;
        async fn transfer_vault(
            &self,
            request: tonic::Request<super::MsgTransferVault>,
        ) -> core::result::Result<tonic::Response<super::MsgTransferVaultResponse>, tonic::Status>;
        async fn update_params(
            &self,
            request: tonic::Request<super::MsgUpdateParams>,
        ) -> core::result::Result<tonic::Response<super::MsgUpdateParamsResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<core::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/side.btcbridge.Msg/SubmitDepositTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitDepositTransactionSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSubmitDepositTransaction>
                        for SubmitDepositTransactionSvc<T>
                    {
                        type Response = super::MsgSubmitDepositTransactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSubmitDepositTransaction>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).submit_deposit_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubmitDepositTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/SubmitWithdrawTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitWithdrawTransactionSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSubmitWithdrawTransaction>
                        for SubmitWithdrawTransactionSvc<T>
                    {
                        type Response = super::MsgSubmitWithdrawTransactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSubmitWithdrawTransaction>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).submit_withdraw_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubmitWithdrawTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/SubmitFeeRate" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitFeeRateSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSubmitFeeRate> for SubmitFeeRateSvc<T> {
                        type Response = super::MsgSubmitFeeRateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSubmitFeeRate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).submit_fee_rate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubmitFeeRateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/UpdateTrustedNonBtcRelayers" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTrustedNonBtcRelayersSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateTrustedNonBtcRelayers>
                        for UpdateTrustedNonBtcRelayersSvc<T>
                    {
                        type Response = super::MsgUpdateTrustedNonBtcRelayersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateTrustedNonBtcRelayers>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_trusted_non_btc_relayers(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateTrustedNonBtcRelayersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/UpdateTrustedFeeProviders" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTrustedFeeProvidersSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateTrustedFeeProviders>
                        for UpdateTrustedFeeProvidersSvc<T>
                    {
                        type Response = super::MsgUpdateTrustedFeeProvidersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateTrustedFeeProviders>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { (*inner).update_trusted_fee_providers(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateTrustedFeeProvidersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/WithdrawToBitcoin" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawToBitcoinSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdrawToBitcoin> for WithdrawToBitcoinSvc<T> {
                        type Response = super::MsgWithdrawToBitcoinResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdrawToBitcoin>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).withdraw_to_bitcoin(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WithdrawToBitcoinSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/SubmitSignatures" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitSignaturesSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSubmitSignatures> for SubmitSignaturesSvc<T> {
                        type Response = super::MsgSubmitSignaturesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSubmitSignatures>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).submit_signatures(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubmitSignaturesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/ConsolidateVaults" => {
                    #[allow(non_camel_case_types)]
                    struct ConsolidateVaultsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgConsolidateVaults> for ConsolidateVaultsSvc<T> {
                        type Response = super::MsgConsolidateVaultsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgConsolidateVaults>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).consolidate_vaults(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConsolidateVaultsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/InitiateDKG" => {
                    #[allow(non_camel_case_types)]
                    struct InitiateDKGSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgInitiateDkg> for InitiateDKGSvc<T> {
                        type Response = super::MsgInitiateDkgResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgInitiateDkg>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).initiate_dkg(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InitiateDKGSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/CompleteDKG" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteDKGSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCompleteDkg> for CompleteDKGSvc<T> {
                        type Response = super::MsgCompleteDkgResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCompleteDkg>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).complete_dkg(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CompleteDKGSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/Refresh" => {
                    #[allow(non_camel_case_types)]
                    struct RefreshSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgRefresh> for RefreshSvc<T> {
                        type Response = super::MsgRefreshResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRefresh>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).refresh(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RefreshSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/CompleteRefreshing" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteRefreshingSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCompleteRefreshing>
                        for CompleteRefreshingSvc<T>
                    {
                        type Response = super::MsgCompleteRefreshingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCompleteRefreshing>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).complete_refreshing(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CompleteRefreshingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/TransferVault" => {
                    #[allow(non_camel_case_types)]
                    struct TransferVaultSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgTransferVault> for TransferVaultSvc<T> {
                        type Response = super::MsgTransferVaultResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgTransferVault>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).transfer_vault(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferVaultSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/side.btcbridge.Msg/UpdateParams" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateParamsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateParams> for UpdateParamsSvc<T> {
                        type Response = super::MsgUpdateParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateParams>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: core::fmt::Debug> core::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "side.btcbridge.Msg";
    }
}
