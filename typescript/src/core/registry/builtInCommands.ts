// biome-ignore-all assist/source/organizeImports: built-in command ownership is centralized here intentionally
import { feature } from 'bun:bundle'
import memoize from 'lodash-es/memoize.js'
import { type Command } from '../../types/command.js'
import { isUsing3PServices } from '../../utils/auth.js'
import { buildCommandCatalog, buildCommandNameSet } from './commandCatalog.js'

import addDir from '../../commands/add-dir/index.js'
import advisor from '../../commands/advisor.js'
import agents from '../../commands/agents/index.js'
import antTrace from '../../commands/ant-trace/index.js'
import autofixPr from '../../commands/autofix-pr/index.js'
import backfillSessions from '../../commands/backfill-sessions/index.js'
import branch from '../../commands/branch/index.js'
import breakCache from '../../commands/break-cache/index.js'
import btw from '../../commands/btw/index.js'
import bughunter from '../../commands/bughunter/index.js'
import chrome from '../../commands/chrome/index.js'
import clear from '../../commands/clear/index.js'
import color from '../../commands/color/index.js'
import commit from '../../commands/commit.js'
import commitPushPr from '../../commands/commit-push-pr.js'
import compact from '../../commands/compact/index.js'
import config from '../../commands/config/index.js'
import { context, contextNonInteractive } from '../../commands/context/index.js'
import copy from '../../commands/copy/index.js'
import cost from '../../commands/cost/index.js'
import ctx_viz from '../../commands/ctx_viz/index.js'
import debugToolCall from '../../commands/debug-tool-call/index.js'
import desktop from '../../commands/desktop/index.js'
import diff from '../../commands/diff/index.js'
import doctor from '../../commands/doctor/index.js'
import effort from '../../commands/effort/index.js'
import env from '../../commands/env/index.js'
import exit from '../../commands/exit/index.js'
import exportCommand from '../../commands/export/index.js'
import {
  extraUsage,
  extraUsageNonInteractive,
} from '../../commands/extra-usage/index.js'
import fast from '../../commands/fast/index.js'
import feedback from '../../commands/feedback/index.js'
import files from '../../commands/files/index.js'
import goodClaude from '../../commands/good-claude/index.js'
import heapDump from '../../commands/heapdump/index.js'
import help from '../../commands/help/index.js'
import hooks from '../../commands/hooks/index.js'
import ide from '../../commands/ide/index.js'
import init from '../../commands/init.js'
import initVerifiers from '../../commands/init-verifiers.js'
import installGitHubApp from '../../commands/install-github-app/index.js'
import installSlackApp from '../../commands/install-slack-app/index.js'
import issue from '../../commands/issue/index.js'
import keybindings from '../../commands/keybindings/index.js'
import login from '../../commands/login/index.js'
import logout from '../../commands/logout/index.js'
import mcp from '../../commands/mcp/index.js'
import memory from '../../commands/memory/index.js'
import mobile from '../../commands/mobile/index.js'
import mockLimits from '../../commands/mock-limits/index.js'
import model from '../../commands/model/index.js'
import oauthRefresh from '../../commands/oauth-refresh/index.js'
import onboarding from '../../commands/onboarding/index.js'
import outputStyle from '../../commands/output-style/index.js'
import passes from '../../commands/passes/index.js'
import perfIssue from '../../commands/perf-issue/index.js'
import permissions from '../../commands/permissions/index.js'
import plan from '../../commands/plan/index.js'
import plugin from '../../commands/plugin/index.js'
import pr_comments from '../../commands/pr_comments/index.js'
import privacySettings from '../../commands/privacy-settings/index.js'
import rateLimitOptions from '../../commands/rate-limit-options/index.js'
import releaseNotes from '../../commands/release-notes/index.js'
import reloadPlugins from '../../commands/reload-plugins/index.js'
import remoteEnv from '../../commands/remote-env/index.js'
import rename from '../../commands/rename/index.js'
import resume from '../../commands/resume/index.js'
import review, { ultrareview } from '../../commands/review.js'
import rewind from '../../commands/rewind/index.js'
import sandboxToggle from '../../commands/sandbox-toggle/index.js'
import securityReview from '../../commands/security-review.js'
import session from '../../commands/session/index.js'
import share from '../../commands/share/index.js'
import skills from '../../commands/skills/index.js'
import stats from '../../commands/stats/index.js'
import status from '../../commands/status/index.js'
import statusline from '../../commands/statusline.js'
import stickers from '../../commands/stickers/index.js'
import summary from '../../commands/summary/index.js'
import tag from '../../commands/tag/index.js'
import tasks from '../../commands/tasks/index.js'
import teleport from '../../commands/teleport/index.js'
import terminalSetup from '../../commands/terminalSetup/index.js'
import theme from '../../commands/theme/index.js'
import thinkback from '../../commands/thinkback/index.js'
import thinkbackPlay from '../../commands/thinkback-play/index.js'
import upgrade from '../../commands/upgrade/index.js'
import usage from '../../commands/usage/index.js'
import version from '../../commands/version.js'
import vim from '../../commands/vim/index.js'
import {
  resetLimits,
  resetLimitsNonInteractive,
} from '../../commands/reset-limits/index.js'
import bridgeKick from '../../commands/bridge-kick.js'

/* eslint-disable @typescript-eslint/no-require-imports */
const agentsPlatform =
  process.env.USER_TYPE === 'ant'
    ? (require('../../commands/agents-platform/index.js').default as Command)
    : null
const proactive =
  feature('PROACTIVE') || feature('KAIROS')
    ? (require('../../commands/proactive.js').default as Command)
    : null
