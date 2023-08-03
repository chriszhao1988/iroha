//! Notification events and their filter
#![allow(irrefutable_let_patterns)]

#[cfg(not(feature = "std"))]
use alloc::{format, string::String, vec::Vec};

use derive_more::Constructor;
use getset::Getters;
use iroha_data_model_derive::model;
use iroha_macro::FromVariant;
use iroha_schema::IntoSchema;
use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use strum::EnumDiscriminants;

pub use self::model::*;
use crate::trigger::TriggerId;

#[model]
pub mod model {
    use super::*;

    /// Notification event for events that arise during block application process like trigger execution for example
    #[derive(
        Debug, Clone, FromVariant, PartialEq, Eq, Decode, Encode, Deserialize, Serialize, IntoSchema,
    )]
    #[ffi_type]
    #[non_exhaustive]
    pub enum NotificationEvent {
        TriggerCompleted(TriggerCompletedEvent),
    }

    /// Event that notifies that a trigger was executed
    #[derive(
        Debug,
        Clone,
        Getters,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Constructor,
        Decode,
        Encode,
        Deserialize,
        Serialize,
        IntoSchema,
    )]
    #[ffi_type]
    #[getset(get = "pub")]
    pub struct TriggerCompletedEvent {
        trigger_id: TriggerId,
        outcome: TriggerCompletedOutcome,
    }

    /// Enum to represent outcome of trigger execution
    #[derive(
        Debug,
        Clone,
        FromVariant,
        EnumDiscriminants,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Decode,
        Encode,
        Deserialize,
        Serialize,
        IntoSchema,
    )]
    #[strum_discriminants(
        name(TriggerCompletedOutcomeType),
        derive(PartialOrd, Ord, Decode, Encode, Deserialize, Serialize, IntoSchema,),
        cfg_attr(
            any(feature = "ffi_import", feature = "ffi_export"),
            derive(iroha_ffi::FfiType)
        ),
        allow(missing_docs),
        repr(u8)
    )]
    #[ffi_type(opaque)]
    pub enum TriggerCompletedOutcome {
        Success,
        Failure(String),
    }

    /// Filter for [`NotificationEvent`]
    #[derive(
        Debug,
        Clone,
        FromVariant,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Decode,
        Encode,
        Deserialize,
        Serialize,
        IntoSchema,
    )]
    #[ffi_type]
    #[non_exhaustive]
    pub enum NotificationEventFilter {
        AcceptAll,
        TriggerCompleted(TriggerCompletedEventFilter),
    }

    /// Filter [`TriggerCompletedEvent`] by
    /// 1. if `triger_id` is some filter based on trigger id
    /// 2. if `outcome_type` is some filter based on execution outcome (success/failure)
    /// 3. if both fields are none accept every event of this type
    #[derive(
        Debug,
        Clone,
        Getters,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Decode,
        Encode,
        Deserialize,
        Serialize,
        IntoSchema,
    )]
    #[ffi_type]
    #[getset(get = "pub")]
    pub struct TriggerCompletedEventFilter {
        trigger_id: Option<TriggerId>,
        outcome_type: Option<TriggerCompletedOutcomeType>,
    }
}

#[cfg(feature = "transparent_api")]
impl super::Filter for NotificationEventFilter {
    type Event = NotificationEvent;

    /// Check if `self` accepts the `event`.
    #[inline]
    fn matches(&self, event: &Self::Event) -> bool {
        match (self, event) {
            (Self::AcceptAll, _) => true,
            (Self::TriggerCompleted(filter), NotificationEvent::TriggerCompleted(event)) => {
                filter.matches(event)
            }
        }
    }
}

#[cfg(feature = "transparent_api")]
impl super::Filter for TriggerCompletedEventFilter {
    type Event = TriggerCompletedEvent;

    /// Check if `self` accepts the `event`.
    #[inline]
    fn matches(&self, event: &Self::Event) -> bool {
        if matches!(self.trigger_id(), Some(trigger_id) if trigger_id != event.trigger_id()) {
            return false;
        }

        if matches!(
            (self.outcome_type(), event.outcome()),
            (
                Some(TriggerCompletedOutcomeType::Success),
                TriggerCompletedOutcome::Failure(_)
            ) | (
                Some(TriggerCompletedOutcomeType::Failure),
                TriggerCompletedOutcome::Success
            )
        ) {
            return false;
        }

        true
    }
}

/// Exports common structs and enums from this module.
pub mod prelude {
    pub use super::{
        NotificationEvent, NotificationEventFilter, TriggerCompletedEvent,
        TriggerCompletedEventFilter, TriggerCompletedOutcome, TriggerCompletedOutcomeType,
    };
}