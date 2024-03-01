use std::collections::HashMap;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_card_item::service::request::summary_item_card_effect_request::SummaryItemCardEffectRequest;
use crate::game_field_energy::service::request::add_field_energy_with_amount_request::AddFieldEnergyWithAmountRequest;
use crate::game_field_energy::service::request::get_current_field_energy_request::GetCurrentFieldEnergyRequest;
use crate::game_field_unit::service::request::get_current_health_point_of_field_unit_by_index_request::GetCurrentHealthPointOfFieldUnitByIndexRequest;
use crate::game_hand::service::request::use_game_hand_item_card_request::UseGameHandItemCardRequest;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_item_card_request::IsItItemCardRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action_info::service::request::notice_use_field_energy_increase_item_card_request::NoticeUseFieldEnergyIncreaseItemCardRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;
use crate::ui_data_generator::service::request::generate_my_field_energy_data_request::GenerateMyFieldEnergyDataRequest;
use crate::ui_data_generator::service::request::generate_use_my_hand_card_data_request::GenerateUseMyHandCardDataRequest;

#[derive(Debug)]
pub struct AddFieldEnergyWithFieldUnitHealthPointRequestForm {
    session_id: String,
    item_card_id: String,
    field_unit_index: String,
}

impl AddFieldEnergyWithFieldUnitHealthPointRequestForm {
    pub fn new(session_id: &str, item_card_id: &str, field_unit_index: &str) -> Self {
        AddFieldEnergyWithFieldUnitHealthPointRequestForm {
            session_id: session_id.to_string(),
            item_card_id: item_card_id.to_string(),
            field_unit_index: field_unit_index.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str { &self.session_id }

    pub fn get_item_card_id(&self) -> &str { &self.item_card_id }

    pub fn get_field_unit_index(&self) -> &str { &self.field_unit_index }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.get_session_id())
    }

    pub fn to_is_this_your_turn_request(&self,
                                        account_unique_id: i32) -> IsThisYourTurnRequest {
        IsThisYourTurnRequest::new(account_unique_id)
    }

    pub fn to_check_protocol_hacking_request(&self, account_unique_id: i32, item_card_number: i32) -> CheckProtocolHackingRequest {
        CheckProtocolHackingRequest::new(account_unique_id, item_card_number)
    }

    pub fn to_is_it_item_card_request(&self, item_card_id: i32) -> IsItItemCardRequest {
        IsItItemCardRequest::new(item_card_id)
    }

    pub fn to_can_use_card_request(&self, account_unique_id: i32, item_card_id: i32) -> CanUseCardRequest {
        CanUseCardRequest::new(account_unique_id, item_card_id)
    }

    pub fn to_summary_item_effect_request(&self, item_card_id: i32) -> SummaryItemCardEffectRequest {
        SummaryItemCardEffectRequest::new(item_card_id)
    }

    pub fn to_use_game_hand_item_card_request(&self, account_unique_id: i32, item_card_id: i32) -> UseGameHandItemCardRequest {
        UseGameHandItemCardRequest::new(account_unique_id, item_card_id)
    }

    pub fn to_place_to_tomb_request(&self, account_unique_id: i32, used_card_id: i32) -> PlaceToTombRequest {
        PlaceToTombRequest::new(account_unique_id, used_card_id)
    }

    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(account_unique_id)
    }

    pub fn to_get_field_unit_health_point_request(&self, account_unique_id: i32, unit_index: i32) -> GetCurrentHealthPointOfFieldUnitByIndexRequest {
        GetCurrentHealthPointOfFieldUnitByIndexRequest::new(account_unique_id, unit_index)
    }

    pub fn to_add_field_energy_with_amount_request(&self, account_unique_id: i32, amount_to_add: i32) -> AddFieldEnergyWithAmountRequest {
        AddFieldEnergyWithAmountRequest::new(account_unique_id, amount_to_add)
    }

    pub fn to_get_current_field_energy_request(
        &self,
        account_unique_id: i32) -> GetCurrentFieldEnergyRequest {

        GetCurrentFieldEnergyRequest::new(
            account_unique_id
        )
    }

    pub fn to_generate_use_my_hand_card_data_request(
        &self,
        used_hand_card_id: i32
    ) -> GenerateUseMyHandCardDataRequest {

        GenerateUseMyHandCardDataRequest::new(
            used_hand_card_id)
    }

    pub fn to_generate_my_field_energy_data_request(
        &self,
        my_remaining_field_energy: i32) -> GenerateMyFieldEnergyDataRequest {

        GenerateMyFieldEnergyDataRequest::new(
            my_remaining_field_energy)
    }

    pub fn to_notice_use_field_energy_increase_item_card_request(
        &self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>,) -> NoticeUseFieldEnergyIncreaseItemCardRequest {

        NoticeUseFieldEnergyIncreaseItemCardRequest::new(
            opponent_unique_id,
            player_hand_use_map_for_notice,
            player_field_energy_map_for_notice)
    }
}