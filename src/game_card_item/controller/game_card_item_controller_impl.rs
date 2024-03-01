use std::sync::Arc;
use async_trait::async_trait;
use diesel::IntoSql;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;

use crate::card_grade::service::card_grade_service::CardGradeService;
use crate::card_grade::service::card_grade_service_impl::CardGradeServiceImpl;
use crate::card_race::service::card_race_service::CardRaceService;
use crate::card_race::service::card_race_service_impl::CardRaceServiceImpl;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::converter::vector_string_to_vector_integer::VectorStringToVectorInteger;

use crate::game_card_item::controller::game_card_item_controller::GameCardItemController;
use crate::game_card_item::controller::request_form::add_field_energy_with_field_unit_health_point_item_request_form::AddFieldEnergyWithFieldUnitHealthPointRequestForm;
use crate::game_card_item::controller::request_form::multiple_target_damage_by_field_unit_death_item_request_form::MultipleTargetDamageByFieldUnitDeathItemRequestForm;
use crate::game_card_item::controller::request_form::catastrophic_damage_item_request_form::CatastrophicDamageItemRequestForm;
use crate::game_card_item::controller::request_form::remove_opponent_field_unit_energy_item_request_form::RemoveOpponentFieldUnitEnergyItemRequestForm;
use crate::game_card_item::controller::request_form::target_death_item_request_form::TargetDeathItemRequestForm;
use crate::game_card_item::controller::response_form::add_field_energy_with_field_unit_health_point_item_response_form::AddFieldEnergyWithFieldUnitHealthPointResponseForm;
use crate::game_card_item::controller::response_form::multiple_target_damage_by_field_unit_death_item_response_form::MultipleTargetDamageByFieldUnitDeathItemResponseForm;
use crate::game_card_item::controller::response_form::catastrophic_damage_item_response_form::CatastrophicDamageItemResponseForm;
use crate::game_card_item::controller::response_form::remove_opponent_field_unit_energy_item_response_form::RemoveOpponentFieldUnitEnergyItemResponseForm;
use crate::game_card_item::controller::response_form::target_death_item_response_form::TargetDeathItemResponseForm;
use crate::game_card_item::service::game_card_item_service::GameCardItemService;

use crate::game_card_item::service::game_card_item_service_impl::GameCardItemServiceImpl;
use crate::game_card_item::service::request::summary_item_card_effect_request::SummaryItemCardEffectRequest;
use crate::game_card_item::service::response::summary_item_card_effect_response::SummaryItemCardEffectResponse;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;

use crate::game_field_energy::service::game_field_energy_service::GameFieldEnergyService;
use crate::game_field_energy::service::game_field_energy_service_impl::GameFieldEnergyServiceImpl;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_hand::service::request::use_game_hand_item_card_request::UseGameHandItemCardRequest;
use crate::game_lost_zone::service::game_lost_zone_service::GameLostZoneService;
use crate::game_lost_zone::service::game_lost_zone_service_impl::GameLostZoneServiceImpl;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::game_main_character::service::game_main_character_service::GameMainCharacterService;
use crate::game_main_character::service::game_main_character_service_impl::GameMainCharacterServiceImpl;

use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_item_card_request::IsItItemCardRequest;

use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::game_turn::controller::response_form::turn_end_response_form::TurnEndResponseForm;
use crate::notify_player_action::service::notify_player_action_service::NotifyPlayerActionService;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::notify_player_action_info::service::notify_player_action_info_service::NotifyPlayerActionInfoService;
use crate::notify_player_action_info::service::notify_player_action_info_service_impl::NotifyPlayerActionInfoServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::service::ui_data_generator_service::UiDataGeneratorService;
use crate::ui_data_generator::service::ui_data_generator_service_impl::UiDataGeneratorServiceImpl;

pub struct GameCardItemControllerImpl {
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
    card_grade_service: Arc<AsyncMutex<CardGradeServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_card_item_service: Arc<AsyncMutex<GameCardItemServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
    game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,
    game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
    game_lost_zone_service: Arc<AsyncMutex<GameLostZoneServiceImpl>>,
    card_race_service: Arc<AsyncMutex<CardRaceServiceImpl>>,
    notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
    ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
}

