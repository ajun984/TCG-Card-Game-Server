use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::account::service::account_service::AccountService;
use crate::account::service::account_service_impl::AccountServiceImpl;
use crate::fake_battle_room::controller::fake_battle_room_controller::FakeBattleRoomController;
use crate::fake_battle_room::controller::request_form::create_fake_battle_room_request_form::CreateFakeBattleRoomRequestForm;
use crate::fake_battle_room::controller::request_form::fake_multi_draw_request_form::FakeMultiDrawRequestForm;
use crate::fake_battle_room::controller::response_form::create_fake_battle_room_response_form::CreateFakeBattleRoomResponseForm;
use crate::fake_battle_room::controller::response_form::fake_multi_draw_response_form::FakeMultiDrawResponseForm;
use crate::fake_battle_room::service::fake_battle_room_service::FakeBattleRoomService;
use crate::fake_battle_room::service::fake_battle_room_service_impl::FakeBattleRoomServiceImpl;
use crate::game_card_unit::controller::response_form::deploy_unit_response_form::DeployUnitResponseForm;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_turn::controller::response_form::turn_end_response_form::TurnEndResponseForm;
use crate::mulligan::controller::response_form::mulligan_response_form::MulliganResponseForm;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::service::ui_data_generator_service::UiDataGeneratorService;
use crate::ui_data_generator::service::ui_data_generator_service_impl::UiDataGeneratorServiceImpl;

pub struct FakeBattleRoomControllerImpl {
    account_service: Arc<AsyncMutex<AccountServiceImpl>>,
    game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    fake_battle_room_service: Arc<AsyncMutex<FakeBattleRoomServiceImpl>>,
    ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
}

impl FakeBattleRoomControllerImpl {
    pub fn new(account_service: Arc<AsyncMutex<AccountServiceImpl>>,
               game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
               game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               fake_battle_room_service: Arc<AsyncMutex<FakeBattleRoomServiceImpl>>,
               ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>) -> Self {

        FakeBattleRoomControllerImpl {
            account_service,
            game_deck_service,
            game_hand_service,
            redis_in_memory_service,
            fake_battle_room_service,
            ui_data_generator_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<FakeBattleRoomControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<FakeBattleRoomControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        FakeBattleRoomControllerImpl::new(
                            AccountServiceImpl::get_instance(),
                            GameDeckServiceImpl::get_instance(),
                            GameHandServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            FakeBattleRoomServiceImpl::get_instance(),
                            UiDataGeneratorServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;

        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }
}

#[async_trait]
impl FakeBattleRoomController for FakeBattleRoomControllerImpl {
    async fn request_to_create_fake_battle_room(
        &self,
        create_fake_battle_room_request_form: CreateFakeBattleRoomRequestForm)
            -> CreateFakeBattleRoomResponseForm {

        let account_service_guard = self.account_service.lock().await;
        let first_login_response =
            account_service_guard.account_login(
                create_fake_battle_room_request_form.to_first_fake_test_login_request()).await;

        let second_login_response =
            account_service_guard.account_login(
                create_fake_battle_room_request_form.to_second_fake_test_login_request()).await;

        let fake_your_session = first_login_response.get_redis_token();
        let fake_opponent_session = second_login_response.get_redis_token();

        let fake_your_unique_id =
            self.is_valid_session(
                create_fake_battle_room_request_form
                    .to_session_validation_request(
                        fake_your_session)).await;

        if fake_your_unique_id == -1 {
            return CreateFakeBattleRoomResponseForm::new("".to_string(), "".to_string());
        }

        let fake_opponent_unique_id =
            self.is_valid_session(
                create_fake_battle_room_request_form
                    .to_session_validation_request(
                        fake_opponent_session)).await;

        if fake_opponent_unique_id == -1 {
            return CreateFakeBattleRoomResponseForm::new("".to_string(), "".to_string());
        }

        let fake_battle_room_service_guard = self.fake_battle_room_service.lock().await;
        let create_fake_battle_room_response =
            fake_battle_room_service_guard.create_fake_battle_room(
                create_fake_battle_room_request_form
                    .to_create_fake_battle_room_request(
                        fake_your_unique_id,
                        fake_opponent_unique_id)).await;

        return CreateFakeBattleRoomResponseForm::from_login_response(
            first_login_response,
            second_login_response)
    }

    async fn request_to_fake_multi_draw(
        &self,
        fake_multi_draw_request_form: FakeMultiDrawRequestForm)
            -> FakeMultiDrawResponseForm {

        let account_unique_id = self.is_valid_session(
            fake_multi_draw_request_form
                .to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("Invalid session");
            return FakeMultiDrawResponseForm::new(HashMap::new())
        }

        let mut game_deck_service_guard =
            self.game_deck_service.lock().await;

        let draw_deck_response =
            game_deck_service_guard.draw_cards_from_deck(
                fake_multi_draw_request_form
                    .to_draw_cards_from_deck_request(
                        account_unique_id,
                        20)).await;

        let drawn_card_list = draw_deck_response.get_drawn_card_list().clone();
        drop(game_deck_service_guard);

        let mut game_hand_service_guard =
            self.game_hand_service.lock().await;

        game_hand_service_guard.add_card_list_to_hand(
            fake_multi_draw_request_form
                .to_add_card_list_to_hand_request(account_unique_id, drawn_card_list.clone())).await;

        drop(game_hand_service_guard);

        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_draw_my_deck_data_response =
            ui_data_generator_service_guard.generate_draw_my_deck_data(
                fake_multi_draw_request_form
                    .to_generate_draw_my_deck_data_request(drawn_card_list)).await;

        drop(ui_data_generator_service_guard);

        return FakeMultiDrawResponseForm::from_response(
            generate_draw_my_deck_data_response)

    }
}
