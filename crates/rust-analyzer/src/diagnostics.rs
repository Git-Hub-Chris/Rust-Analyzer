//! Book keeping for keeping diagnostics easily in sync with the client.

#[cfg(feature = "flycheck")]
pub(crate) mod to_proto;

#[cfg(feature = "flycheck")]
use std::sync::Arc;

use ide::FileId;
use rustc_hash::{FxHashMap, FxHashSet};
use std::mem;

#[cfg(feature = "flycheck")]
pub(crate) type CheckFixes = Arc<FxHashMap<FileId, Vec<Fix>>>;

#[derive(Debug, Default, Clone)]
pub struct DiagnosticsMapConfig {
    pub remap_prefix: FxHashMap<String, String>,
    pub warnings_as_info: Vec<String>,
    pub warnings_as_hint: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct DiagnosticCollection {
    // FIXME: should be FxHashMap<FileId, Vec<ra_id::Diagnostic>>
    pub(crate) native: FxHashMap<FileId, Vec<lsp_types::Diagnostic>>,
    // FIXME: should be Vec<flycheck::Diagnostic>
    #[cfg(feature = "flycheck")]
    pub(crate) check: FxHashMap<FileId, Vec<lsp_types::Diagnostic>>,
    #[cfg(feature = "flycheck")]
    pub(crate) check_fixes: CheckFixes,
    changes: FxHashSet<FileId>,
}

#[derive(Debug, Clone)]
#[cfg(feature = "flycheck")]
pub(crate) struct Fix {
    // Fixes may be triggerable from multiple ranges.
    pub(crate) ranges: Vec<lsp_types::Range>,
    pub(crate) action: crate::lsp_ext::CodeAction,
}

impl DiagnosticCollection {
    #[cfg(feature = "flycheck")]
    pub(crate) fn clear_check(&mut self) {
        Arc::make_mut(&mut self.check_fixes).clear();
        self.changes.extend(self.check.drain().map(|(key, _value)| key))
    }

    #[cfg(feature = "flycheck")]
    pub(crate) fn add_check_diagnostic(
        &mut self,
        file_id: FileId,
        diagnostic: lsp_types::Diagnostic,
        fix: Option<Fix>,
    ) {
        let diagnostics = self.check.entry(file_id).or_default();
        for existing_diagnostic in diagnostics.iter() {
            if are_diagnostics_equal(existing_diagnostic, &diagnostic) {
                return;
            }
        }

        let check_fixes = Arc::make_mut(&mut self.check_fixes);
        check_fixes.entry(file_id).or_default().extend(fix);
        diagnostics.push(diagnostic);
        self.changes.insert(file_id);
    }

    pub(crate) fn set_native_diagnostics(
        &mut self,
        file_id: FileId,
        diagnostics: Vec<lsp_types::Diagnostic>,
    ) {
        if let Some(existing_diagnostics) = self.native.get(&file_id) {
            if existing_diagnostics.len() == diagnostics.len()
                && diagnostics
                    .iter()
                    .zip(existing_diagnostics)
                    .all(|(new, existing)| are_diagnostics_equal(new, existing))
            {
                return;
            }
        }

        self.native.insert(file_id, diagnostics);
        self.changes.insert(file_id);
    }

    pub(crate) fn diagnostics_for(
        &self,
        file_id: FileId,
    ) -> impl Iterator<Item = &lsp_types::Diagnostic> {
        let iter = self.native.get(&file_id).into_iter().flatten();
        #[cfg(feature = "flycheck")]
        let iter = iter.chain(self.check.get(&file_id).into_iter().flatten());
        iter
    }

    pub(crate) fn take_changes(&mut self) -> Option<FxHashSet<FileId>> {
        if self.changes.is_empty() {
            return None;
        }
        Some(mem::take(&mut self.changes))
    }
}

fn are_diagnostics_equal(left: &lsp_types::Diagnostic, right: &lsp_types::Diagnostic) -> bool {
    left.source == right.source
        && left.severity == right.severity
        && left.range == right.range
        && left.message == right.message
}
