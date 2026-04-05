from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "api-auth-bootstrap-services-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/services/api/logging.ts",
    "references/typescript/src/services/api/filesApi.ts",
    "references/typescript/src/services/api/dumpPrompts.ts",
    "references/typescript/src/services/api/client.ts",
    "references/typescript/src/services/api/ultrareviewQuota.ts",
    "references/typescript/src/services/api/claude.ts",
    "references/typescript/src/services/api/sessionIngress.ts",
    "references/typescript/src/services/api/promptCacheBreakDetection.ts",
    "references/typescript/src/services/api/emptyUsage.ts",
    "references/typescript/src/services/api/errorUtils.ts",
    "references/typescript/src/services/api/adminRequests.ts",
    "references/typescript/src/services/api/withRetry.ts",
    "references/typescript/src/services/api/codex-fetch-adapter.ts",
    "references/typescript/src/services/api/errors.ts",
    "references/typescript/src/services/api/grove.ts",
    "references/typescript/src/services/api/bootstrap.ts",
    "references/typescript/src/services/api/firstTokenDate.ts",
    "references/typescript/src/services/api/referral.ts",
    "references/typescript/src/services/api/metricsOptOut.ts",
    "references/typescript/src/services/api/overageCreditGrant.ts",
    "references/typescript/src/services/api/usage.ts",
    "references/typescript/src/services/oauth/codex-client.ts",
    "references/typescript/src/services/oauth/auth-code-listener.ts",
    "references/typescript/src/services/oauth/client.ts",
    "references/typescript/src/services/oauth/crypto.ts",
    "references/typescript/src/services/oauth/index.ts",
    "references/typescript/src/services/oauth/getOauthProfile.ts",
    "references/typescript/src/services/claudeAiLimits.ts",
]


def test_typescript_api_auth_bootstrap_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_api_auth_bootstrap_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 28
