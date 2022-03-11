use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Timestamp};
near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    requesters: UnorderedSet<AccountId>,
    providers: UnorderedSet<AccountId>,
    requests: UnorderedMap<String, Request>,
    responses: UnorderedMap<String, Response>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner: env::predecessor_account_id(),
            requesters: UnorderedSet::new(b"requester".to_vec()),
            providers: UnorderedSet::new(b"provider".to_vec()),
            requests: UnorderedMap::new(b"requests".to_vec()),
            responses: UnorderedMap::new(b"responses".to_vec()),
        }
    }

    pub fn add_new_requesters(&mut self, requesters: Vec<AccountId>) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Caller is not owner"
        );
        for requester in requesters.iter() {
            self.requesters.insert(requester);
        }
    }

    pub fn add_new_providers(&mut self, providers: Vec<AccountId>) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Caller is not owner"
        );
        for provider in providers.iter() {
            self.providers.insert(provider);
        }
    }

    pub fn create_request(&mut self, request_id: &String, request: &Request) {
        assert!(
            self.requesters.contains(&env::predecessor_account_id()),
            "Only requesters are allowed to create request"
        );
        self.requests.insert(request_id, request);
    }

    pub fn provide_data(&mut self, request_id: &String, response_string: &String) {
        assert!(
            self.providers.contains(&env::predecessor_account_id()),
            "Only providers are allowed to create request"
        );
        self.responses.insert(
            request_id,
            &Response {
                result: response_string.clone(),
                timestamp: env::block_timestamp(),
            },
        );
    }

    pub fn get_all_requests(&self) -> Vec<Request> {
        self.requests.values().collect()
    }

    pub fn get_data_response(&self, request_id: &String) -> Option<Response> {
        self.responses.get(request_id)
    }
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Request {
    pub request_id: String,
    pub json_path: String,
    pub uri: String,
    pub period: Option<u64>,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Response {
    pub result: String,
    pub timestamp: Timestamp,
}

#[cfg(not(target = "wasm32"))]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
