//! An RTP observer inspects the media received by a set of selected producers.
//!
//! mediasoup implements the following RTP observers:
//! * [`AudioLevelObserver`](crate::audio_level_observer::AudioLevelObserver)

use crate::data_structures::AppData;
use crate::producer::{Producer, ProducerId};
use crate::uuid_based_wrapper_type;
use crate::worker::RequestError;
use async_trait::async_trait;
use event_listener_primitives::HandlerId;

uuid_based_wrapper_type!(
    /// RTP observer identifier.
    RtpObserverId
);

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct RtpObserverAddProducerOptions {
    /// The id of the Producer to be added.
    pub producer_id: ProducerId,
}

impl RtpObserverAddProducerOptions {
    /// * `producer_id` - The id of the Producer to be added.
    pub fn new(producer_id: ProducerId) -> Self {
        Self { producer_id }
    }
}

/// An RTP observer inspects the media received by a set of selected producers.
///
/// mediasoup implements the following RTP observers:
/// * [`AudioLevelObserver`](crate::audio_level_observer::AudioLevelObserver)
#[async_trait(?Send)]
pub trait RtpObserver {
    /// RtpObserver id.
    fn id(&self) -> RtpObserverId;

    /// Whether the RtpObserver is paused.
    fn paused(&self) -> bool;

    /// Custom application data.
    fn app_data(&self) -> &AppData;

    /// Whether the RTP observer is closed.
    fn closed(&self) -> bool;

    /// Pauses the RTP observer. No RTP is inspected until resume() is called.
    async fn pause(&self) -> Result<(), RequestError>;

    /// Resumes the RTP observer. RTP is inspected again.
    async fn resume(&self) -> Result<(), RequestError>;

    /// Provides the RTP observer with a new producer to monitor.
    async fn add_producer(
        &self,
        rtp_observer_add_producer_options: RtpObserverAddProducerOptions,
    ) -> Result<(), RequestError>;

    /// Removes the given producer from the RTP observer.
    async fn remove_producer(&self, producer_id: ProducerId) -> Result<(), RequestError>;

    /// Callback is called when the RTP observer is paused.
    fn on_pause<F: Fn() + Send + Sync + 'static>(&self, callback: F) -> HandlerId;

    /// Callback is called when the RTP observer is resumed.
    fn on_resume<F: Fn() + Send + Sync + 'static>(&self, callback: F) -> HandlerId;

    /// Callback is called when a new producer is added into the RTP observer.
    fn on_add_producer<F: Fn(&Producer) + Send + Sync + 'static>(&self, callback: F) -> HandlerId;

    /// Callback is called when a producer is removed from the RTP observer.
    fn on_remove_producer<F: Fn(&Producer) + Send + Sync + 'static>(
        &self,
        callback: F,
    ) -> HandlerId;

    /// Callback is called when the router this RTP observer belongs to is closed for whatever reason. The RTP
    /// observer itself is also closed.
    fn on_router_close<F: FnOnce() + Send + 'static>(&self, callback: F) -> HandlerId;

    /// Callback is called when the RTP observer is closed for whatever reason.
    fn on_close<F: FnOnce() + Send + 'static>(&self, callback: F) -> HandlerId;
}
