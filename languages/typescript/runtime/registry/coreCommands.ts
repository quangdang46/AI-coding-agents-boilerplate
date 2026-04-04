import { existsSync, readFileSync } from 'node:fs'
import { join } from 'node:path'

import { loadRuntimeSummary } from '../config/loadRuntimeSummary.ts'
import { loadContextSummary } from '../context/loadContextState.ts'
import { loadSessionSnapshot } from '../state/sessionState.ts'
import { inferBrandRoot, instructionCandidates } from '../utils/brand.ts'
import { loadRuntimeConfigSummary } from '../utils/config.ts'

export type CoreCommandName =
  | 'help'
  | 'status'
  | 'session'
  | 'export'
  | 'agents'
  | 'config'
  | 'doctor'
  | 'context'
  | 'theme'
  | 'output_style'
  | 'memory'
  | 'plan'
  | 'review'
  | 'model'
  | 'effort'
  | 'fast'
  | 'passes'
  | 'usage'
  | 'sandbox'
  | 'permissions'
  | 'files'
  | 'resume'
  | 'compact'
  | 'diff'
  | 'cost'
  | 'clear'
  | 'stats'
  | 'tasks'
  | 'tag'
  | 'vim'
  | 'color'
  | 'keybindings'
  | 'copy'
  | 'terminal'
  | 'exit'
  | 'hooks'
  | 'branch'
  | 'skills'
  | 'commit'
  | 'release_notes'
  | 'commit_push_pr'
  | 'statusline'
  | 'thinkback'
  | 'thinkback_play'
  | 'assistant'
  | 'rename'
  | 'pr_comments'
  | 'insights'
  | 'ide'
  | 'rate_limit_options'
  | 'upgrade'
  | 'init_verifiers'
  | 'create_moved_to_plugin_command'
  | 'add_dir'
  | 'issue'

export type CoreCommandHandler = (root: string) => string

export type CoreCommandDefinition = {
  name: CoreCommandName
  run: CoreCommandHandler
}

function readText(path: string): string {
  return readFileSync(path, 'utf8').trim()
}

function readState(path: string): Record<string, string> {
  if (!existsSync(path)) {
    return {}
  }
  return Object.fromEntries(
    readText(path)
      .split('\n')
      .filter((line) => line.includes('='))
      .map((line) => {
        const [key, ...rest] = line.split('=')
        return [key, rest.join('=')]
      }),
  )
}

function configSummary(root: string): string {
  const { defaultProvider, providerModel, approvalMode } = loadRuntimeConfigSummary(root)
  return `provider=${defaultProvider} model=${providerModel} approval_mode=${approvalMode}`
}

function sessionSummary(root: string): string {
  const latest = readState(join(inferBrandRoot(root), 'sessions/latest.state'))
  return `session_id=${latest.session_id ?? 'missing'} turn_count=${latest.turn_count ?? '0'}`
}

function helpSummary(root: string): string {
  const brandRoot = inferBrandRoot(root)
  const commandCount = coreCommands.length
  return `help_ready=true brand_root=${brandRoot.split('/').at(-1) ?? '.claude'} command_count=${commandCount}`
}

function statusSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `status=ready session_id=${session.sessionId} turn_count=${session.turnCount} usage_entries=${session.usageEntries}`
}

function exportSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  const brandRoot = inferBrandRoot(root)
  const exists = existsSync(join(root, session.exportPath))
  return `export_path=${session.exportPath} export_exists=${exists && brandRoot.length > 0}`
}

function agentsSummary(root: string): string {
  const brandRoot = inferBrandRoot(root)
  const agentDir = join(brandRoot, 'agents')
  return `agents_ready=${existsSync(agentDir)} agent_root=${agentDir.split('/').at(-2) ?? 'missing'}`
}

function contextSummary(root: string): string {
  const summary = loadContextSummary(root)
  return `prompt_digest=${summary.promptDigest} context_digest=${summary.contextDigest}`
}

function themeSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `theme_ready=true session_id=${session.sessionId} turn_count=${session.turnCount}`
}

function outputStyleSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `output_style=default usage_entries=${session.usageEntries} context_digest=${session.contextDigest}`
}

function usageSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `usage_entries=${session.usageEntries} total_cost_micros=${session.totalCostMicros}`
}

function memorySummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `memory_entries=${session.usageEntries} context_digest=${session.contextDigest}`
}

function planSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `plan_ready=true session_id=${session.sessionId} turn_count=${session.turnCount}`
}

function reviewSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `review_ready=true session_id=${session.sessionId} usage_entries=${session.usageEntries}`
}

function modelSummary(root: string): string {
  const { defaultProvider, providerModel } = loadRuntimeConfigSummary(root)
  return `model_provider=${defaultProvider} model_name=${providerModel}`
}

function effortSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `effort_level=normal turn_count=${session.turnCount} usage_entries=${session.usageEntries}`
}

function fastSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `fast_mode=available session_id=${session.sessionId} context_digest=${session.contextDigest}`
}

function passesSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `passes_ready=true usage_entries=${session.usageEntries} total_cost_micros=${session.totalCostMicros}`
}

