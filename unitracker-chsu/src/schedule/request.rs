/// # Request logic:
///
/// ## Link:
///
/// https://www.chsu.ru/raspisanie/cache
///
/// ## base64-encoded parameters:
///
/// WyJzdHVkZW50IiwiMTczOTU4MjQyNDUwNTc3NTcxMSIsbnVsbCwiMDYuMDIuMjAyNCIsIjA2LjAyLjIwMjQiXQ => ["student","1739582424505775711",null,"06.02.2024","06.02.2024"]
/// WyJ0dXRvciIsIjE3Mzk1ODI0MjQ1MDU3NzU3MTEiLCIxNDcyMzE0MDI1NjAwNjIwNDA1IiwiMDguMDIuMjAyNCIsIjA4LjAyLjIwMjQiXQ => ["tutor","1739582424505775711","1472314025600620405","08.02.2024","08.02.2024"]
///
/// ## Argument format
/// type ("student"/"tutor"),
/// group ID (nullable, stays at whichever was last requested for tutor request),
/// tutor ID (nullable, stays at whichever was last requested for student request),
/// start date (dd.MM.yyyy),
/// end date (dd.MM.yyyy)
///
/// _=.json to specify the format
///
/// ### Example:
///
/// https://www.chsu.ru/raspisanie/cache/WyJzdHVkZW50IiwiMTczOTU4MjQyNDUwNTc3NTcxMSIsbnVsbCwiMDYuMDIuMjAyNCIsIjA2LjAyLjIwMjQiXQ_=.json?1707052923221