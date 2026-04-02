import { expect, test } from 'bun:test'
import { mkdtemp, mkdir, rm, writeFile } from 'node:fs/promises'
import { join } from 'node:path'
import { tmpdir } from 'node:os'
import { clearSkillCaches, getSkillDirCommands } from '../../src/core/registry/skills.js'

test('project-local skill directories are discoverable', { timeout: 20000 }, async () => {
  const root = await mkdtemp(join(tmpdir(), 'aicd-skill-'))

  try {
    const skillDir = join(root, '.claude', 'skills', 'migration-plan')
    await mkdir(skillDir, { recursive: true })
    await writeFile(
      join(skillDir, 'SKILL.md'),
      `---\nname: migration-plan\ndescription: Generate migration planning help\n---\n\nUse this skill for migration planning.\n`,
    )

    clearSkillCaches()
    const commands = await getSkillDirCommands(root)
    expect(commands.some(command => command.name === 'migration-plan')).toBe(true)
  } finally {
    clearSkillCaches()
    await rm(root, { recursive: true, force: true })
  }
})
