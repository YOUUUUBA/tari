// Copyright 2021. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::{
    error::WalletError,
    tokens::{
        infrastructure::{TokenManagerRequest, TokenManagerResponse},
        Token,
    },
};
use tari_service_framework::{reply_channel::SenderService, Service};

use tari_core::transactions::{transaction::Transaction, transaction_protocol::TxId};

#[derive(Clone)]
pub struct TokenManagerHandle {
    handle: SenderService<TokenManagerRequest, Result<TokenManagerResponse, WalletError>>,
}

impl TokenManagerHandle {
    pub fn new(sender: SenderService<TokenManagerRequest, Result<TokenManagerResponse, WalletError>>) -> Self {
        Self { handle: sender }
    }

    pub async fn list_owned_tokens(&mut self) -> Result<Vec<Token>, WalletError> {
        match self.handle.call(TokenManagerRequest::ListOwned {}).await?? {
            TokenManagerResponse::ListOwned { tokens } => Ok(tokens),
            /* _ => Err(WalletError::UnexpectedApiResponse{ method: "list_owned_tokens".to_string(), api:
             * "TokenManagerService".to_string()}), */
        }
    }
}
