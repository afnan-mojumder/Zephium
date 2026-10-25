//! Agent context adapters for Linux.
//!
//! Autonomous agent execution ("Work") is currently dormant on Linux.
//! This module provides the type contracts required by zephium-engine
//! so that crates and workspace features compile cleanly across platforms.
#![allow(dead_code, unused_variables)]

use std::time::Instant;
use zephium_agentic::{
    ContextNavigationTarget, SemanticActionAttemptId, SemanticActionNativeFailure,
    SemanticActionNativeRequest, SemanticActionNativeSettlement, SemanticInvocationId,
    SemanticRuntimeInvocation, SemanticRuntimePortFailure, SemanticSnapshot,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AgentHistoryBackTicket;

pub(crate) struct AgentSemanticRuntimeController;

impl AgentSemanticRuntimeController {
    pub(crate) fn cancel(&self) {}
    pub(crate) fn draining_action(&self, _attempt: SemanticActionAttemptId) -> bool {
        false
    }
    pub(crate) fn revoked_settling_action(&self, _attempt: SemanticActionAttemptId) -> bool {
        false
    }
    pub(crate) fn timeout_action(&self, _attempt: SemanticActionAttemptId) -> bool {
        false
    }
    pub(crate) fn settling_action(&self, _attempt: SemanticActionAttemptId) -> bool {
        false
    }
    pub(crate) fn timeout(&self, _invocation: SemanticInvocationId) -> bool {
        false
    }
    pub(crate) fn witness_identity(&self) -> Option<(usize, usize, u16)> {
        None
    }
}

pub(crate) struct AgentOwnedView {
    view: wry::WebView,
}

impl AgentOwnedView {
    pub(crate) fn view(&self) -> &wry::WebView {
        &self.view
    }

    pub(crate) fn work_navigation(
        &self,
    ) -> Option<&crate::platform::work_document_navigation::WorkDocumentNavigation> {
        None
    }

    pub(crate) fn semantic(&self) -> Option<&AgentSemanticRuntimeController> {
        None
    }

    pub(crate) fn semantic_pending_for_audit(&self) -> Option<bool> {
        None
    }

    pub(crate) fn semantic_runtime_parked(&self) -> bool {
        true
    }

    pub(crate) fn semantic_runtime_ready_for_history(&self) -> bool {
        false
    }

    pub(crate) fn park_semantic_runtime(
        &mut self,
        _completion: impl FnOnce(bool) + 'static,
    ) -> Result<(), ()> {
        Err(())
    }

    pub(crate) fn dispatch_semantic(
        &self,
        _invocation: SemanticRuntimeInvocation,
        _completion: impl FnOnce(Result<SemanticSnapshot, SemanticRuntimePortFailure>) + 'static,
    ) -> Result<(), SemanticRuntimePortFailure> {
        Err(SemanticRuntimePortFailure::Retired)
    }

    pub(crate) fn dispatch_retained_semantic_action(
        &self,
        request: SemanticActionNativeRequest,
        _admitted_at: Instant,
        _authority: Box<dyn Fn() -> bool>,
        completion: impl FnOnce(SemanticActionNativeSettlement) + 'static,
    ) {
        let completed_at = request.requested_at();
        completion(request.fail(SemanticActionNativeFailure::Shutdown, completed_at));
    }

    pub(crate) fn record_media_ports(
        &self,
    ) -> Option<(
        crate::platform::record_media::MediaDispatch,
        crate::platform::record_media::FrameCapture,
        crate::platform::record_media::PrivacyDispatch,
    )> {
        None
    }

    pub(crate) fn cancel_work_storage(&self) -> bool {
        true
    }

    pub(crate) fn work_native_activity_drained(&self) -> bool {
        true
    }

    pub(crate) fn retire_semantic_runtime(&self) -> bool {
        true
    }

    pub(crate) fn close(&mut self) -> Result<(), ()> {
        Ok(())
    }

    pub(crate) fn set_work_leased(&self, _leased: bool) -> bool {
        true
    }

    pub(crate) fn diagnose_work_cookie_disk(&self) {}

    pub(crate) fn begin_history_lease(&self, _target: &ContextNavigationTarget) -> Result<(), ()> {
        Err(())
    }

    pub(crate) fn enroll_work_history_with_completion(
        &mut self,
        _target: ContextNavigationTarget,
        _healthy: bool,
        _completion: impl FnOnce(bool) + 'static,
    ) {
    }

    pub(crate) fn progress_work_storage(&mut self) {}

    pub(crate) fn work_storage_ready(&self) -> bool {
        false
    }

    pub(crate) fn admit_work_store_page(&mut self) -> bool {
        false
    }

    pub(crate) fn prepare_semantic_document_load(&mut self) -> Result<(), ()> {
        Err(())
    }

    pub(crate) fn prepare_history_back(&mut self) -> Result<AgentHistoryBackTicket, ()> {
        Err(())
    }

    pub(crate) fn settle_history_back(
        &mut self,
        _ticket: AgentHistoryBackTicket,
    ) -> Result<(), ()> {
        Ok(())
    }

    pub(crate) fn history_destination(
        &self,
        _ticket: AgentHistoryBackTicket,
    ) -> Option<&ContextNavigationTarget> {
        None
    }

    pub(crate) fn refuse_history_back(&mut self, _ticket: AgentHistoryBackTicket) -> bool {
        true
    }

    pub(crate) fn reactivate_history_destination(
        &mut self,
        _ticket: AgentHistoryBackTicket,
    ) -> Result<ContextNavigationTarget, ()> {
        Err(())
    }

    pub(crate) fn dispatch_history_back(&self, _ticket: AgentHistoryBackTicket) -> bool {
        false
    }

    pub(crate) fn dispatch_history_back_guarded(
        &self,
        _ticket: AgentHistoryBackTicket,
        _guard: Box<dyn FnOnce() -> bool + 'static>,
    ) -> bool {
        false
    }

    pub(crate) fn activate_work_human(&self) -> bool {
        false
    }

    pub(crate) fn note_work_sign_in(&self) {}

    pub(crate) fn finish_work_human_activity(&self, _completion: impl FnOnce(bool) + 'static) {}

    pub(crate) fn sample_work_human_wall(
        &self,
        _completion: impl FnOnce(String) + 'static,
    ) -> Result<(), ()> {
        Err(())
    }

    pub(crate) fn work_session_return_pending(&self) -> bool {
        false
    }

    pub(crate) fn enroll_handed_on_document(
        &mut self,
        _target: ContextNavigationTarget,
    ) -> Result<(), ()> {
        Err(())
    }
}
