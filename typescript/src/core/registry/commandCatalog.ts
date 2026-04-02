import type { Command } from '../../types/command.js'

export function buildCommandCatalog(args: {
  primary: Command[]
  optional: Array<Command | null | false | undefined>
  internalOnly: Command[]
  includeInternalOnly: boolean
}): Command[] {
  const { primary, optional, internalOnly, includeInternalOnly } = args

  return [
    ...primary,
    ...optional.filter(Boolean),
    ...(includeInternalOnly ? internalOnly : []),
  ] as Command[]
}

export function buildCommandNameSet(commands: Command[]): Set<string> {
  return new Set(commands.flatMap(command => [command.name, ...(command.aliases ?? [])]))
}
