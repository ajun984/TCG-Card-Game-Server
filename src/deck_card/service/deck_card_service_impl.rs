use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::common::converter::vector_to_hash_converter::VectorToHashConverter;
use crate::deck_card::repository::deck_card_repository::DeckCardRepository;

use crate::deck_card::repository::deck_card_repository_impl::DeckCardRepositoryImpl;
use crate::deck_card::service::deck_card_service::DeckCardService;
use crate::deck_card::service::request::deck_card_list_request::DeckCardListRequest;
use crate::deck_card::service::request::deck_configuration_request::DeckConfigurationRequest;
use crate::deck_card::service::response::deck_card_list_response::DeckCardListResponse;
use crate::deck_card::service::response::deck_configuration_response::DeckConfigurationResponse;

pub struct DeckCardServiceImpl {
    repository: Arc<AsyncMutex<DeckCardRepositoryImpl>>
}

impl DeckCardServiceImpl {
    pub fn new(repository: Arc<AsyncMutex<DeckCardRepositoryImpl>>) -> Self {
        DeckCardServiceImpl {
            repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<DeckCardServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<DeckCardServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        DeckCardServiceImpl::new(
                            DeckCardRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn check_deck_card_list_count(&self, deck: &Vec<i32>) -> Result<(), DeckConfigurationResponse> {
        match deck.len() {
            40 => {
                Ok(())
            }
            length => {
                // 40장이 아닌 경우
                let error_string = format!("덱에 총 {}장이 있습니다. 정확히 40장을 맞춰주세요!", length);
                println!("{}", error_string);

                return Err(DeckConfigurationResponse::new(false, error_string));
            }
        }
    }

    async fn validate_deck(&self, request_deck_list: &Vec<i32>) -> Result<(), DeckConfigurationResponse> {
        let mut card_count_map = HashMap::new();

        // TODO: Energy Card 의 경우 3장 넘어가도 됩니다.
        // TODO: 신화, 전설, 영웅, 언커먼, 일반 덱 구성 규칙 지켰는지 파악해야합니다.
        for card_id in request_deck_list.iter() {
            let card_counts = card_count_map.entry(card_id).or_insert(0);
            *card_counts += 1;

            if *card_id == 93 {
                println!("일반 && 에너지 카드는 수량에 상관없이 추가 할 수 있습니다");
                continue;
            }

            if *card_counts > 3 {
                let error_string =
                    format!("{}번 카드가 3장이 넘습니다. 같은 카드는 덱에 3장 이하여야 합니다!", card_id);
                println!("{}", error_string);

                return Err(DeckConfigurationResponse::new(false, error_string));
            }
        }

        Ok(())
    }
}

#[async_trait]
impl DeckCardService for DeckCardServiceImpl {
    async fn deck_configuration_register(&self, deck_configuration_request: DeckConfigurationRequest) -> DeckConfigurationResponse {
        println!("DeckCardServiceImpl: deck_configuration_register()");

        let deck_card_id_vector = deck_configuration_request.card_id_list_of_deck();

        if let Err(error_response) = self.check_deck_card_list_count(deck_card_id_vector).await {
            return error_response;
        }

        // if let Err(error_response) = self.validate_deck(deck_card_id_vector).await {
        //     return error_response;
        // }

        let deck_card_vector = deck_configuration_request.to_deck_card_list();

        let deck_card_repository = self.repository.lock().await;
        let result = deck_card_repository.save_deck_card_list(deck_card_vector).await;
        match result {
            Ok(success_message) => {
                DeckConfigurationResponse::new(true, success_message)
            }
            Err(error_message) => {
                DeckConfigurationResponse::new(false, error_message)
            }
        }
    }
    async fn deck_card_list(&self, deck_card_list_request: DeckCardListRequest) -> DeckCardListResponse {
        println!("DeckCardServiceImpl: deck_card_list()");

        let deck_card_repository = self.repository.lock().await;
        let deck_id_i32 = deck_card_list_request.deck_id();
        let result = deck_card_repository.get_card_list(deck_id_i32).await;
        match result {
            Ok(opt_list) => {
                let card_id_count_list = opt_list.unwrap();
                DeckCardListResponse::new(card_id_count_list)
            }
            Err(e) => {
                let empty_list = Vec::new();
                DeckCardListResponse::new(empty_list)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;
    use crate::deck_card::repository::deck_card_repository_impl::DeckCardRepositoryImpl;

    use crate::deck_card;

    #[tokio::test]
    async fn test_deck_config_save() {
        let deck_card_service_mutex = DeckCardServiceImpl::get_instance();
        let deck_card_service_mutex_guard = deck_card_service_mutex.lock().await;

        let deck_id = 18118;

        // let card_id_list_very_long = [1, 1, 1, 2, 2, 3, 3, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7,
        //     8, 8, 9, 9, 9, 11, 11, 11, 12, 12, 12, 13, 13, 13, 14, 14, 14, 15, 15, 15, 16, 16, 16, 17, 17, 17];
        // let card_id_list_too_many_duplicated_cards = [1, 1, 1, 1, 2, 2, 3, 3, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7,
        //     8, 8, 9, 9, 9, 11, 11, 11, 12, 12, 12];
        let test_card_id_list
            = [1, 1, 1, 2, 2, 3, 3, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 9,
            9, 9, 11, 11, 11, 12, 12, 12, 13, 13, 13, 14, 14, 14, 15, 15, 15, 16, 17, 18,];

        let mut card_vec = Vec::new();
        for id in test_card_id_list {
            card_vec.push(id);
        }
        let deck_config_request = DeckConfigurationRequest::new(deck_id, card_vec);

        let result = deck_card_service_mutex_guard.deck_configuration_register(deck_config_request).await;
        println!("is_success: {}", result.get_is_success());
        println!("message: {}", result.get_message());
    }
    #[tokio::test]
    async fn test_deck_card_list() {
        let deck_card_service_mutex = DeckCardServiceImpl::get_instance();
        let deck_card_service_mutex_guard = deck_card_service_mutex.lock().await;

        let deck_card_list_request = DeckCardListRequest::new(8);

        let result = deck_card_service_mutex_guard.deck_card_list(deck_card_list_request).await;
        println!("{:?}", result.get_card_id_list());
    }

    #[tokio::test]
    #[cfg(not(feature = "deck_card_test"))]
    async fn dummy_test() {
        assert!(true);
    }

    #[test]
    async fn test_deck_configuration_register() {
        // DELETE FROM deck_cards WHERE deck_id = 7777;
        let card_list = vec![
            19, 8, 8, 8, 9, 9, 25, 25, 25, 27, 27, 27, 151, 20, 20, 20, 2, 2, 2,
            26, 26, 26, 30, 31, 31, 31, 32, 32, 32, 33, 33, 35, 35, 36, 36, 93, 93, 93, 93, 93,
        ];

        let mut deck_configuration_request = DeckConfigurationRequest::new(7777, card_list);

        let deck_card_service = DeckCardServiceImpl::get_instance();
        let deck_card_service_guard = deck_card_service.lock().await;

        let result = deck_card_service_guard.deck_configuration_register(deck_configuration_request.clone()).await;

        assert_eq!(result.get_is_success(), true);
    }
}