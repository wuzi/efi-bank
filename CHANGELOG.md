# Changelog

## 0.3.0 - 2026-11-03

Breaking contract correction release. A 0.2.3 requirement does not select 0.3.0;
see the complete migration table and compiled examples in README.md.

- Return real action/list envelopes for carnet operations; require resend email.
  Keep cancellation-response names as deprecated forwarding aliases during 0.3.x.
- Correct immediate expiry/calendar/copy-and-paste fields, due-date PUT creation
  with caller txid and object adjustments, and interval-based one-page lists.
- Configure Pix webhooks by key with webhookUrl; preserve GET/list metadata and
  acknowledge bounded success without fabricated data, including empty 204.
- Send Pix via PUT v3 with caller-persisted identity and nested payer/recipient.
  Require matching response identity; keep sent lookup separate from received Pix.
- Return status-only Split link/unlink acknowledgements and preserve nested charge
  data as JSON; do not claim provider config GET availability or settlement.
- Isolate Pix/Billing OAuth caches and coalesce successful domain refreshes;
  conditionally invalidate rejected tokens and bound refresh queue wait to 20s.
- Encode string identifiers as single segments; reject empty/literal dot segments
  before authentication. Add non-exhaustive Error with InvalidPath and numeric
  ResponseTooLarge. External error matches require a wildcard.
- Bound API bodies to 8 MiB and OAuth bodies to 1 MiB by default, configurable
  with nonzero builder setters; preserve bounded marked diagnostics/status and
  raw Retry-After. Limits are engineering policy, not provider maxima.
- Retain GET-only single 401 replay, no mutation retry, finite HTTP deadlines,
  caller-owned transport policy, working Billing APIs and separate PDF safeguards.
- Raise the normal Tokio floor to 1.51.6 with sync/time only; test helpers remain
  dev-only. Refresh only the required locked Tokio companion family.

## 0.2.3 - 2026-10-16

- Preserve the complete 0.2.2 SDK source, including billet due-date updates, in a new patch release with tracked release metadata.

## 0.2.2

- Add `BillingBilletUpdateRequest` and `Client::billing_charge_update_billet` to update an unpaid boleto's due date through `PUT /v1/charge/{id}/billet`.
