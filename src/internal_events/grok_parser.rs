use super::{ErrorTypes, InternalEvent};
use metrics::counter;

#[derive(Debug)]
pub(crate) struct GrokParserEventProcessed;

impl InternalEvent for GrokParserEventProcessed {
    fn emit_logs(&self) {
        trace!(message = "Processed one event.");
    }

    fn emit_metrics(&self) {
        counter!("events_processed_total", 1);
    }
}

#[derive(Debug)]
pub(crate) struct GrokParserFailedMatch<'a> {
    pub value: &'a str,
}

impl InternalEvent for GrokParserFailedMatch<'_> {
    fn emit_logs(&self) {
        warn!(
            message = "Grok pattern failed to match.",
            field = &super::truncate_string_at(self.value, 60)[..],
            rate_limit_secs = 30
        );
    }

    fn emit_metrics(&self) {
        counter!("processing_errors_total", 1,
            "error_type" => ErrorTypes::MatchFailed.to_str(),
        );
    }
}

#[derive(Debug)]
pub(crate) struct GrokParserMissingField<'a> {
    pub field: &'a str,
}

impl InternalEvent for GrokParserMissingField<'_> {
    fn emit_logs(&self) {
        warn!(message = "Field does not exist.", field = %self.field);
    }

    fn emit_metrics(&self) {
        counter!("processing_errors_total", 1,
            "error_type" => ErrorTypes::FieldMissing.to_str(),
        );
    }
}

#[derive(Debug)]
pub(crate) struct GrokParserConversionFailed<'a> {
    pub name: &'a str,
    pub error: crate::types::Error,
}

impl<'a> InternalEvent for GrokParserConversionFailed<'a> {
    fn emit_logs(&self) {
        warn!(
            message = "Could not convert types.",
            name = %self.name,
            error = ?self.error,
            rate_limit_secs = 30
        );
    }

    fn emit_metrics(&self) {
        counter!("processing_errors_total", 1,
            "error_type" => ErrorTypes::TypeConversionFailed.to_str(),
        );
    }
}
