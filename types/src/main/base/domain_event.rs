use std::fmt::Debug;

use derive_new::new;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(new, Debug, Clone, PartialEq, SmartDefault)]
pub struct DomainEvent {
    #[new(value = "EventId::new()")]
    #[default(Default::default())]
    pub id: EventId,
    #[new(value = "OffsetDateTime::now_utc()")]
    #[default(_code = "OffsetDateTime::now_utc()")]
    created: OffsetDateTime,
}

#[derive(new, PartialEq, Eq, Debug, Clone, Default, Serialize, Deserialize, Hash)]
pub struct EventId {
    #[new(value = "Uuid::new_v4()")]
    pub(crate) value: Uuid,
}

#[enum_dispatch]
pub trait DomainEventTrait: Debug {}

// todo возможно понадобится
// serialize_trait_object!(DomainEventTrait<T>);

// impl DomainEventTrait for DomainEvent {}

// impl dyn DomainEventTrait + 'static {
//     pub fn downcast_ref<T: DomainEventTrait + 'static>(&self) -> Option<&T> {
//         unsafe { Some(&*(self as *const dyn DomainEventTrait as *const T)) }
//     }
// }