const briefCommand =
  feature('KAIROS') || feature('KAIROS_BRIEF')
    ? (require('../../commands/brief.js').default as Command)
    : null
const assistantCommand = feature('KAIROS')
  ? (require('../../commands/assistant/index.js').default as Command)
  : null
const bridge = feature('BRIDGE_MODE')
  ? (require('../../commands/bridge/index.js').default as Command)
  : null
const remoteControlServerCommand =
  feature('DAEMON') && feature('BRIDGE_MODE')
    ? (require('../../commands/remoteControlServer/index.js').default as Command)
    : null
const voiceCommand = feature('VOICE_MODE')
  ? (require('../../commands/voice/index.js').default as Command)
  : null
const forceSnip = feature('HISTORY_SNIP')
  ? (require('../../commands/force-snip.js').default as Command)
  : null
const workflowsCmd = feature('WORKFLOW_SCRIPTS')
  ? (require('../../commands/workflows/index.js').default as Command)
  : null
const webCmd = feature('CCR_REMOTE_SETUP')
  ? (require('../../commands/remote-setup/index.js').default as Command)
  : null
const subscribePr = feature('KAIROS_GITHUB_WEBHOOKS')
  ? (require('../../commands/subscribe-pr.js').default as Command)
  : null
const ultraplan = feature('ULTRAPLAN')
  ? (require('../../commands/ultraplan.js').default as Command)
  : null
const torch = feature('TORCH')
  ? (require('../../commands/torch.js').default as Command)
  : null
const peersCmd = feature('UDS_INBOX')
  ? (require('../../commands/peers/index.js').default as Command)
  : null
const forkCmd = feature('FORK_SUBAGENT')
  ? (require('../../commands/fork/index.js').default as Command)
  : null
const buddy = feature('BUDDY')
  ? (require('../../commands/buddy/index.js').default as Command)
  : null
/* eslint-enable @typescript-eslint/no-require-imports */

const usageReport: Command = {
  type: 'prompt',
  name: 'insights',
  description: 'Generate a report analyzing your Claude Code sessions',
  contentLength: 0,
  progressMessage: 'analyzing your sessions',
  source: 'builtin',
  async getPromptForCommand(args, context) {
    const real = (await import('../../commands/insights.js')).default
    if (real.type !== 'prompt') throw new Error('unreachable')
    return real.getPromptForCommand(args, context)
  },
}

export const INTERNAL_ONLY_COMMANDS = [
  backfillSessions,
  breakCache,
  bughunter,
  commit,
  commitPushPr,
  ctx_viz,
  goodClaude,
  issue,
  initVerifiers,
  ...(forceSnip ? [forceSnip] : []),
  mockLimits,
  bridgeKick,
  version,
  ...(subscribePr ? [subscribePr] : []),
  resetLimits,
  resetLimitsNonInteractive,
  onboarding,
  share,
  summary,
  teleport,
  antTrace,
  perfIssue,
  env,
  oauthRefresh,
  debugToolCall,
  agentsPlatform,
  autofixPr,
].filter(Boolean) as Command[]

const BUILT_IN_COMMANDS = memoize((): Command[] =>
  buildCommandCatalog({
    primary: [
      addDir,
      advisor,
      agents,
      branch,
      btw,
      chrome,
      clear,
      color,
      compact,
      config,
      copy,
      desktop,
      context,
      contextNonInteractive,
      cost,
      diff,
      doctor,
      effort,
      exit,
      fast,
      files,
      heapDump,
      help,
      ide,
      init,
      keybindings,
      installGitHubApp,
      installSlackApp,
      mcp,
      memory,
      mobile,
      model,
      outputStyle,
      remoteEnv,
      plugin,
      pr_comments,
      releaseNotes,
      reloadPlugins,
      rename,
      resume,
      session,
      skills,
      stats,
      status,
      statusline,
      stickers,
      tag,
      theme,
      feedback,
      review,
      ultrareview,
      rewind,
      securityReview,
      terminalSetup,
      upgrade,
      extraUsage,
      extraUsageNonInteractive,
      rateLimitOptions,
      usage,
      usageReport,
      vim,
      thinkback,
      thinkbackPlay,
      permissions,
      plan,
      privacySettings,
      hooks,
      exportCommand,
      sandboxToggle,
      ...(!isUsing3PServices() ? [logout, login()] : []),
      passes,
      tasks,
    ],
    optional: [
      webCmd,
      forkCmd,
      buddy,
      proactive,
      briefCommand,
      assistantCommand,
      bridge,
      remoteControlServerCommand,
      voiceCommand,
      peersCmd,
      workflowsCmd,
      torch,
      ultraplan,
    ],
    internalOnly: INTERNAL_ONLY_COMMANDS,
    includeInternalOnly:
      process.env.USER_TYPE === 'ant' && !process.env.IS_DEMO,
  }),
)

export function getBuiltInCommands(): Command[] {
  return BUILT_IN_COMMANDS()
}

export const builtInCommandNames = memoize(
  (): Set<string> => buildCommandNameSet(getBuiltInCommands()),
)

export const REMOTE_SAFE_COMMANDS: Set<Command> = new Set([
  session,
  exit,
  clear,
  help,
  theme,
  color,
  vim,
  cost,
  usage,
  copy,
  btw,
  feedback,
  plan,
  keybindings,
  statusline,
  stickers,
  mobile,
])

export const BRIDGE_SAFE_COMMANDS: Set<Command> = new Set(
  [compact, clear, cost, summary, releaseNotes, files].filter(
    (command): command is Command => command !== null,
  ),
)
