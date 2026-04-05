# TypeScript API, auth, and bootstrap-facing services ownership

The archived service files in bead `aicd-3ix.4.11.1.1` are not preserved as a one-for-one shipped services subtree in the current AICD TypeScript proving slice.

The current TypeScript pack keeps long-term ownership for base runtime behavior under `languages/typescript/runtime/`, especially `runtime/config/`, `runtime/providers/`, `runtime/context/`, and `runtime/state/`. OAuth-specific service files align to the optional `oauth-onboarding` feature pack rather than the shipped core runtime.

## Ownership and disposition

- `references/typescript/src/services/api/logging.ts` — owned conceptually by the shipped runtime core and session/export summary surfaces rather than preserved one-for-one as a service module.
- `references/typescript/src/services/api/filesApi.ts` — owned conceptually by shipped runtime file and tool execution boundaries rather than preserved one-for-one as a service module.
- `references/typescript/src/services/api/dumpPrompts.ts` — owned conceptually by shipped prompt/context and export summary boundaries rather than preserved one-for-one as a service module.
- `references/typescript/src/services/api/client.ts` — owned conceptually by shipped runtime/provider/config boundaries rather than preserved one-for-one as a service module.
- `references/typescript/src/services/api/ultrareviewQuota.ts` — archive-only or future optional workflow ownership; quota logic is not part of the shipped base proving slice.
- `references/typescript/src/services/api/claude.ts` — owned conceptually by the shipped runtime provider boundary under `languages/typescript/runtime/providers/` rather than preserved one-for-one as a service module.
- `references/typescript/src/services/api/sessionIngress.ts` — owned conceptually by shipped runtime context/state/session boundaries rather than preserved one-for-one as a service module.
- `references/typescript/src/services/api/promptCacheBreakDetection.ts` — owned conceptually by shipped prompt/context digest and runtime config summary boundaries rather than preserved one-for-one as a service module.
- `references/typescript/src/services/api/emptyUsage.ts` — owned conceptually by shipped runtime state/session usage accounting rather than preserved one-for-one as a service module.
- `references/typescript/src/services/api/errorUtils.ts` — archive-only helper retained for source fidelity; no narrower shipped service owner exists today.
- `references/typescript/src/services/api/adminRequests.ts` — archive-only or future optional admin-surface ownership; admin request handling is not part of the shipped base proving slice.
- `references/typescript/src/services/api/withRetry.ts` — owned conceptually by shipped runtime provider/config boundaries rather than preserved one-for-one as a service helper.
- `references/typescript/src/services/api/codex-fetch-adapter.ts` — archive-only or future optional provider-integration ownership; no narrower shipped service owner exists today.
- `references/typescript/src/services/api/errors.ts` — archive-only helper retained for source fidelity; no narrower shipped service owner exists today.
- `references/typescript/src/services/api/grove.ts` — archive-only or future optional provider/integration ownership; no shipped base runtime owner exists today.
- `references/typescript/src/services/api/bootstrap.ts` — owned conceptually by shipped runtime entrypoint/config boundaries under `languages/typescript/runtime/entrypoints/` and `languages/typescript/runtime/config/`.
- `references/typescript/src/services/api/firstTokenDate.ts` — owned conceptually by shipped runtime state/session usage accounting rather than preserved one-for-one as a service module.
- `references/typescript/src/services/api/referral.ts` — archive-only or future optional product-surface ownership; referral behavior is not part of the shipped base proving slice.
- `references/typescript/src/services/api/metricsOptOut.ts` — owned conceptually by shipped runtime config/settings boundaries rather than preserved one-for-one as a service module.
- `references/typescript/src/services/api/overageCreditGrant.ts` — archive-only or future optional billing/product ownership; no shipped base runtime owner exists today.
- `references/typescript/src/services/api/usage.ts` — owned conceptually by shipped runtime state/session usage accounting and command summaries rather than preserved one-for-one as a service module.
- `references/typescript/src/services/oauth/codex-client.ts` — future feature-pack ownership aligned to `oauth-onboarding`; OAuth-specific service wiring is not shipped in the base proving slice.
- `references/typescript/src/services/oauth/auth-code-listener.ts` — future feature-pack ownership aligned to `oauth-onboarding`; OAuth-specific service wiring is not shipped in the base proving slice.
- `references/typescript/src/services/oauth/client.ts` — future feature-pack ownership aligned to `oauth-onboarding`; OAuth-specific service wiring is not shipped in the base proving slice.
- `references/typescript/src/services/oauth/crypto.ts` — future feature-pack ownership aligned to `oauth-onboarding`; OAuth-specific service wiring is not shipped in the base proving slice.
- `references/typescript/src/services/oauth/index.ts` — future feature-pack ownership aligned to `oauth-onboarding`; OAuth-specific service wiring is not shipped in the base proving slice.
- `references/typescript/src/services/oauth/getOauthProfile.ts` — future feature-pack ownership aligned to `oauth-onboarding`; OAuth-specific service wiring is not shipped in the base proving slice.
- `references/typescript/src/services/claudeAiLimits.ts` — archive-only or future optional provider-limit ownership; no narrower shipped base runtime owner exists today.

## Shipped-language-pack rule

This subset is complete when each archived API/auth/bootstrap service row has an explicit shipped-owner or archive-only rationale. These service files must not be mistaken for already-shipped one-for-one runtime modules just because the proving slice preserves their underlying responsibilities.
