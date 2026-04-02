import { expect, test } from 'bun:test'
import { fileURLToPath } from 'node:url'

test('CLI version fast-path starts successfully', () => {
  const proc = Bun.spawnSync({
    cmd: [process.execPath, 'run', './src/entrypoints/cli.tsx', '--version'],
    cwd: fileURLToPath(new URL('../../', import.meta.url)),
    stdout: 'pipe',
    stderr: 'pipe',
    env: { ...process.env, NODE_ENV: 'test' },
  })

  expect(proc.exitCode).toBe(0)
  const stdout = new TextDecoder().decode(proc.stdout)
  expect(stdout).toContain('Claude Code')
})