function sandboxSummary(root: string): string {
  const summary = loadRuntimeSummary(root)
  return `sandbox_ready=true approval_mode=${summary.approvalMode} bash_policy=${summary.bashPolicy}`
}

function permissionSummary(root: string): string {
  const summary = loadRuntimeSummary(root)
  return `approval_mode=${summary.approvalMode} bash_policy=${summary.bashPolicy} file_write_policy=${summary.fileWritePolicy}`
}

function fileSummary(root: string): string {
  const brandRoot = inferBrandRoot(root)
  const sessionState = existsSync(join(brandRoot, 'sessions/local-main-session.state'))
  const exportState = existsSync(join(brandRoot, 'sessions/local-main-session.export.md'))
  const usageState = existsSync(join(brandRoot, 'sessions/summary.state'))
  return `session_state=${sessionState} export_state=${exportState} usage_state=${usageState}`
}

function statsSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `stats_ready=true usage_entries=${session.usageEntries} total_cost_micros=${session.totalCostMicros}`
}

function resumeSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `resume_session_id=${session.sessionId} resume_turn_count=${session.turnCount}`
}

function compactSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `compact_ready=true usage_entries=${session.usageEntries} export_path=${session.exportPath}`
}

function diffSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `diff_ready=true context_digest=${session.contextDigest} turn_count=${session.turnCount}`
}

function costSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `cost_entries=${session.usageEntries} total_cost_micros=${session.totalCostMicros}`
}

function clearSummary(root: string): string {
  const latest = readState(join(inferBrandRoot(root), 'sessions/latest.state'))
  const hasSession = (latest.session_id ?? '').length > 0
  return `clearable=${hasSession} retained_session_id=${latest.session_id ?? 'missing'}`
}

function tagSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `tag_ready=true session_id=${session.sessionId} context_digest=${session.contextDigest}`
}

function vimSummary(root: string): string {
  return 'vim_mode=available'
}

function colorSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `color_ready=true session_id=${session.sessionId} usage_entries=${session.usageEntries}`
}

function keybindingsSummary(root: string): string {
  return 'keybindings_ready=true profile=default'
}

function copySummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `copy_ready=true session_id=${session.sessionId} turn_count=${session.turnCount}`
}

function terminalSummary(root: string): string {
  const brandRoot = inferBrandRoot(root)
  return `terminal_ready=true brand_root=${brandRoot.split('/').at(-1) ?? '.claude'}`
}

function exitSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `exit_ready=true session_id=${session.sessionId} usage_entries=${session.usageEntries}`
}

function hooksSummary(root: string): string {
  const brandRoot = inferBrandRoot(root)
  return `hooks_ready=true instructions_present=${instructionCandidates(root, brandRoot).length > 0}`
}

function branchSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `branch_ready=true session_id=${session.sessionId} turn_count=${session.turnCount}`
}

function skillsSummary(root: string): string {
  const brandRoot = inferBrandRoot(root)
  const skillsDir = join(brandRoot, 'skills')
  return `skills_ready=${existsSync(skillsDir)} skills_root=${skillsDir.split('/').at(-2) ?? 'missing'}`
}

function commitSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `commit_ready=true session_id=${session.sessionId} usage_entries=${session.usageEntries}`
}

function releaseNotesSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `release_notes_ready=true session_id=${session.sessionId} turn_count=${session.turnCount}`
}

function commitPushPrSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `commit_push_pr_ready=true session_id=${session.sessionId} usage_entries=${session.usageEntries}`
}

function statuslineSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `statusline_ready=true session_id=${session.sessionId} turn_count=${session.turnCount}`
}

function thinkbackSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `thinkback_ready=true session_id=${session.sessionId} context_digest=${session.contextDigest}`
}

function thinkbackPlaySummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `thinkback_play_ready=true session_id=${session.sessionId} turn_count=${session.turnCount}`
}

function assistantSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `assistant_ready=true session_id=${session.sessionId} usage_entries=${session.usageEntries}`
}

function renameSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `rename_ready=true session_id=${session.sessionId} turn_count=${session.turnCount}`
}

function prCommentsSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `pr_comments_ready=true session_id=${session.sessionId} usage_entries=${session.usageEntries}`
}

function insightsSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `insights_ready=true context_digest=${session.contextDigest} usage_entries=${session.usageEntries}`
}

function ideSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  const { defaultProvider, providerModel } = loadRuntimeConfigSummary(root)
  return `ide_ready=true provider=${defaultProvider} model=${providerModel} session_id=${session.sessionId}`
}

function rateLimitOptionsSummary(root: string): string {
  const summary = loadRuntimeSummary(root)
  return `rate_limit_options_ready=true approval_mode=${summary.approvalMode} bash_policy=${summary.bashPolicy} file_write_policy=${summary.fileWritePolicy}`
}

function upgradeSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  const { defaultProvider, providerModel, approvalMode } = loadRuntimeConfigSummary(root)
  return `upgrade_ready=true provider=${defaultProvider} model=${providerModel} approval_mode=${approvalMode} session_id=${session.sessionId}`
}

function initVerifiersSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  const { defaultProvider, providerModel, approvalMode } = loadRuntimeConfigSummary(root)
  return `init_verifiers_ready=true provider=${defaultProvider} model=${providerModel} approval_mode=${approvalMode} session_id=${session.sessionId} turn_count=${session.turnCount}`
}

function createMovedToPluginCommandSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  const { defaultProvider, providerModel, approvalMode } = loadRuntimeConfigSummary(root)
  return `create_moved_to_plugin_command_ready=true provider=${defaultProvider} model=${providerModel} approval_mode=${approvalMode} session_id=${session.sessionId} context_digest=${session.contextDigest}`
}

function addDirSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  const { defaultProvider, providerModel, approvalMode } = loadRuntimeConfigSummary(root)
  return `add_dir_ready=true provider=${defaultProvider} model=${providerModel} approval_mode=${approvalMode} session_id=${session.sessionId} turn_count=${session.turnCount}`
}

function issueSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  const { defaultProvider, providerModel, approvalMode } = loadRuntimeConfigSummary(root)
  return `issue_ready=true provider=${defaultProvider} model=${providerModel} approval_mode=${approvalMode} session_id=${session.sessionId} context_digest=${session.contextDigest}`
}

function taskSummary(root: string): string {
  const latest = readState(join(inferBrandRoot(root), 'sessions/latest.state'))
  const turnCount = latest.turn_count ?? '0'
  return `task_count=1 active_task=session-loop turn_count=${turnCount}`
}

function doctorSummary(root: string): string {
  const brandRoot = inferBrandRoot(root)
  const required = [
    join(brandRoot, 'sessions/README.md'),
    join(brandRoot, 'sessions/latest.state'),
    join(brandRoot, 'sessions/summary.state'),
  ]
  const missing = required
    .filter((path) => !existsSync(path))
    .map((path) => path.replace(`${root}/`, ''))
  if (instructionCandidates(root, brandRoot).length === 0) {
    missing.push('instruction-surface')
  }
  return missing.length === 0 ? 'doctor=ok' : `doctor=missing:${missing.join(',')}`
}

export const coreCommands: CoreCommandDefinition[] = [
  { name: 'help', run: helpSummary },
  { name: 'status', run: statusSummary },
  { name: 'session', run: sessionSummary },
  { name: 'export', run: exportSummary },
  { name: 'agents', run: agentsSummary },
  { name: 'config', run: configSummary },
  { name: 'doctor', run: doctorSummary },
  { name: 'context', run: contextSummary },
  { name: 'theme', run: themeSummary },
  { name: 'output_style', run: outputStyleSummary },
  { name: 'memory', run: memorySummary },
  { name: 'plan', run: planSummary },
  { name: 'review', run: reviewSummary },
  { name: 'model', run: modelSummary },
  { name: 'effort', run: effortSummary },
  { name: 'fast', run: fastSummary },
  { name: 'passes', run: passesSummary },
  { name: 'usage', run: usageSummary },
  { name: 'sandbox', run: sandboxSummary },
  { name: 'permissions', run: permissionSummary },
  { name: 'files', run: fileSummary },
  { name: 'resume', run: resumeSummary },
  { name: 'compact', run: compactSummary },
  { name: 'diff', run: diffSummary },
  { name: 'cost', run: costSummary },
  { name: 'clear', run: clearSummary },
  { name: 'stats', run: statsSummary },
  { name: 'tasks', run: taskSummary },
  { name: 'tag', run: tagSummary },
  { name: 'vim', run: vimSummary },
  { name: 'color', run: colorSummary },
  { name: 'keybindings', run: keybindingsSummary },
  { name: 'copy', run: copySummary },
  { name: 'terminal', run: terminalSummary },
  { name: 'exit', run: exitSummary },
  { name: 'hooks', run: hooksSummary },
  { name: 'branch', run: branchSummary },
  { name: 'skills', run: skillsSummary },
  { name: 'commit', run: commitSummary },
  { name: 'release_notes', run: releaseNotesSummary },
  { name: 'commit_push_pr', run: commitPushPrSummary },
  { name: 'statusline', run: statuslineSummary },
  { name: 'thinkback', run: thinkbackSummary },
  { name: 'thinkback_play', run: thinkbackPlaySummary },
  { name: 'assistant', run: assistantSummary },
  { name: 'rename', run: renameSummary },
  { name: 'pr_comments', run: prCommentsSummary },
  { name: 'insights', run: insightsSummary },
  { name: 'ide', run: ideSummary },
  { name: 'rate_limit_options', run: rateLimitOptionsSummary },
  { name: 'upgrade', run: upgradeSummary },
  { name: 'init_verifiers', run: initVerifiersSummary },
  { name: 'create_moved_to_plugin_command', run: createMovedToPluginCommandSummary },
  { name: 'add_dir', run: addDirSummary },
  { name: 'issue', run: issueSummary },
]

export function getCoreCommandRegistry(): Record<CoreCommandName, CoreCommandHandler> {
  return Object.fromEntries(coreCommands.map((command) => [command.name, command.run])) as Record<
    CoreCommandName,
    CoreCommandHandler
  >
}
