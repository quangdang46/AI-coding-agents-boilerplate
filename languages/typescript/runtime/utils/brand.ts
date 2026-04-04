import { readdirSync, statSync } from 'node:fs'
import { join } from 'node:path'

export function inferBrandRoot(root: string): string {
  const candidates = readdirSync(root)
    .map((name) => join(root, name))
    .filter((path) => {
      const name = path.split('/').at(-1) ?? ''
      if (
        (!name.startsWith('.') && name !== '__BRAND_ROOT__') ||
        name.endsWith('-plugin')
      ) {
        return false
      }
      if (!statSync(path).isDirectory()) {
        return false
      }
      return ['settings.json', 'settings.local.json', 'instructions.md', 'sessions'].some((child) =>
        exists(join(root, name, child)),
      )
    })
    .sort()

  return candidates[0] ?? join(root, '.claude')
}

export function instructionCandidates(root: string, brandRoot: string): string[] {
  const out: string[] = []
  for (const name of readdirSync(root).sort()) {
    if (!name.endsWith('.md') || ['README.md', 'AGENTS.md', 'CLAUDE.md'].includes(name)) {
      continue
    }
    out.push(join(root, name))
  }
  for (const name of ['AGENTS.md', 'CLAUDE.md']) {
    const path = join(root, name)
    if (exists(path)) out.push(path)
  }
  for (const name of ['instructions.md']) {
    const path = join(brandRoot, name)
    if (exists(path)) out.push(path)
  }
  for (const name of readdirSync(brandRoot).sort()) {
    if (!name.endsWith('.md') || name === 'instructions.md') {
      continue
    }
    out.push(join(brandRoot, name))
  }
  return out
}

function exists(path: string): boolean {
  try {
    return statSync(path) !== undefined
  } catch {
    return false
  }
}
