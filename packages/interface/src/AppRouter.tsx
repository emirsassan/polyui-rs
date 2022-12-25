import { lazy } from '@loadable/component';
import { useCurrentLibrary, useInvalidateQuery } from '@polyui/client';
import { Navigate, Route, Routes } from 'react-router';

import { AppLayout } from './AppLayout';
import { useKeybindHandler } from './hooks/useKeyboardHandler';

export function AppRouter() {
	const { library } = useCurrentLibrary();

	useKeybindHandler();
	useInvalidateQuery();

	return (
		<Routes>
			<Route path="onboarding" element={<OnboardingScreen />} />
			<Route element={<AppLayout />}>
				{library === undefined ? (
					<Route
						path="*"
						element={<h1 className="p-4 text-white">Please install an instance.</h1>}
					/>
				) : (
					<></>
				)}
			</Route>
		</Routes>
	);
}
