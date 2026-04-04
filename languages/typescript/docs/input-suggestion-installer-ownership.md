# TypeScript input, suggestion, and native-installer ownership

The archived helper files in bead `aicd-3ix.4.14.1.7` are not preserved as a one-for-one shipped utility subtree in the AICD TypeScript language pack. Their final disposition is to map to the shipped runtime/template interaction surface where applicable, or to remain archive-only when they describe product-specific installer or suggestion behavior outside the proving slice.

## Ownership and disposition

- `references/typescript/src/utils/processUserInput/processUserInput.ts` — owned conceptually by shipped TypeScript template input handling and runtime prompt/context assembly rather than preserved as a direct utility port.
- `references/typescript/src/utils/processUserInput/processTextPrompt.ts` — owned conceptually by shipped TypeScript template prompt construction and runtime context handling.
- `references/typescript/src/utils/processUserInput/processBashCommand.tsx` — archive-only prompt-processing helper retained for source fidelity; the shipped proving slice does not expose this interactive helper as a standalone runtime module.
- `references/typescript/src/utils/processUserInput/processSlashCommand.tsx` — archive-only prompt-processing helper retained for source fidelity; command-lane behavior is owned separately and not by this utility bead.
- `references/typescript/src/utils/suggestions/slackChannelSuggestions.ts` — archive-only product suggestion helper retained for source fidelity; not part of the shipped proving slice.
- `references/typescript/src/utils/suggestions/skillUsageTracking.ts` — archive-only suggestion helper retained for source fidelity; not part of the shipped proving slice.
- `references/typescript/src/utils/suggestions/commandSuggestions.ts` — archive-only interactive suggestion helper retained for source fidelity; not part of the shipped proving slice.
- `references/typescript/src/utils/suggestions/shellHistoryCompletion.ts` — archive-only suggestion helper retained for source fidelity; not part of the shipped proving slice.
- `references/typescript/src/utils/suggestions/directoryCompletion.ts` — archive-only suggestion helper retained for source fidelity; not part of the shipped proving slice.
- `references/typescript/src/utils/nativeInstaller/pidLock.ts` — archive-only native-installer helper retained for source fidelity; not part of the shipped TypeScript template contract.
- `references/typescript/src/utils/nativeInstaller/installer.ts` — archive-only native-installer helper retained for source fidelity; not part of the shipped TypeScript template contract.
- `references/typescript/src/utils/nativeInstaller/packageManagers.ts` — archive-only native-installer helper retained for source fidelity; not part of the shipped TypeScript template contract.
- `references/typescript/src/utils/nativeInstaller/download.ts` — archive-only native-installer helper retained for source fidelity; not part of the shipped TypeScript template contract.
- `references/typescript/src/utils/nativeInstaller/index.ts` — archive-only native-installer helper retained for source fidelity; not part of the shipped TypeScript template contract.

## Shipped-language-pack rule

This cluster is complete when each archived input/suggestion/native-installer row has either a clear shipped interaction/runtime owner or an explicit archive-only rationale. Product-specific suggestion flows and native installer internals must not be mistaken for required base-runtime extraction work.
