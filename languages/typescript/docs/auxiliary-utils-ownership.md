# TypeScript auxiliary utils ownership

The archived helper files in bead `aicd-3ix.4.14.1.8` are not preserved as a one-for-one shipped runtime subtree in the current AICD TypeScript proving slice.

This bead spans several small utility subtrees that sit outside the proving-slice runtime boundary. Some rows in its original coverage manifest were already dispositioned by earlier closed ownership beads:

- `references/typescript/src/utils/memory/versions.ts`
- `references/typescript/src/utils/memory/types.ts`
  - already covered by `languages/typescript/docs/utils-runtime-helper-ownership.md`
- `references/typescript/src/utils/filePersistence/filePersistence.ts`
- `references/typescript/src/utils/filePersistence/outputsScanner.ts`
- `references/typescript/src/utils/filePersistence/types.ts`
  - already covered by `languages/typescript/docs/utils-standalone-ownership-b.md`

The remaining rows below are owned by this bead and receive their final disposition here.

## Ownership and disposition

- `references/typescript/src/utils/powershell/dangerousCmdlets.ts` — archive-only PowerShell safety helper retained for source fidelity; the shipped proving slice does not expose a dedicated PowerShell runtime boundary.
- `references/typescript/src/utils/powershell/staticPrefix.ts` — archive-only PowerShell prefix helper retained for source fidelity; PowerShell-specific command handling remains outside the shipped proving slice.
- `references/typescript/src/utils/powershell/parser.ts` — archive-only PowerShell parser retained for source fidelity; parser-level PowerShell handling is not part of the shipped proving slice.
- `references/typescript/src/utils/sandbox/sandbox-adapter.ts` — archive-only sandbox adapter retained for source fidelity; the shipped proving slice exposes sandbox policy summaries but not this archived adapter module.
- `references/typescript/src/utils/sandbox/sandbox-ui-utils.ts` — archive-only sandbox UI helper retained for source fidelity; sandbox UI/runtime helpers remain outside the shipped proving slice.
- `references/typescript/src/utils/dxt/zip.ts` — archive-only DXT zip helper retained for source fidelity; DXT packaging/runtime flows are outside the shipped proving slice.
- `references/typescript/src/utils/dxt/helpers.ts` — archive-only DXT helper retained for source fidelity; DXT-specific runtime support is not part of the shipped proving slice.
- `references/typescript/src/utils/git/gitFilesystem.ts` — archive-only git filesystem helper retained for source fidelity; advanced git workflow support remains outside the shipped proving slice.
- `references/typescript/src/utils/git/gitignore.ts` — archive-only gitignore helper retained for source fidelity; git workflow helpers are not part of the shipped proving slice.
- `references/typescript/src/utils/git/gitConfigParser.ts` — archive-only git config parser retained for source fidelity; advanced git config handling remains outside the shipped proving slice.
- `references/typescript/src/utils/mcp/elicitationValidation.ts` — archive-only MCP validation helper retained for source fidelity; advanced MCP helper logic remains outside the shipped proving slice.
- `references/typescript/src/utils/mcp/dateTimeParser.ts` — archive-only MCP date-time helper retained for source fidelity; advanced MCP helper logic remains outside the shipped proving slice.
- `references/typescript/src/utils/background/remote/preconditions.ts` — archive-only remote background preconditions helper retained for source fidelity; remote background orchestration remains outside the shipped proving slice.
- `references/typescript/src/utils/background/remote/remoteSession.ts` — archive-only remote background session helper retained for source fidelity; remote background session handling is not part of the shipped proving slice.
- `references/typescript/src/utils/teleport/environmentSelection.ts` — archive-only teleport environment-selection helper retained for source fidelity; teleport/remote bundle flows remain outside the shipped proving slice.
- `references/typescript/src/utils/teleport/gitBundle.ts` — archive-only teleport git-bundle helper retained for source fidelity; teleport bundle generation is not part of the shipped proving slice.
- `references/typescript/src/utils/teleport/environments.ts` — archive-only teleport environment helper retained for source fidelity; teleport environment modeling remains outside the shipped proving slice.
- `references/typescript/src/utils/teleport/api.ts` — archive-only teleport API helper retained for source fidelity; teleport API integration is not part of the shipped proving slice.
- `references/typescript/src/utils/ultraplan/ccrSession.ts` — archive-only ultraplan session helper retained for source fidelity; ultraplan workflows remain outside the shipped proving slice.
- `references/typescript/src/utils/ultraplan/prompt.txt` — archive-only ultraplan prompt artifact retained for source fidelity; ultraplan prompt assets are not part of the shipped proving slice.
- `references/typescript/src/utils/ultraplan/keyword.ts` — archive-only ultraplan keyword helper retained for source fidelity; ultraplan keyword parsing remains outside the shipped proving slice.

## Previously dispositioned rows still in this bead's manifest

- `references/typescript/src/utils/memory/versions.ts` — already dispositioned as archive-only in `languages/typescript/docs/utils-runtime-helper-ownership.md`.
- `references/typescript/src/utils/memory/types.ts` — already dispositioned as archive-only in `languages/typescript/docs/utils-runtime-helper-ownership.md`.
- `references/typescript/src/utils/filePersistence/filePersistence.ts` — already dispositioned as archive-only in `languages/typescript/docs/utils-standalone-ownership-b.md`.
- `references/typescript/src/utils/filePersistence/outputsScanner.ts` — already dispositioned as archive-only in `languages/typescript/docs/utils-standalone-ownership-b.md`.
- `references/typescript/src/utils/filePersistence/types.ts` — already dispositioned as archive-only in `languages/typescript/docs/utils-standalone-ownership-b.md`.

## Shipped-language-pack rule

This subset is complete when each row claimed by the bead has an explicit archive-only or shipped-owner rationale, whether documented here or by the earlier canonical ownership docs referenced above. These snapshot-era auxiliary utility files must not be mistaken for already-shipped TypeScript runtime modules.
