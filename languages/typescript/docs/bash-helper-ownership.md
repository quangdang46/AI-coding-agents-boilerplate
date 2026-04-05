# TypeScript bash-helper ownership

The archived helper files in bead `aicd-3ix.4.14.1.5` are not preserved as a one-for-one shipped runtime subtree in the current AICD TypeScript proving slice.

The shipped TypeScript language pack currently exposes bash execution only through the proving-slice runtime/config path in `languages/typescript/runtime/registry/coreTools.ts` and `languages/typescript/runtime/utils/toolExecution.ts`; it does not ship the archived `src/utils/bash/*` parser, quoting, spec, and shell-state helper stack as a dedicated `languages/typescript/runtime/bash/` boundary. These archived helpers therefore remain archive-only unless future shell-analysis or advanced command-runtime work explicitly adopts them.

## Ownership and disposition

- `references/typescript/src/utils/bash/shellCompletion.ts` — archive-only shell-completion helper retained for source fidelity; the shipped proving slice does not expose a dedicated bash completion runtime boundary.
- `references/typescript/src/utils/bash/prefix.ts` — archive-only bash prefix helper retained for source fidelity; the shipped proving slice does not ship this helper as a standalone runtime module.
- `references/typescript/src/utils/bash/bashParser.ts` — archive-only bash parser retained for source fidelity; advanced bash parsing remains outside the shipped proving slice.
- `references/typescript/src/utils/bash/treeSitterAnalysis.ts` — archive-only tree-sitter bash analysis helper retained for source fidelity; tree-sitter-based shell analysis is not part of the shipped proving slice.
- `references/typescript/src/utils/bash/shellQuote.ts` — archive-only shell quoting helper retained for source fidelity; the proving slice does not expose the archived quoting helper stack as a runtime boundary.
- `references/typescript/src/utils/bash/registry.ts` — archive-only bash helper registry retained for source fidelity; helper-registry coordination remains outside the shipped proving slice.
- `references/typescript/src/utils/bash/ast.ts` — archive-only bash AST helper retained for source fidelity; AST-level bash modeling is not part of the shipped proving slice.
- `references/typescript/src/utils/bash/specs/alias.ts` — archive-only bash spec helper retained for source fidelity; command-spec coverage remains outside the shipped proving slice.
- `references/typescript/src/utils/bash/specs/nohup.ts` — archive-only bash spec helper retained for source fidelity; command-spec coverage remains outside the shipped proving slice.
- `references/typescript/src/utils/bash/specs/time.ts` — archive-only bash spec helper retained for source fidelity; command-spec coverage remains outside the shipped proving slice.
- `references/typescript/src/utils/bash/specs/timeout.ts` — archive-only bash spec helper retained for source fidelity; command-spec coverage remains outside the shipped proving slice.
- `references/typescript/src/utils/bash/specs/index.ts` — archive-only bash spec index retained for source fidelity; the shipped proving slice does not ship this spec package.
- `references/typescript/src/utils/bash/specs/srun.ts` — archive-only bash spec helper retained for source fidelity; advanced shell command-spec handling is outside the shipped proving slice.
- `references/typescript/src/utils/bash/specs/pyright.ts` — archive-only bash spec helper retained for source fidelity; advanced shell command-spec handling is outside the shipped proving slice.
- `references/typescript/src/utils/bash/specs/sleep.ts` — archive-only bash spec helper retained for source fidelity; advanced shell command-spec handling is outside the shipped proving slice.
- `references/typescript/src/utils/bash/shellQuoting.ts` — archive-only shell-quoting helper retained for source fidelity; the shipped proving slice does not expose this quoting module as a runtime boundary.
- `references/typescript/src/utils/bash/heredoc.ts` — archive-only heredoc helper retained for source fidelity; heredoc parsing/rendering remains outside the shipped proving slice.
- `references/typescript/src/utils/bash/ParsedCommand.ts` — archive-only parsed-command contract retained for source fidelity; the shipped proving slice does not ship the archived parsed-command model as a standalone runtime module.
- `references/typescript/src/utils/bash/parser.ts` — archive-only bash parser facade retained for source fidelity; advanced parser orchestration remains outside the shipped proving slice.
- `references/typescript/src/utils/bash/commands.ts` — archive-only bash command helper retained for source fidelity; the shipped proving slice does not expose this command catalog as a runtime boundary.
- `references/typescript/src/utils/bash/bashPipeCommand.ts` — archive-only bash pipe-command helper retained for source fidelity; pipe-command parsing and modeling are outside the shipped proving slice.
- `references/typescript/src/utils/bash/ShellSnapshot.ts` — archive-only shell-snapshot model retained for source fidelity; shell state snapshotting is not part of the shipped proving slice.
- `references/typescript/src/utils/bash/shellPrefix.ts` — archive-only shell-prefix helper retained for source fidelity; shell-prefix modeling remains outside the shipped proving slice.

## Shipped-language-pack rule

This subset is complete when each archived bash-helper row has an explicit archive-only or shipped-owner rationale. These snapshot-era bash helper files must not be mistaken for already-shipped TypeScript runtime modules.