impl GameCardItemControllerImpl {
    pub fn new(game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
               card_grade_service: Arc<AsyncMutex<CardGradeServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_card_item_service: Arc<AsyncMutex<GameCardItemServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
               game_main_character_service: Arc<AsyncMutex<GameMainCharacterServiceImpl>>,
               game_deck_service: Arc<AsyncMutex<GameDeckServiceImpl>>,
               game_lost_zone_service: Arc<AsyncMutex<GameLostZoneServiceImpl>>,
               card_race_service: Arc<AsyncMutex<CardRaceServiceImpl>>,
               notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
               ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,) -> Self {

        GameCardItemControllerImpl {
            game_hand_service,
            game_tomb_service,
            card_grade_service,
            battle_room_service,
            game_card_item_service,
            game_field_unit_service,
            game_protocol_validation_service,
            redis_in_memory_service,
            notify_player_action_service,
            game_field_energy_service,
            game_main_character_service,
            game_deck_service,
            game_lost_zone_service,
            card_race_service,
            notify_player_action_info_service,
            ui_data_generator_service
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardItemControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardItemControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardItemControllerImpl::new(
                            GameHandServiceImpl::get_instance(),
                            GameTombServiceImpl::get_instance(),
                            CardGradeServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            GameCardItemServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameFieldEnergyServiceImpl::get_instance(),
                            GameMainCharacterServiceImpl::get_instance(),
                            GameDeckServiceImpl::get_instance(),
                            GameLostZoneServiceImpl::get_instance(),
                            CardRaceServiceImpl::get_instance(),
                            NotifyPlayerActionInfoServiceImpl::get_instance(),
                            UiDataGeneratorServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    // TODO: 모든 Controller는 검증 로직 때문에 반복을 줄이기 위해 추후 Aspect 처리 필요함
    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;
        drop(redis_in_memory_service_guard);
        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }

    async fn is_valid_protocol(&self, check_protocol_hacking_request: CheckProtocolHackingRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let check_protocol_hacking_response = game_protocol_validation_service_guard.check_protocol_hacking(check_protocol_hacking_request).await;
        drop(game_protocol_validation_service_guard);
        check_protocol_hacking_response.is_success()
    }

    async fn is_it_item_card(&self, is_it_item_card_request: IsItItemCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let is_it_item_card_response = game_protocol_validation_service_guard.is_it_item_card(is_it_item_card_request).await;
        drop(game_protocol_validation_service_guard);
        is_it_item_card_response.is_success()
    }

    async fn is_able_to_use(&self, can_use_card_request: CanUseCardRequest) -> bool {
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let can_use_card_response = game_protocol_validation_service_guard.can_use_card(can_use_card_request).await;
        drop(game_protocol_validation_service_guard);
        can_use_card_response.is_success()
    }

    async fn use_item_card(&self, use_game_hand_item_card_request: UseGameHandItemCardRequest) -> i32 {
        let mut game_hand_service_guard = self.game_hand_service.lock().await;
        let use_game_hand_item_card_response = game_hand_service_guard.use_item_card(use_game_hand_item_card_request).await;
        drop(game_hand_service_guard);
        use_game_hand_item_card_response.get_found_item_card_id()
    }

    async fn place_used_card_to_tomb(&self, place_to_tomb_request: PlaceToTombRequest) {
        let mut game_tomb_service_guard = self.game_tomb_service.lock().await;
        game_tomb_service_guard.add_used_card_to_tomb(place_to_tomb_request).await;
    }

    async fn get_summary_of_item_card(&self, summary_item_card_effect_request: SummaryItemCardEffectRequest) -> SummaryItemCardEffectResponse {
        let mut game_card_item_service_guard = self.game_card_item_service.lock().await;
        let summary_item_card_effect_response = game_card_item_service_guard.summary_item_card(summary_item_card_effect_request).await;
        drop(game_card_item_service_guard);
        summary_item_card_effect_response
    }
}

#[async_trait]
impl GameCardItemController for GameCardItemControllerImpl {
    async fn request_to_use_target_death_item(
        &self, target_death_item_request_form: TargetDeathItemRequestForm)
        -> TargetDeathItemResponseForm {

        println!("GameCardItemControllerImpl: request_to_use_target_death_item()");

        // 1. Redis 에서 토큰을 가지고 있는지 검증
        let account_unique_id = self.is_valid_session(
            target_death_item_request_form
                .to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("Invalid session");
            return TargetDeathItemResponseForm::default()
        }

        // 2. 자신의 턴인지 검증
        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                target_death_item_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return TargetDeathItemResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        let item_card_id_string = target_death_item_request_form.get_item_card_id();
        let item_card_id = item_card_id_string.parse::<i32>().unwrap();

        // 3. Hand 에 있는지 확인하여 해킹 여부 검증
        let check_protocol_hacking_response = self.is_valid_protocol(
            target_death_item_request_form
                .to_check_protocol_hacking_request(account_unique_id, item_card_id)).await;

        if !check_protocol_hacking_response {
            println!("해킹범을 검거합니다!");
            return TargetDeathItemResponseForm::default()
        }

        // 4. 실제 아이템 카드가 맞는지 확인
        let is_it_item_response = self.is_it_item_card(
            target_death_item_request_form
                .to_is_it_item_card_request(item_card_id)).await;

        if !is_it_item_response {
            println!("아이템 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return TargetDeathItemResponseForm::default()
        }

        // 5. GameProtocolValidation Service 호출하여 사용 가능한지 조건 검사 (신화 > 4라운드 제약)
        let can_use_card_response = self.is_able_to_use(
            target_death_item_request_form
                .to_can_use_card_request(account_unique_id, item_card_id)).await;

        if !can_use_card_response {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return TargetDeathItemResponseForm::default()
        }

        // 6. 효과를 적용하기 위해 Game Card Item Service 호출하여 필요 효과 설정
        let summarized_item_effect_response = self.get_summary_of_item_card(
            target_death_item_request_form
                .to_summary_item_effect_request(item_card_id)).await;

        // TODO: 카드 사용 시 필요한 에너지가 존재한다면 이에 대한 추가 검증이 필요

        // 7. Hand Service 호출하여 카드 사용
        let usage_hand_card = self.use_item_card(
            target_death_item_request_form
                .to_use_game_hand_item_card_request(account_unique_id, item_card_id)).await;

        // 8. Item 카드 사용이므로 Tomb Service 호출하여 무덤 배치
        self.place_used_card_to_tomb(
            target_death_item_request_form
                .to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        // 9. 즉사 스킬 적용을 위해 상대방의 고유 id 값을 확보
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                target_death_item_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let opponent_target_unit_index_string =
            target_death_item_request_form.get_opponent_target_unit_index();
        let opponent_target_unit_index =
            opponent_target_unit_index_string.parse::<i32>().unwrap();

        // TODO: 추후 즉사 면역인 언데드 등등에 대한 조건도 필요함
        // 10. 타겟 인덱스 유닛이 신화 미만인지 확인
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let find_target_unit_id_by_index_response =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                target_death_item_request_form
                    .to_find_target_unit_id_by_index_request(
                        opponent_unique_id,
                        opponent_target_unit_index)).await;

        if find_target_unit_id_by_index_response.get_found_opponent_unit_id() == -1 {
            println!("필드 위에 존재하지 않는 대상에 대한 요청이므로 당신도 해킹범입니다.");
            return TargetDeathItemResponseForm::default()
        }

        let card_grade_service_guard =
            self.card_grade_service.lock().await;

        let opponent_target_unit_grade =
            card_grade_service_guard.get_card_grade(
                &find_target_unit_id_by_index_response.get_found_opponent_unit_id()).await;

        drop(card_grade_service_guard);

        // 11. 신화 등급인 경우 Field Unit Service 를 호출하여 상대 유닛에 Alternatives 적용
        if opponent_target_unit_grade == GradeEnum::Mythical {
            game_field_unit_service_guard.apply_damage_to_target_unit_index(
                target_death_item_request_form.to_apply_damage_to_target_unit_index(
                    opponent_unique_id,
                    opponent_target_unit_index,
                    summarized_item_effect_response.get_alternatives_damage())).await;

            let updated_health_point =
                game_field_unit_service_guard.get_current_health_point_of_field_unit_by_index(
                    target_death_item_request_form
                        .to_get_current_health_point_of_field_unit_by_index_request(
                            opponent_unique_id,
                            opponent_target_unit_index)).await.get_current_unit_health_point();

            let maybe_dead_unit_index =
                game_field_unit_service_guard.judge_death_of_unit(
                    target_death_item_request_form
                        .to_judge_death_of_unit_request(
                            opponent_unique_id,
                            opponent_target_unit_index)).await.get_dead_unit_index();

            drop(game_field_unit_service_guard);

            let mut ui_data_generator_service_guard =
                self.ui_data_generator_service.lock().await;

            let generate_use_my_hand_card_data_response =
                ui_data_generator_service_guard.generate_use_my_hand_card_data(
                    target_death_item_request_form
                        .to_generate_use_my_hand_card_data_request(
                            usage_hand_card)).await;

            let generate_opponent_specific_unit_health_point_data_response =
                ui_data_generator_service_guard.generate_opponent_specific_unit_health_point_data(
                    target_death_item_request_form
                        .to_generate_opponent_specific_unit_health_point_data_request(
                            opponent_target_unit_index,
                            updated_health_point)).await;

            let generate_opponent_specific_unit_death_data_response =
                ui_data_generator_service_guard.generate_opponent_specific_unit_death_data(
                    target_death_item_request_form
                        .to_generate_opponent_specific_unit_death_data_request(
                            maybe_dead_unit_index)).await;

            drop(ui_data_generator_service_guard);

            let mut notify_player_action_info_service_guard =
                self.notify_player_action_info_service.lock().await;

            let notice_response =
                notify_player_action_info_service_guard.notice_use_instant_unit_death_item_card(
                    target_death_item_request_form
                        .to_notice_use_instant_unit_death_item_card_request(
                            opponent_unique_id,
                            generate_use_my_hand_card_data_response
                                .get_player_hand_use_map_for_notice().clone(),
                            generate_opponent_specific_unit_health_point_data_response
                                .get_player_field_unit_health_point_map_for_notice().clone(),
                            generate_opponent_specific_unit_death_data_response
                                .get_player_field_unit_death_map_for_notice().clone())).await;

            println!("notice_response: {:?}", notice_response);

            drop(notify_player_action_info_service_guard);

            return TargetDeathItemResponseForm::from_response(
                generate_use_my_hand_card_data_response,
                generate_opponent_specific_unit_health_point_data_response,
                generate_opponent_specific_unit_death_data_response)
        }

        // 12. Field Unit Service 를 호출하여 상대 유닛에 즉사 적용
        game_field_unit_service_guard.apply_instant_death_to_target_unit_index(
            target_death_item_request_form
                .to_apply_instant_death_to_target_unit_index_request(
                    opponent_unique_id,
                    opponent_target_unit_index)).await;

        let updated_health_point =
            game_field_unit_service_guard.get_current_health_point_of_field_unit_by_index(
                target_death_item_request_form
                    .to_get_current_health_point_of_field_unit_by_index_request(
                        opponent_unique_id,
                        opponent_target_unit_index)).await.get_current_unit_health_point();

        let maybe_dead_unit_index =
            game_field_unit_service_guard.judge_death_of_unit(
                target_death_item_request_form
                    .to_judge_death_of_unit_request(
                        opponent_unique_id,
                        opponent_target_unit_index)).await.get_dead_unit_index();

        drop(game_field_unit_service_guard);

        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_use_my_hand_card_data_response =
            ui_data_generator_service_guard.generate_use_my_hand_card_data(
                target_death_item_request_form
                    .to_generate_use_my_hand_card_data_request(
                        usage_hand_card)).await;

        let generate_opponent_specific_unit_health_point_data_response =
            ui_data_generator_service_guard.generate_opponent_specific_unit_health_point_data(
                target_death_item_request_form
                    .to_generate_opponent_specific_unit_health_point_data_request(
                        opponent_target_unit_index,
                        updated_health_point)).await;

        let generate_opponent_specific_unit_death_data_response =
            ui_data_generator_service_guard.generate_opponent_specific_unit_death_data(
                target_death_item_request_form
                    .to_generate_opponent_specific_unit_death_data_request(
                        maybe_dead_unit_index)).await;

        drop(ui_data_generator_service_guard);

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        let notice_response =
            notify_player_action_info_service_guard.notice_use_instant_unit_death_item_card(
                target_death_item_request_form
                    .to_notice_use_instant_unit_death_item_card_request(
                        opponent_unique_id,
                        generate_use_my_hand_card_data_response
                            .get_player_hand_use_map_for_notice().clone(),
                        generate_opponent_specific_unit_health_point_data_response
                            .get_player_field_unit_health_point_map_for_notice().clone(),
                        generate_opponent_specific_unit_death_data_response
                            .get_player_field_unit_death_map_for_notice().clone())).await;

        println!("notice_response: {:?}", notice_response);

        drop(notify_player_action_info_service_guard);

        TargetDeathItemResponseForm::from_response(
            generate_use_my_hand_card_data_response,
            generate_opponent_specific_unit_health_point_data_response,
            generate_opponent_specific_unit_death_data_response)
    }

    async fn request_to_use_add_field_energy_with_field_unit_health_point(
        &self, add_field_energy_with_field_unit_health_point_request_form: AddFieldEnergyWithFieldUnitHealthPointRequestForm)
        -> AddFieldEnergyWithFieldUnitHealthPointResponseForm {

        println!("GameCardItemControllerImpl: request_to_use_add_field_energy_with_field_unit_health_point()");

        let account_unique_id = self.is_valid_session(
            add_field_energy_with_field_unit_health_point_request_form
                .to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("Invalid session");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::default()
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        let item_card_id_string =
            add_field_energy_with_field_unit_health_point_request_form.get_item_card_id();
        let item_card_id =
            item_card_id_string.parse::<i32>().unwrap();

        let check_protocol_hacking_response = self.is_valid_protocol(
            add_field_energy_with_field_unit_health_point_request_form
                .to_check_protocol_hacking_request(account_unique_id, item_card_id)).await;

        if !check_protocol_hacking_response {
            println!("해킹범을 검거합니다!");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::default()
        }

        let is_it_item_response = self.is_it_item_card(
            add_field_energy_with_field_unit_health_point_request_form
                .to_is_it_item_card_request(item_card_id)).await;

        if !is_it_item_response {
            println!("아이템 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::default()
        }

        let can_use_card_response = self.is_able_to_use(
            add_field_energy_with_field_unit_health_point_request_form
                .to_can_use_card_request(account_unique_id, item_card_id)).await;

        if !can_use_card_response {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::default()
        }

        let field_unit_index_string =
            add_field_energy_with_field_unit_health_point_request_form.get_field_unit_index();
        let field_unit_index =
            field_unit_index_string.parse::<i32>().unwrap();

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let current_health_point_of_field_unit =
            game_field_unit_service_guard.get_current_health_point_of_field_unit_by_index(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_get_field_unit_health_point_request(
                        account_unique_id,
                        field_unit_index)).await.get_current_unit_health_point();

        if current_health_point_of_field_unit == -1 {
            println!("필드에 존재하지 않는 유닛을 지정하여 보냈으므로 당신도 해킹범입니다!");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::default()
        }

        drop(game_field_unit_service_guard);

        let mut summarized_item_effect_response = self.get_summary_of_item_card(
            add_field_energy_with_field_unit_health_point_request_form
                .to_summary_item_effect_request(item_card_id)).await;

        let field_energy_amount_to_increase = summarized_item_effect_response
            .get_field_energy_addition_calculator()
                .calculation_of_field_energy_increment(current_health_point_of_field_unit);

        let game_field_energy_service_guard =
            self.game_field_energy_service.lock().await;

        let add_field_energy_with_amount_response =
            game_field_energy_service_guard.add_field_energy_with_amount(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_add_field_energy_with_amount_request(
                        account_unique_id,
                        field_energy_amount_to_increase)).await;

        if !add_field_energy_with_amount_response.is_success() {
            println!("필드에 에너지를 추가하는 과정에서 문제가 발생했습니다!");
            return AddFieldEnergyWithFieldUnitHealthPointResponseForm::default()
        }

        let updated_field_energy =
            game_field_energy_service_guard.get_current_field_energy(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_get_current_field_energy_request(
                        account_unique_id)).await.get_field_energy_count();

        drop(game_field_energy_service_guard);

        let mut battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id = battle_room_service_guard
            .find_opponent_by_account_unique_id(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let usage_hand_card = self.use_item_card(
            add_field_energy_with_field_unit_health_point_request_form
                .to_use_game_hand_item_card_request(account_unique_id, item_card_id)).await;

        self.place_used_card_to_tomb(
            add_field_energy_with_field_unit_health_point_request_form
                .to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_use_my_hand_card_data_response =
            ui_data_generator_service_guard.generate_use_my_hand_card_data(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_generate_use_my_hand_card_data_request(usage_hand_card)).await;

        let generate_my_field_energy_data_response =
            ui_data_generator_service_guard.generate_my_field_energy_data(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_generate_my_field_energy_data_request(updated_field_energy)).await;

        drop(ui_data_generator_service_guard);

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        let notice_response =
            notify_player_action_info_service_guard.notice_use_field_energy_increase_item_card(
                add_field_energy_with_field_unit_health_point_request_form
                    .to_notice_use_field_energy_increase_item_card_request(
                        opponent_unique_id,
                        generate_use_my_hand_card_data_response
                            .get_player_hand_use_map_for_notice().clone(),
                        generate_my_field_energy_data_response
                            .get_player_field_energy_map_for_notice().clone())).await;

        println!("notice_response: {:?}", notice_response);

        drop(notify_player_action_info_service_guard);

        AddFieldEnergyWithFieldUnitHealthPointResponseForm::from_response(
            generate_use_my_hand_card_data_response,
            generate_my_field_energy_data_response)
    }

    async fn request_to_use_catastrophic_damage_item(
        &self, catastrophic_damage_item_request_form: CatastrophicDamageItemRequestForm)
        -> CatastrophicDamageItemResponseForm {

        println!("GameCardItemControllerImpl: request_to_use_catastrophic_damage_item()");

        let account_unique_id = self.is_valid_session(
            catastrophic_damage_item_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("유효하지 않은 세션입니다.");
            return CatastrophicDamageItemResponseForm::default()
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                catastrophic_damage_item_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return CatastrophicDamageItemResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        // TODO: 프로토콜 검증은 추후 추가

        let item_card_id_string = catastrophic_damage_item_request_form.get_item_card_id();
        let item_card_id = item_card_id_string.parse::<i32>().unwrap();

        let is_it_item_response = self.is_it_item_card(
            catastrophic_damage_item_request_form
                .to_is_it_item_card_request(item_card_id)).await;

        if !is_it_item_response {
            println!("아이템 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return CatastrophicDamageItemResponseForm::default()
        }

        let can_use_card_response = self.is_able_to_use(
            catastrophic_damage_item_request_form
                .to_can_use_card_request(account_unique_id, item_card_id)).await;

        if !can_use_card_response {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return CatastrophicDamageItemResponseForm::default()
        }

        let mut summarized_item_effect_response =
            self.get_summary_of_item_card(
                catastrophic_damage_item_request_form
                    .to_summary_item_effect_request(item_card_id)).await;

        let damage_for_field_unit =
            summarized_item_effect_response.get_catastrophic_damage_for_field_unit();

        let damage_for_main_character =
            summarized_item_effect_response.get_catastrophic_damage_for_main_character();

        let mut battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id = battle_room_service_guard
            .find_opponent_by_account_unique_id(
                catastrophic_damage_item_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

       game_field_unit_service_guard.apply_catastrophic_damage_to_field_unit(
           catastrophic_damage_item_request_form
               .to_apply_catastrophic_damage_to_field_unit_request(
                   opponent_unique_id,
                   damage_for_field_unit)).await;

        let current_health_point_list_to_notice =
            game_field_unit_service_guard.get_current_health_point_of_all_field_unit(
                catastrophic_damage_item_request_form
                    .to_get_current_health_point_of_all_unit_request(
                        opponent_unique_id)).await.get_current_unit_health_point().clone();

        let dead_unit_index_list =
            game_field_unit_service_guard.judge_death_of_every_field_unit(
                catastrophic_damage_item_request_form
                    .to_judge_death_of_every_field_unit_request(
                        opponent_unique_id)).await.get_dead_unit_index_list();

        drop(game_field_unit_service_guard);

        let usage_hand_card = self.use_item_card(
            catastrophic_damage_item_request_form
                .to_use_game_hand_item_card_request(account_unique_id, item_card_id)).await;

        self.place_used_card_to_tomb(
            catastrophic_damage_item_request_form
                .to_place_to_tomb_request(account_unique_id, usage_hand_card)).await;

        let mut game_main_character_service_guard =
            self.game_main_character_service.lock().await;

        let mut current_opponent_health_point_to_notice =
            game_main_character_service_guard.get_current_main_character_health_point(
                catastrophic_damage_item_request_form
                    .to_get_current_health_point_of_main_character_request(
                        opponent_unique_id)).await.get_current_health_point();

        drop(game_main_character_service_guard);

        let mut opponent_survival_status_to_notice = StatusMainCharacterEnum::Survival;

        // 상대 본체에는 피해를 가하지 않는 경우가 있을 수 있으므로 다음과 같이 처리
        if damage_for_main_character != -1 {
            let mut game_main_character_service_guard =
                self.game_main_character_service.lock().await;

            game_main_character_service_guard.apply_damage_to_main_character(
                catastrophic_damage_item_request_form
                    .to_apply_damage_to_main_character(
                        opponent_unique_id,
                        damage_for_main_character)).await;

            let current_opponent_health_point =
                game_main_character_service_guard.get_current_main_character_health_point(
                    catastrophic_damage_item_request_form
                        .to_get_current_health_point_of_main_character_request(
                            opponent_unique_id)).await.get_current_health_point();

            let opponent_survival_status =
                game_main_character_service_guard.check_main_character_of_account_unique_id(
                    catastrophic_damage_item_request_form
                        .to_check_main_character_survival_request(
                            opponent_unique_id)).await.get_status_main_character().clone();

            current_opponent_health_point_to_notice = current_opponent_health_point;
            opponent_survival_status_to_notice = opponent_survival_status;

            drop(game_main_character_service_guard);
        }

        let will_be_lost_deck_card_count =
            summarized_item_effect_response.get_will_be_lost_deck_card_count();

        let mut lost_deck_card_list_to_notice = Vec::new();

        // 다른 광역기의 경우 로스트 존으로 이동시키는 기능이 없을 수 있으므로 다음과 같이 처리
        if will_be_lost_deck_card_count != -1 {
            let mut game_deck_service_guard =
                self.game_deck_service.lock().await;

            let draw_cards_from_deck_response =
                game_deck_service_guard.draw_cards_from_deck(
                    catastrophic_damage_item_request_form
                        .to_draw_cards_from_deck_request(
                            opponent_unique_id,
                            will_be_lost_deck_card_count)).await;

            drop(game_deck_service_guard);

            let mut will_be_lost_deck_card_list =
                draw_cards_from_deck_response.get_drawn_card_list().clone();

            let mut game_lost_zone_service_guard =
                self.game_lost_zone_service.lock().await;

            for will_be_lost_card in will_be_lost_deck_card_list.clone() {
                game_lost_zone_service_guard.place_card_to_lost_zone(
                    catastrophic_damage_item_request_form
                        .to_place_card_to_lost_zone_request(
                            opponent_unique_id,
                            will_be_lost_card)).await;
            }

            lost_deck_card_list_to_notice = will_be_lost_deck_card_list.clone();

            drop(game_lost_zone_service_guard);
        }

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        let notice_use_hand_card_response =
            notify_player_action_info_service_guard.notice_use_hand_card(
                catastrophic_damage_item_request_form
                    .to_notice_use_hand_card_request(
                        opponent_unique_id,
                        usage_hand_card)).await;

        let notice_apply_damage_to_opponent_main_character_response =
            notify_player_action_info_service_guard.notice_apply_damage_to_opponent_main_character(
                catastrophic_damage_item_request_form
                    .to_notice_apply_damage_to_opponent_main_character_request(
                        opponent_unique_id,
                        damage_for_main_character,
                        current_opponent_health_point_to_notice,
                        opponent_survival_status_to_notice)).await;

        let notice_lost_deck_card_of_opponent_response =
            notify_player_action_info_service_guard.notice_lost_deck_card_of_opponent(
                catastrophic_damage_item_request_form.to_notice_lost_deck_card_of_opponent_request(
                    opponent_unique_id,
                    lost_deck_card_list_to_notice)).await;

        drop(notify_player_action_info_service_guard);

        CatastrophicDamageItemResponseForm::default()
    }

    async fn request_to_use_applying_multiple_target_damage_by_field_unit_death_item(
        &self, multiple_target_damage_by_field_unit_death_item_request_form: MultipleTargetDamageByFieldUnitDeathItemRequestForm)
        -> MultipleTargetDamageByFieldUnitDeathItemResponseForm {

        println!("GameCardItemControllerImpl: request_to_use_catastrophic_damage_by_field_unit_death_item()");

        let account_unique_id = self.is_valid_session(
            multiple_target_damage_by_field_unit_death_item_request_form
                .to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("유효하지 않은 세션입니다.");
            return MultipleTargetDamageByFieldUnitDeathItemResponseForm::default()
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return MultipleTargetDamageByFieldUnitDeathItemResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        // TODO: 프로토콜 검증은 추후 추가

        // 사용할 변수들 사전 parsing
        let item_card_id_string =
            multiple_target_damage_by_field_unit_death_item_request_form.get_item_card_id();
        let item_card_id =
            item_card_id_string.to_string().parse::<i32>().unwrap();

        let my_field_unit_index_string =
            multiple_target_damage_by_field_unit_death_item_request_form.get_unit_index();
        let my_field_unit_index =
            my_field_unit_index_string.to_string().parse::<i32>().unwrap();

        let opponent_target_unit_index_list_string =
            multiple_target_damage_by_field_unit_death_item_request_form.get_opponent_target_unit_index_list().to_vec();
        let mut opponent_target_unit_index_list =
            VectorStringToVectorInteger::vector_string_to_vector_i32(opponent_target_unit_index_list_string);

        opponent_target_unit_index_list.sort();
        opponent_target_unit_index_list.reverse();

        let reversed_opponent_target_unit_index_list = opponent_target_unit_index_list.clone();

        // 사용할 아이템 카드 요약 정보
        let mut summarized_item_effect_response = self.get_summary_of_item_card(
            multiple_target_damage_by_field_unit_death_item_request_form
                .to_summary_item_effect_request(item_card_id)).await;

        let target_unit_count =
            summarized_item_effect_response.get_target_count_that_can_be_damaged();
        let can_be_sacrificed_unit_list =
            summarized_item_effect_response.get_unit_list_that_can_be_sacrificed();

        if target_unit_count != opponent_target_unit_index_list.len() as i32 {
            println!("{}마리의 상대 유닛을 정확히 지정해주세요!", target_unit_count);
            return MultipleTargetDamageByFieldUnitDeathItemResponseForm::default()
        }

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let fount_unit_card_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_find_target_unit_id_by_index_request(
                        account_unique_id,
                        my_field_unit_index)).await.get_found_opponent_unit_id();

        if !can_be_sacrificed_unit_list.contains(&fount_unit_card_id) {
            println!("제물로 사용할 수 없는 유닛입니다!");
            return MultipleTargetDamageByFieldUnitDeathItemResponseForm::default()
        }

        let health_point_of_sacrifice =
            game_field_unit_service_guard.get_current_health_point_of_field_unit_by_index(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_get_current_health_point_of_field_unit_by_index_request(
                        account_unique_id,
                        my_field_unit_index)).await.get_current_unit_health_point();

       game_field_unit_service_guard.apply_instant_death_to_target_unit_index(
           multiple_target_damage_by_field_unit_death_item_request_form
               .to_apply_instant_death_to_target_unit_index_request(
                   account_unique_id,
                   my_field_unit_index)).await;

        let mut battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id = battle_room_service_guard
            .find_opponent_by_account_unique_id(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let mut updated_health_point_list = Vec::new();
        let mut dead_unit_index_list = Vec::new();

        for opponent_unit_index in reversed_opponent_target_unit_index_list {
            game_field_unit_service_guard.apply_damage_to_target_unit_index(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_apply_damage_to_target_unit_request(
                        opponent_unique_id,
                        opponent_unit_index,
                        health_point_of_sacrifice)).await;

            let health_point_of_damaged_unit =
                game_field_unit_service_guard.get_current_health_point_of_field_unit_by_index(
                    multiple_target_damage_by_field_unit_death_item_request_form
                        .to_get_current_health_point_of_field_unit_by_index_request(
                            opponent_unique_id,
                            opponent_unit_index)).await.get_current_unit_health_point();

            updated_health_point_list.push(health_point_of_damaged_unit);

            let maybe_dead_unit_index =
                game_field_unit_service_guard.judge_death_of_unit(
                    multiple_target_damage_by_field_unit_death_item_request_form
                        .to_judge_death_of_unit_request(
                            opponent_unique_id,
                            opponent_unit_index)).await.get_dead_unit_index();

            if maybe_dead_unit_index != -1 {
                dead_unit_index_list.push(maybe_dead_unit_index);
            }
        }

        updated_health_point_list.reverse();
        dead_unit_index_list.reverse();

        drop(game_field_unit_service_guard);

        let usage_hand_item_card = self.use_item_card(
            multiple_target_damage_by_field_unit_death_item_request_form
                .to_use_game_hand_item_card_request(account_unique_id, item_card_id)).await;

        self.place_used_card_to_tomb(
            multiple_target_damage_by_field_unit_death_item_request_form
                .to_place_to_tomb_request(account_unique_id, usage_hand_item_card)).await;

        self.place_used_card_to_tomb(
            multiple_target_damage_by_field_unit_death_item_request_form
                .to_place_to_tomb_request(account_unique_id, fount_unit_card_id)).await;

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        let notice_use_hand_card_response =
            notify_player_action_info_service_guard.notice_use_hand_card(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_notice_use_hand_card_request(
                        opponent_unique_id,
                        usage_hand_item_card)).await;

        let notice_instant_death_of_specific_unit_response =
            notify_player_action_info_service_guard.notice_instant_death_of_specific_unit(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_notice_instant_death_of_specific_unit_request(
                        opponent_unique_id,
                        my_field_unit_index)).await;

        let notice_apply_damage_to_multiple_opponent_unit_response =
            notify_player_action_info_service_guard.notice_apply_damage_to_multiple_opponent_unit(
                multiple_target_damage_by_field_unit_death_item_request_form
                    .to_notice_apply_damage_to_multiple_opponent_unit_request(
                        opponent_unique_id,
                        health_point_of_sacrifice,
                        opponent_target_unit_index_list.clone(),
                        updated_health_point_list,
                        dead_unit_index_list)).await;

        drop(notify_player_action_info_service_guard);

        MultipleTargetDamageByFieldUnitDeathItemResponseForm::from_response(
            notice_use_hand_card_response,
            notice_instant_death_of_specific_unit_response,
            notice_apply_damage_to_multiple_opponent_unit_response)
    }

    async fn request_to_use_opponent_field_unit_energy_removal_item(
        &self, remove_opponent_field_unit_energy_item_request_form: RemoveOpponentFieldUnitEnergyItemRequestForm)
        -> RemoveOpponentFieldUnitEnergyItemResponseForm {

        println!("GameCardItemControllerImpl: request_to_use_opponent_field_unit_energy_removal_item()");

        let account_unique_id = self.is_valid_session(
            remove_opponent_field_unit_energy_item_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("유효하지 않은 세션입니다.");
            return RemoveOpponentFieldUnitEnergyItemResponseForm::default()
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                remove_opponent_field_unit_energy_item_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return RemoveOpponentFieldUnitEnergyItemResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        // TODO: 프로토콜 검증은 추후 추가

        let item_card_id_string =
            remove_opponent_field_unit_energy_item_request_form.get_item_card_id();
        let item_card_id =
            item_card_id_string.parse::<i32>().unwrap();

        let check_protocol_hacking_response = self.is_valid_protocol(
            remove_opponent_field_unit_energy_item_request_form
                .to_check_protocol_hacking_request(account_unique_id, item_card_id)).await;

        if !check_protocol_hacking_response {
            println!("해킹범을 검거합니다!");
            return RemoveOpponentFieldUnitEnergyItemResponseForm::default()
        }

        let is_it_item_response = self.is_it_item_card(
            remove_opponent_field_unit_energy_item_request_form
                .to_is_it_item_card_request(item_card_id)).await;

        if !is_it_item_response {
            println!("아이템 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return RemoveOpponentFieldUnitEnergyItemResponseForm::default()
        }

        let can_use_card_response = self.is_able_to_use(
            remove_opponent_field_unit_energy_item_request_form
                .to_can_use_card_request(account_unique_id, item_card_id)).await;

        if !can_use_card_response {
            println!("신화 카드는 4라운드 이후부터 사용 할 수 있습니다!");
            return RemoveOpponentFieldUnitEnergyItemResponseForm::default()
        }

        let opponent_field_unit_index_string =
            remove_opponent_field_unit_energy_item_request_form.get_opponent_target_unit_index();
        let opponent_field_unit_index =
            opponent_field_unit_index_string.parse::<i32>().unwrap();

        let mut summarized_item_effect_response = self.get_summary_of_item_card(
            remove_opponent_field_unit_energy_item_request_form
                .to_summary_item_effect_request(item_card_id)).await;

        let energy_quantity = summarized_item_effect_response.get_will_be_removed_energy_count();
        let alternative_damage = summarized_item_effect_response.get_alternatives_damage();

        let mut battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                remove_opponent_field_unit_energy_item_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let found_opponent_unit_id =
            game_field_unit_service_guard.find_target_unit_id_by_index(
                remove_opponent_field_unit_energy_item_request_form
                    .to_find_target_unit_id_by_index_request(
                        opponent_unique_id,
                        opponent_field_unit_index)).await.get_found_opponent_unit_id();

        let mut card_race_service_guard =
            self.card_race_service.lock().await;

        let found_opponent_unit_race =
            card_race_service_guard.get_card_race(&found_opponent_unit_id).await;

        drop(card_race_service_guard);

        let current_attached_energy_of_opponent_unit =
            game_field_unit_service_guard.get_current_attached_energy_of_field_unit_by_index(
                remove_opponent_field_unit_energy_item_request_form
                    .to_get_current_attached_energy_of_unit_by_index_request(
                        opponent_unique_id,
                        opponent_field_unit_index)).await.get_current_attached_energy_map().clone();

        if current_attached_energy_of_opponent_unit
            .get_energy_quantity(&RaceEnumValue::from(found_opponent_unit_race as i32)).is_none() {

            println!("붙은 에너지가 존재하지 않아 변환 데미지를 입힙니다.");

            game_field_unit_service_guard.apply_damage_to_target_unit_index(
                remove_opponent_field_unit_energy_item_request_form
                    .to_apply_damage_to_target_unit_request(
                        opponent_unique_id,
                        opponent_field_unit_index,
                        alternative_damage)).await;

            let updated_health_point_of_damaged_unit =
                game_field_unit_service_guard.get_current_health_point_of_field_unit_by_index(
                    remove_opponent_field_unit_energy_item_request_form
                        .to_get_current_health_point_of_field_unit_by_index_request(
                            opponent_unique_id,
                            opponent_field_unit_index)).await.get_current_unit_health_point();

            let maybe_dead_unit_index =
                game_field_unit_service_guard.judge_death_of_unit(
                    remove_opponent_field_unit_energy_item_request_form
                        .to_judge_death_of_unit_request(
                            opponent_unique_id,
                            opponent_field_unit_index)).await.get_dead_unit_index();

            drop(game_field_unit_service_guard);

            let used_hand_card_id = self.use_item_card(
                remove_opponent_field_unit_energy_item_request_form
                    .to_use_game_hand_item_card_request(account_unique_id, item_card_id)).await;

            self.place_used_card_to_tomb(
                remove_opponent_field_unit_energy_item_request_form
                    .to_place_to_tomb_request(account_unique_id, used_hand_card_id)).await;


            let mut notify_player_action_info_service_guard =
                self.notify_player_action_info_service.lock().await;

            let notice_use_hand_card_response =
                notify_player_action_info_service_guard.notice_use_hand_card(
                    remove_opponent_field_unit_energy_item_request_form
                        .to_notice_use_hand_card_request(
                            opponent_unique_id,
                            used_hand_card_id)).await;

            let notice_apply_damage_response =
                notify_player_action_info_service_guard.notice_apply_damage_to_specific_opponent_unit(
                    remove_opponent_field_unit_energy_item_request_form
                        .to_notice_apply_damage_to_specific_opponent_unit_request(
                            opponent_unique_id,
                            opponent_field_unit_index,
                            alternative_damage,
                            updated_health_point_of_damaged_unit,
                            maybe_dead_unit_index)).await;

            drop(notify_player_action_info_service_guard);

            return RemoveOpponentFieldUnitEnergyItemResponseForm::from_alternative_response(
                notice_use_hand_card_response, notice_apply_damage_response)
        }

        game_field_unit_service_guard.detach_multiple_energy_from_field_unit(
            remove_opponent_field_unit_energy_item_request_form
                .to_detach_energy_from_field_unit_request(
                    opponent_unique_id,
                    opponent_field_unit_index,
                    found_opponent_unit_race,
                    energy_quantity)).await;

        let updated_attached_energy_map =
            game_field_unit_service_guard.get_current_attached_energy_of_field_unit_by_index(
                remove_opponent_field_unit_energy_item_request_form
                    .to_get_current_attached_energy_of_unit_by_index_request(
                        opponent_unique_id,
                        opponent_field_unit_index)).await.get_current_attached_energy_map().clone();

        drop(game_field_unit_service_guard);

        let used_hand_card_id = self.use_item_card(
            remove_opponent_field_unit_energy_item_request_form
                .to_use_game_hand_item_card_request(account_unique_id, item_card_id)).await;

        self.place_used_card_to_tomb(
            remove_opponent_field_unit_energy_item_request_form
                .to_place_to_tomb_request(account_unique_id, used_hand_card_id)).await;

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        let notice_use_hand_card_response =
            notify_player_action_info_service_guard.notice_use_hand_card(
                remove_opponent_field_unit_energy_item_request_form
                    .to_notice_use_hand_card_request(opponent_unique_id, used_hand_card_id)).await;

        let notice_remove_energy_of_specific_opponent_unit_response =
            notify_player_action_info_service_guard.notice_remove_energy_of_specific_opponent_unit(
                remove_opponent_field_unit_energy_item_request_form
                    .to_notice_remove_energy_of_specific_opponent_unit_request(
                        opponent_unique_id,
                        opponent_field_unit_index,
                        updated_attached_energy_map)).await;

        drop(notify_player_action_info_service_guard);

        RemoveOpponentFieldUnitEnergyItemResponseForm::from_response(
            notice_use_hand_card_response, notice_remove_energy_of_specific_opponent_unit_response)
    }
}