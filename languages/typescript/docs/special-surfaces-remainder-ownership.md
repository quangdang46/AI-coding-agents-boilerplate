# TypeScript special-surfaces remainder ownership

The remaining archived TypeScript special-surface files from bead `aicd-3ix.4.16.1` are not preserved as one-for-one shipped runtime modules in the current AICD TypeScript proving slice.

The repository’s current taxonomy is sufficient to classify them conservatively: `memdir` rows align to the optional `session-memory` and `team-memory` feature-pack families, `buddy` rows align to deferred companion-style affordances, `native-ts` rows align to deferred native-performance modules, and the remaining `voice`, `vim`, and `moreright` files stay archive-only until a narrower owning feature-pack or shipped runtime boundary exists.

## Ownership and disposition

- `references/typescript/src/voice/voiceModeEnabled.ts` — archive-only voice helper retained for source fidelity; voice behavior remains outside the shipped TypeScript proving slice and no narrower shipped owner exists today.
- `references/typescript/src/vim/transitions.ts` — archive-only vim support helper retained for source fidelity; the proving slice ships the summarized `vim` command surface, not the archived vim runtime package.
- `references/typescript/src/vim/motions.ts` — archive-only vim support helper retained for source fidelity; the archived vim motion engine is not a shipped TypeScript runtime boundary.
- `references/typescript/src/vim/operators.ts` — archive-only vim support helper retained for source fidelity; the proving slice does not ship the archived vim operator package.
- `references/typescript/src/vim/textObjects.ts` — archive-only vim support helper retained for source fidelity; text-object behavior remains outside the shipped TypeScript proving slice.
- `references/typescript/src/vim/types.ts` — archive-only vim type surface retained for source fidelity; no direct shipped TypeScript owner exists for the archived vim runtime types.
- `references/typescript/src/memdir/teamMemPaths.ts` — future feature-pack ownership aligned to the implemented `team-memory` family; archived team-memory path helpers are not shipped one-for-one as runtime modules.
- `references/typescript/src/memdir/paths.ts` — future feature-pack ownership aligned to `session-memory` and `team-memory`; archived memdir path helpers remain optional memory-family infrastructure.
- `references/typescript/src/memdir/teamMemPrompts.ts` — future feature-pack ownership aligned to the implemented `team-memory` family; archived team-memory prompting remains optional feature-pack behavior.
- `references/typescript/src/memdir/memoryAge.ts` — future feature-pack ownership aligned to the implemented `session-memory` family; memory-age helpers are not part of the shipped core runtime.
- `references/typescript/src/memdir/memoryScan.ts` — future feature-pack ownership aligned to the implemented `session-memory` family; archived memory scanning remains optional memory-pack infrastructure.
- `references/typescript/src/memdir/memoryTypes.ts` — future feature-pack ownership aligned to `session-memory` and `team-memory`; archived memory types are not preserved one-for-one as shipped core runtime types.
- `references/typescript/src/memdir/memdir.ts` — future feature-pack ownership aligned to `session-memory` and `team-memory`; the archived memdir root module remains optional memory-family infrastructure.
- `references/typescript/src/memdir/findRelevantMemories.ts` — future feature-pack ownership aligned to the implemented `session-memory` family; relevant-memory lookup remains optional feature-pack behavior.
- `references/typescript/src/buddy/CompanionSprite.tsx` — archive-only companion-affordance UI retained for source fidelity; buddy/companion behavior remains deferred and is not part of the shipped TypeScript proving slice.
- `references/typescript/src/buddy/prompt.ts` — archive-only companion prompt helper retained for source fidelity; no shipped TypeScript owner exists for archived buddy prompting.
- `references/typescript/src/buddy/types.ts` — archive-only companion type surface retained for source fidelity; buddy types remain outside the shipped proving slice.
- `references/typescript/src/buddy/useBuddyNotification.tsx` — archive-only companion notification helper retained for source fidelity; buddy notification behavior remains deferred.
- `references/typescript/src/buddy/sprites.ts` — archive-only companion sprite asset helper retained for source fidelity; the proving slice does not ship the archived buddy asset package.
- `references/typescript/src/buddy/companion.ts` — archive-only companion runtime helper retained for source fidelity; companion affordances remain deferred rather than shipped.
- `references/typescript/src/native-ts/file-index/index.ts` — archive-only native-performance helper retained for source fidelity; native file-index modules remain deferred and are not required for current runtime correctness.
- `references/typescript/src/native-ts/yoga-layout/enums.ts` — archive-only native-performance helper retained for source fidelity; native layout modules remain deferred rather than shipped.
- `references/typescript/src/native-ts/yoga-layout/index.ts` — archive-only native-performance helper retained for source fidelity; the proving slice does not ship archived native layout bindings.
- `references/typescript/src/native-ts/color-diff/index.ts` — archive-only native-performance helper retained for source fidelity; native color-diff behavior remains deferred rather than a shipped runtime dependency.
- `references/typescript/src/moreright/useMoreRight.tsx` — archive-only product-affordance helper retained for source fidelity; no current feature-pack or shipped runtime owner exists for the archived moreright surface.

## Shipped-language-pack rule

This remainder is complete when each archived special-surface row has a clear future feature-pack owner or an explicit archive-only rationale. Deferred companion, native-performance, and low-frequency product affordances must not be mistaken for already-shipped TypeScript runtime ownership.
