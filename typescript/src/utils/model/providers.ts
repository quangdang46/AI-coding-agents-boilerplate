import type { AnalyticsMetadata_I_VERIFIED_THIS_IS_NOT_CODE_OR_FILEPATHS } from '../../services/analytics/index.js'
export { getAPIProvider, type APIProvider } from '../../core/providers/providerSelect.js'
export { isFirstPartyAnthropicBaseUrl } from '../../core/providers/anthropic.js'

import { getAPIProvider } from '../../core/providers/providerSelect.js'

export function getAPIProviderForStatsig(): AnalyticsMetadata_I_VERIFIED_THIS_IS_NOT_CODE_OR_FILEPATHS {
  return getAPIProvider() as AnalyticsMetadata_I_VERIFIED_THIS_IS_NOT_CODE_OR_FILEPATHS
}
