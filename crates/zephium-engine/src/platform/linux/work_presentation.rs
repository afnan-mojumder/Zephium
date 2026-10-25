//! Work observation and human presentation for Linux (WebKitGTK).
#![allow(dead_code, unused_variables)]
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;

use gtk::prelude::*;
use wry::WebViewExtUnix as _;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PresentationState {
    Prepared,
    Acquiring,
    Ready,
    Retiring,
    Retired,
    Unavailable,
    Expired,
    Failed,
}

pub(crate) struct WorkObservationPresentation {
    view: webkit2gtk::WebView,
    deadline: Instant,
    state: PresentationState,
}

impl WorkObservationPresentation {
    #[cfg(feature = "agentic-browser-qa")]
    pub(crate) fn liveness_facts(&self) -> (bool, bool, bool, bool, bool) {
        (
            true,
            self.view.is_visible(),
            self.view.is_visible(),
            true,
            self.state == PresentationState::Ready,
        )
    }

    pub(crate) fn prepare(
        view: &wry::WebView,
        deadline: Instant,
        #[cfg(feature = "native-agentic-work-lifetime-diagnostic")] _failure: impl Fn(crate::WorkObservationPresentationFailure)
            + 'static,
    ) -> Result<Self, PresentationState> {
        if Instant::now() >= deadline {
            return Err(PresentationState::Expired);
        }
        let webview = view.webview();
        Ok(Self {
            view: webview,
            deadline,
            state: PresentationState::Prepared,
        })
    }

    pub(crate) fn present(&mut self) -> PresentationState {
        if Instant::now() >= self.deadline {
            self.state = PresentationState::Expired;
            return self.state;
        }
        self.state = PresentationState::Ready;
        self.state
    }

    pub(crate) fn poll(&mut self) -> PresentationState {
        if self.state == PresentationState::Ready && Instant::now() >= self.deadline {
            self.state = PresentationState::Expired;
        }
        self.state
    }

    pub(crate) fn renew(&mut self, deadline: Instant) -> bool {
        if self.state == PresentationState::Ready && deadline > Instant::now() {
            self.deadline = deadline;
            true
        } else {
            false
        }
    }

    pub(crate) fn human_current(&self) -> bool {
        self.state == PresentationState::Ready
    }

    pub(crate) fn human_fence(&self) -> Box<dyn Fn() -> bool> {
        let view = self.view.clone();
        Box::new(move || view.is_visible())
    }

    pub(crate) fn visible_for_audit(&self) -> bool {
        self.view.is_visible()
    }

    pub(crate) fn retain_frame_capture(&mut self, _guard: Option<Arc<AtomicBool>>) {}

    pub(crate) fn retire(&mut self) -> PresentationState {
        self.state = PresentationState::Retired;
        self.state
    }
}

pub(crate) struct WorkHumanPresentation {
    view: webkit2gtk::WebView,
    deadline: Instant,
    presented: bool,
    retired: bool,
}

impl WorkHumanPresentation {
    pub(crate) fn prepare(
        view: &wry::WebView,
        _region: zephium_agentic::WorkBrowserHumanRegion,
        deadline: Instant,
    ) -> Option<Self> {
        if deadline <= Instant::now() {
            return None;
        }
        Some(Self {
            view: view.webview(),
            deadline,
            presented: false,
            retired: false,
        })
    }

    pub(crate) fn present(&mut self) -> bool {
        if self.presented || self.retired || Instant::now() >= self.deadline {
            return false;
        }
        self.presented = true;
        self.view.show();
        true
    }

    pub(crate) fn qualify_geometry_invalidation(&self) -> bool {
        true
    }

    pub(crate) fn current(&self) -> bool {
        self.presented && !self.retired && Instant::now() < self.deadline
    }

    pub(crate) fn visible_for_audit(&self) -> bool {
        self.view.is_visible()
    }

    pub(crate) fn refit(&mut self) {}

    pub(crate) fn retire(&mut self) -> bool {
        if self.retired {
            return false;
        }
        self.retired = true;
        true
    }
}
