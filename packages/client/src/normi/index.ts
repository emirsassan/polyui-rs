import { CustomHooks } from '@rspc/client';
// @ts-expect-error
import { __useMutation, __useQuery } from '@rspc/react/internal';

import { NormiOptions } from './utils';

export function normiCustomHooks(
	{ contextSharing }: NormiOptions,
	nextHooks?: () => CustomHooks
): () => CustomHooks {
	const next = nextHooks?.();

	return () => ({
		mapQueryKey: next?.mapQueryKey,
		doQuery: next?.doQuery,
		doMutation: next?.doMutation
	});
}
