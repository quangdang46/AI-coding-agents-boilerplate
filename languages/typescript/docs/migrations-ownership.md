# TypeScript migrations ownership

The archived TypeScript migration files in bead `aicd-3ix.4.4.1.2` are historical upgrade helpers from the source snapshot, not base-runtime modules that the shipped AICD TypeScript language pack must carry forward as a live migrations package.

## Ownership and disposition

- `references/typescript/src/migrations/migrateBypassPermissionsAcceptedToSettings.ts` — owned conceptually by the shipped TypeScript runtime config/settings boundary under `languages/typescript/runtime/utils/config.ts`, without preserving the archived one-shot migration helper directly.
- `references/typescript/src/migrations/migrateOpusToOpus1m.ts` — archive-only provider/model migration artifact retained for source fidelity; not part of the shipped base TypeScript proving slice.
- `references/typescript/src/migrations/migrateEnableAllProjectMcpServersToSettings.ts` — owned conceptually by optional MCP/runtime settings work rather than the shipped base TypeScript runtime.
- `references/typescript/src/migrations/resetAutoModeOptInForDefaultOffer.ts` — archive-only product/offer migration artifact retained for source fidelity; not part of the shipped base TypeScript proving slice.
- `references/typescript/src/migrations/migrateReplBridgeEnabledToRemoteControlAtStartup.ts` — owned conceptually by optional bridge/remote-control feature work rather than the shipped base TypeScript runtime.
- `references/typescript/src/migrations/migrateSonnet45ToSonnet46.ts` — archive-only provider/model migration artifact retained for source fidelity; not part of the shipped base TypeScript proving slice.
- `references/typescript/src/migrations/migrateLegacyOpusToCurrent.ts` — archive-only provider/model migration artifact retained for source fidelity; not part of the shipped base TypeScript proving slice.
- `references/typescript/src/migrations/resetProToOpusDefault.ts` — archive-only provider/model migration artifact retained for source fidelity; not part of the shipped base TypeScript proving slice.
- `references/typescript/src/migrations/migrateFennecToOpus.ts` — archive-only provider/model migration artifact retained for source fidelity; not part of the shipped base TypeScript proving slice.
- `references/typescript/src/migrations/migrateSonnet1mToSonnet45.ts` — archive-only provider/model migration artifact retained for source fidelity; not part of the shipped base TypeScript proving slice.
- `references/typescript/src/migrations/migrateAutoUpdatesToSettings.ts` — archive-only updater/install migration artifact retained for source fidelity; the shipped TypeScript template does not carry the archived updater migration path.

## Shipped-language-pack rule

This cluster is complete when each archived migration row has either a clear shipped config/runtime owner or an explicit archive-only rationale. Historical snapshot migrations must not be mistaken for required base-runtime extraction work.
