import { PropsWithChildren, createContext, useCallback, useContext, useMemo } from 'react';
import { proxy, subscribe, useSnapshot } from 'valtio';

import { useBridgeQuery } from '../rspc';
import { valtioPersist } from '../stores';

const libraryCacheLocalStorageKey = 'polyui-library-list';

type OnNoLibraryFunc = () => void | Promise<void>;

const currentLibraryUuidStore = valtioPersist('sdActiveLibrary', {
	id: null as string | null
});

const CringeContext = createContext<{
	onNoLibrary: OnNoLibraryFunc;
}>(undefined!);

export const LibraryContextProvider = ({
	onNoLibrary,
	children
}: PropsWithChildren<{ onNoLibrary: OnNoLibraryFunc }>) => {
	return <CringeContext.Provider value={{ onNoLibrary }}>{children}</CringeContext.Provider>;
};

export function getLibraryIdRaw(): string | null {
	return currentLibraryUuidStore.id;
}

export function onLibraryChange(func: (newLibraryId: string | null) => void) {
	subscribe(currentLibraryUuidStore, () => func(currentLibraryUuidStore.id));
}

export const useCurrentLibrary = () => {
	const currentLibraryUuid = useSnapshot(currentLibraryUuidStore).id;
	const ctx = useContext(CringeContext);
	if (ctx === undefined)
		throw new Error(
			"The 'LibraryContextProvider' was not mounted and you attempted do use the 'useCurrentLibrary' hook. Please add the provider in your component tree."
		);
	const { data: libraries, isLoading } = useBridgeQuery(['library.list'], {
		keepPreviousData: true,
		initialData: () => {
			const cachedData = localStorage.getItem(libraryCacheLocalStorageKey);
			if (cachedData) {
				try {
					return JSON.parse(cachedData);
				} catch (e) {
					console.error("Error loading cached 'polyui-library-list' data", e);
				}
			}
			return undefined;
		},
		onSuccess: (data: any) => {
			localStorage.setItem(libraryCacheLocalStorageKey, JSON.stringify(data));

			if (data?.length === 0) {
				ctx.onNoLibrary();
			}
		}
	});

	const switchLibrary = useCallback((libraryUuid: string) => {
		currentLibraryUuidStore.id = libraryUuid;
	}, []);

	const library = useMemo(() => {
		const current = libraries?.find((l: any) => l.uuid === currentLibraryUuid);
		if (libraries && !current && libraries[0]?.uuid) {
			switchLibrary(libraries[0]?.uuid);
		}

		return current;
	}, [libraries, currentLibraryUuid]);

	return {
		library,
		libraries,
		isLoading,
		switchLibrary
	};
};
