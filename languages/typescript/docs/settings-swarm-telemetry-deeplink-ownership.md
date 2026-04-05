# TypeScript settings, swarm, telemetry, and deep-link helper ownership

The archived operational helper files in bead `aicd-3ix.4.14.1.4` are not preserved as a one-for-one shipped utility subtree in the current AICD TypeScript proving slice.

This cluster splits into several ownership outcomes:

- settings helpers map conceptually to the shipped TypeScript runtime/config boundary and branded settings files
- swarm helpers align to optional multi-agent or team-oriented workflow directions rather than the shipped base runtime
- telemetry helpers remain archive-only or future optional observability work outside the proving slice
- deep-link helpers remain optional integration behavior outside the shipped base runtime

## Ownership and disposition

- `references/typescript/src/utils/settings/validation.ts` — owned conceptually by the shipped TypeScript runtime/config boundary and branded settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/pluginOnlyPolicy.ts` — owned conceptually by the shipped runtime/config boundary and feature-enablement settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/mdm/rawRead.ts` — owned conceptually by the shipped runtime/config boundary and branded settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/mdm/settings.ts` — owned conceptually by the shipped runtime/config boundary and branded settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/mdm/constants.ts` — owned conceptually by the shipped runtime/config boundary and branded settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/toolValidationConfig.ts` — owned conceptually by the shipped runtime/config boundary and tool policy configuration contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/changeDetector.ts` — owned conceptually by the shipped runtime/config boundary and branded settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/types.ts` — owned conceptually by the shipped runtime/config boundary and branded settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/permissionValidation.ts` — owned conceptually by the shipped runtime/config boundary and tool policy configuration contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/applySettingsChange.ts` — owned conceptually by the shipped runtime/config boundary and branded settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/internalWrites.ts` — owned conceptually by the shipped runtime/config boundary and branded settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/validationTips.ts` — owned conceptually by the shipped runtime/config boundary and settings guidance contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/settingsCache.ts` — owned conceptually by the shipped runtime/config boundary and branded settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/allErrors.ts` — owned conceptually by the shipped runtime/config boundary and settings validation contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/managedPath.ts` — owned conceptually by the shipped runtime/config boundary and branded settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/settings.ts` — owned conceptually by the shipped runtime/config boundary and branded settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/constants.ts` — owned conceptually by the shipped runtime/config boundary and branded settings contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/schemaOutput.ts` — owned conceptually by the shipped runtime/config boundary and settings/schema contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/settings/validateEditTool.ts` — owned conceptually by the shipped runtime/config boundary and tool policy configuration contract, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/swarm/teammateLayoutManager.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; swarm layout helpers are not part of the shipped base runtime.
- `references/typescript/src/utils/swarm/teammateModel.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; swarm teammate modeling is not part of the shipped base runtime.
- `references/typescript/src/utils/swarm/It2SetupPrompt.tsx` — future feature-pack ownership aligned to multi-agent/team workflow directions; swarm setup UI is not part of the shipped base runtime.
- `references/typescript/src/utils/swarm/teammatePromptAddendum.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; swarm prompting remains outside the shipped base runtime.
- `references/typescript/src/utils/swarm/teammateInit.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; teammate bootstrap remains outside the shipped base runtime.
- `references/typescript/src/utils/swarm/reconnection.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; swarm reconnection behavior remains outside the shipped base runtime.
- `references/typescript/src/utils/swarm/spawnInProcess.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; in-process swarm spawning is not part of the shipped base runtime.
- `references/typescript/src/utils/swarm/leaderPermissionBridge.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; leader/permission swarm bridges remain outside the shipped base runtime.
- `references/typescript/src/utils/swarm/inProcessRunner.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; in-process swarm execution is not part of the shipped base runtime.
- `references/typescript/src/utils/swarm/permissionSync.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; swarm permission sync remains outside the shipped base runtime.
- `references/typescript/src/utils/swarm/spawnUtils.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; swarm spawn helpers remain outside the shipped base runtime.
- `references/typescript/src/utils/swarm/teamHelpers.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; team-oriented swarm helpers remain outside the shipped base runtime.
- `references/typescript/src/utils/swarm/constants.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; swarm constants remain outside the shipped base runtime.
- `references/typescript/src/utils/swarm/backends/detection.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; swarm backend detection is not part of the shipped base runtime.
- `references/typescript/src/utils/swarm/backends/PaneBackendExecutor.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; pane backend execution is not part of the shipped base runtime.
- `references/typescript/src/utils/swarm/backends/ITermBackend.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; iTerm backend support is not part of the shipped base runtime.
- `references/typescript/src/utils/swarm/backends/types.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; swarm backend types remain outside the shipped base runtime.
- `references/typescript/src/utils/swarm/backends/registry.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; swarm backend registry remains outside the shipped base runtime.
- `references/typescript/src/utils/swarm/backends/it2Setup.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; iTerm swarm setup remains outside the shipped base runtime.
- `references/typescript/src/utils/swarm/backends/teammateModeSnapshot.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; teammate-mode snapshots remain outside the shipped base runtime.
- `references/typescript/src/utils/swarm/backends/InProcessBackend.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; in-process swarm backend support is not part of the shipped base runtime.
- `references/typescript/src/utils/swarm/backends/TmuxBackend.ts` — future feature-pack ownership aligned to multi-agent/team workflow directions; tmux swarm backend support is not part of the shipped base runtime.
- `references/typescript/src/utils/deepLink/registerProtocol.ts` — archive-only or future optional integration ownership; deep-link protocol registration is not part of the shipped base proving slice.
- `references/typescript/src/utils/deepLink/banner.ts` — archive-only or future optional integration ownership; deep-link UI/banner behavior is not part of the shipped base proving slice.
- `references/typescript/src/utils/deepLink/protocolHandler.ts` — archive-only or future optional integration ownership; deep-link protocol handling is not part of the shipped base proving slice.
- `references/typescript/src/utils/deepLink/terminalLauncher.ts` — archive-only or future optional integration ownership; deep-link terminal launching is not part of the shipped base proving slice.
- `references/typescript/src/utils/deepLink/parseDeepLink.ts` — archive-only or future optional integration ownership; deep-link parsing is not part of the shipped base proving slice.
- `references/typescript/src/utils/deepLink/terminalPreference.ts` — archive-only or future optional integration ownership; deep-link terminal preference handling is not part of the shipped base proving slice.
- `references/typescript/src/utils/telemetry/logger.ts` — archive-only or future optional observability ownership; telemetry logging is not part of the shipped base proving slice.
- `references/typescript/src/utils/telemetry/instrumentation.ts` — archive-only or future optional observability ownership; telemetry instrumentation is not part of the shipped base proving slice.
- `references/typescript/src/utils/telemetry/events.ts` — archive-only or future optional observability ownership; telemetry event definitions are not part of the shipped base proving slice.
- `references/typescript/src/utils/telemetry/perfettoTracing.ts` — archive-only or future optional observability ownership; tracing integrations are not part of the shipped base proving slice.
- `references/typescript/src/utils/telemetry/skillLoadedEvent.ts` — archive-only or future optional observability ownership; telemetry for skill loading is not part of the shipped base proving slice.
- `references/typescript/src/utils/telemetry/bigqueryExporter.ts` — archive-only or future optional observability ownership; telemetry export plumbing is not part of the shipped base proving slice.
- `references/typescript/src/utils/telemetry/betaSessionTracing.ts` — archive-only or future optional observability ownership; beta tracing support is not part of the shipped base proving slice.
- `references/typescript/src/utils/telemetry/pluginTelemetry.ts` — archive-only or future optional observability ownership; plugin telemetry is not part of the shipped base proving slice.
- `references/typescript/src/utils/telemetry/sessionTracing.ts` — archive-only or future optional observability ownership; session tracing is not part of the shipped base proving slice.

## Shipped-language-pack rule

This subset is complete when each archived settings/swarm/telemetry/deep-link row has an explicit shipped-owner or archive-only rationale. These operational helpers must not be mistaken for already-shipped one-for-one runtime modules just because the proving slice preserves settings/config responsibilities and optional workflow taxonomy.
