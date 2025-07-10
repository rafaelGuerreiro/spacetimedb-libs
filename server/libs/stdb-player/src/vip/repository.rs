use crate::vip::{StdbOwnVipV1, VipStatusV1, stdb_own_vip_v1};
use spacetimedb::ReducerContext;
use stdb_common::prelude::{ResultExt, ServiceResult, Uuid, validate_str, validate_uuid};

pub trait VipRepository {
    fn find_vip(&self, sender_id: &Uuid, receiver_id: &Uuid) -> Option<StdbOwnVipV1>;

    fn insert_vip(&self, sender_id: Uuid, receiver_id: Uuid, tag: String) -> ServiceResult<StdbOwnVipV1>;
}

impl VipRepository for ReducerContext {
    fn find_vip(&self, sender_id: &Uuid, receiver_id: &Uuid) -> Option<StdbOwnVipV1> {
        self.db
            .stdb_own_vip_v1()
            .player_ids_index()
            .filter((sender_id, receiver_id))
            .next()
    }

    fn insert_vip(&self, sender_id: Uuid, receiver_id: Uuid, tag: String) -> ServiceResult<StdbOwnVipV1> {
        validate_uuid("sender_id", &sender_id)?;
        validate_uuid("receiver_id", &receiver_id)?;
        validate_str("tag", &tag, 0, 32)?;

        let sender = self.find_vip(&sender_id, &receiver_id);
        let receiver = self.find_vip(&receiver_id, &sender_id);

        match (&sender, &receiver) {
            (None, None) | (Some(_), None) => {
                // Neither players tried to add each other. Let's create Invite requests.
                // Or the receiver doesn't have an invite yet
                upsert_vip(
                    self,
                    &receiver,
                    &receiver_id,
                    &sender_id,
                    "".to_string(),
                    VipStatusV1::InviteReceived,
                )?;
                upsert_vip(self, &sender, &sender_id, &receiver_id, tag, VipStatusV1::InviteSent)
            },
            (None, Some(r)) | (Some(_), Some(r)) => {
                // Receiver had an invite and now the sender is adding the receiver
                upsert_vip(self, &receiver, &receiver_id, &sender_id, r.tag.clone(), VipStatusV1::Friends)?;
                upsert_vip(self, &sender, &sender_id, &receiver_id, tag, VipStatusV1::Friends)
            },
        }
    }
}

fn upsert_vip(
    ctx: &ReducerContext,
    sender: &Option<StdbOwnVipV1>,
    sender_id: &Uuid,
    receiver_id: &Uuid,
    tag: String,
    status: VipStatusV1,
) -> ServiceResult<StdbOwnVipV1> {
    let new_row = match sender {
        Some(sender) => {
            let mut sender = sender.clone();
            sender.tag = tag;
            sender.status = status;
            sender
        },
        None => StdbOwnVipV1 {
            vip_id: 0,
            sender_id: sender_id.clone(),
            receiver_id: receiver_id.clone(),
            tag,
            status,
            created_at: ctx.timestamp,
        },
    };

    ctx.db
        .stdb_own_vip_v1()
        .vip_id()
        .try_insert_or_update(new_row)
        .map_conflict_ctx("failed to insert vip")
}
