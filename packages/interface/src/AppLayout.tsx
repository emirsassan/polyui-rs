import { useCurrentLibrary } from '@polyui/client';
import clsx from 'clsx';
import { Suspense } from 'react';
import { Outlet } from 'react-router-dom';

import { Sidebar } from './components/layout/Sidebar';
import { Toasts } from './components/primitive/Toasts';
import { useOperatingSystem } from './hooks/useOperatingSystem';

export function AppLayout() {
	const { libraries } = useCurrentLibrary();
	const os = useOperatingSystem();

	if (libraries?.length === 0) return null;

	return (
		<div
			className={clsx(
				'flex h-screen overflow-hidden text-ink select-none cursor-default',
				os === 'macOS' && 'rounded-[10px] has-blur-effects',
				os !== 'browser' && os !== 'windows' && 'border border-app-frame'
			)}
			onContextMenu={(e) => {
				e.preventDefault();
				return false;
			}}
		>
			<Sidebar />
			<div className="relative flex w-full">
				<Suspense fallback={<div className="w-screen h-screen bg-app" />}>
					<Outlet />
				</Suspense>
			</div>
			<Toasts />
		</div>
	);
}
