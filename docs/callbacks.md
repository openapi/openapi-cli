# Callbacks In Openapi APIs

This note summarizes how callbacks are used across the Openapi specifications in [`oas/`](/home/francesco/Develop/Openapi/openapi-cli/oas/00-list.txt).

## Base Rule

In the usual callback model, the caller submits a request and also provides a callback URL owned by the caller.

That means:

- the client exposes an HTTP endpoint
- the client passes that endpoint to the API, typically in a field such as `callback`, `callback.url`, or `callback_data.url`
- the Openapi service calls that URL later when the asynchronous result is ready or when the workflow changes state

This is the default mental model to use for almost all callback-producing APIs in this repository.

The main exception is `oauthv2`: it is not the place where the callback URL is supplied for business workflows. Instead, it provides a platform-level view of callback executions through `/callbacks` and `/callbacks/{id}`.

## Main Ideas

Callbacks appear when an API cannot guarantee that the full business result is available in the same HTTP response.

The recurring patterns are:

1. Asynchronous document or report production.
   The initial `POST` opens a request, while the final outcome arrives later through a callback or through later polling on `{id}` endpoints.

2. Operational notifications.
   The callback is used to notify state transitions such as completion, delivery, activation, signature result, monitoring hits, or evidence availability.

3. Hybrid async model.
   Many APIs support both polling and callback. This is important because it means callbacks are not a cosmetic feature: they are the scalable integration path, while polling is usually the fallback.

4. Callback payload customization.
   Several APIs expose callback schemas with fields such as URL, method, headers, custom data, retry policy, field wrapping, or extra metadata. The common intent is to let the integrator correlate the async event with its own workflow.

5. Callback observability.
   `oauthv2` is a special case: it does not produce domain events itself, but exposes `/callbacks` and `/callbacks/{id}` to inspect callback history. This makes callbacks a platform-level concern, not just a per-API feature.

## APIs That Use Callbacks

### Highest importance

- `invoice`
  The callback is central because invoicing flows are stateful and long-running. Configuration changes, receipt handling, and invoice lifecycle events all benefit from push delivery instead of polling.

- `sdi`
  Very high importance. SDI is intrinsically event-driven: invoice submission, notifications, signature, legal storage, and simulation flows all produce delayed outcomes where callbacks are operationally critical.

- `company`
  High importance for monitoring and enriched asynchronous business intelligence. The `/monitor` flow is explicitly callback-oriented, and some high-value datasets also expose callback-aware async behavior.

- `risk`
  High importance because many reports, KYC checks, patrimonial products, and monitoring flows are not instantaneous. Callbacks reduce latency and orchestration complexity in credit/risk pipelines.

- `ufficiopostale`
  High importance because postal products, registered mail, telegrams, massive mailings, and court-related shipments evolve over time. Delivery and processing updates are a natural callback use case.

- `visurecamerali`
  High importance in practice because registry documents and business-register extractions are classic asynchronous document workflows. Callback support is valuable when requesting large volumes or integrating into back-office automation.

- `docuengine`
  High importance for the same reason: request creation and later document availability fit the callback model well, especially for document-heavy integrations.

- `visengine`
  High importance. The API is structured around requests, searches, and later document fulfillment, and its `CallbackData` model shows callbacks are part of the intended orchestration pattern.

### Medium importance

- `pec`
  Medium to high importance. PEC creation, activation, modification, conservation, transformation, and related workflows benefit from callback-driven updates because mailbox provisioning and lifecycle operations are not always immediate.

- `catasto`
  Medium to high importance. Cadastral, mortgage, map extract, and address-related requests often behave like asynchronous retrieval jobs, so callbacks help in document retrieval and workflow continuity.

- `esignature`
  Medium to high importance. Signature and certificate issuance workflows frequently need post-submission updates, especially when human actions or external providers are involved.

- `firmadigitale`
  Medium to high importance. Similar to `esignature`, with request lifecycle and delayed completion as the main reason callbacks matter.

- `bollettini`
  Medium importance. Payment-related flows can require post-request confirmation or downstream outcome notification.

- `gatewaysms`
  Medium importance. Messaging platforms commonly use callbacks for delivery reports or message-state updates, even if the visible path structure in the spec is narrower than in document APIs.

- `trust`
  Medium importance. Identity, mobile, email, IP, URL, and IDV checks can produce asynchronous outcomes, especially when deeper verification or expert review is involved.

### Platform and observability importance

- `oauthv2`
  Strategic importance rather than domain importance. It is the callback observability surface of the platform, via `/callbacks` and `/callbacks/{id}`. This is important because it lets integrators audit, troubleshoot, and inspect the callback layer itself.

## Practical Reading

If an API exposes:

- a `POST` that creates a request,
- a `GET /{id}` to inspect status,
- and callback-related schema fields such as `callback`, `callback_data`, or `callbacks`,

then that API should be treated as async-first.

In operational terms, the most callback-dependent families in this repository are:

- invoicing and SDI
- company monitoring and risk monitoring
- document retrieval and official certificates
- postal and delivery workflows
- signature and identity verification workflows

## Paths With Explicit Callback Presence

The OAS files with explicit callback presence in paths or operation definitions are:

- `bollettini`
- `catasto`
- `company`
- `docuengine`
- `esignature`
- `firmadigitale`
- `gatewaysms`
- `invoice`
- `oauthv2`
- `pec`
- `risk`
- `sdi`
- `trust`
- `ufficiopostale`
- `visengine`
- `visurecamerali`

## Conclusion

Across this repository, callbacks are not an edge feature. They are one of the main architectural patterns used by Openapi whenever the business result is delayed, multi-step, externally dependent, or operationally monitored.

The most important takeaway is this:

- polling tells you the current state
- callbacks let the platform tell you when the state matters

For high-volume or production-grade integrations, the callback path should generally be considered the primary integration model.
