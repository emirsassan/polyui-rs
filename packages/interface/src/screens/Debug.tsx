import { useBridgeQuery, useLibraryQuery } from '@polyui/client';

import CodeBlock from '../components/primitive/Codeblock';
import { usePlatform } from '../util/Platform';

export default function DebugScreen() {
	const platform = usePlatform();
	const { data: nodeState } = useBridgeQuery(['nodeState']);
	const { data: libraryState } = useBridgeQuery(['library.list']);
	const { data: jobs } = useLibraryQuery(['jobs.getRunning']);
	const { data: jobHistory } = useLibraryQuery(['jobs.getHistory']);
	const { data: buildInfo } = useBridgeQuery(['buildInfo']);

	return (
		<div className="flex flex-col w-full h-screen custom-scroll page-scroll app-background">
			<div data-tauri-drag-region className="flex flex-shrink-0 w-full h-5" />
			<div className="flex flex-col p-5 pt-2 space-y-5 pb-7">
				<h1 className="text-lg font-bold ">Developer Debugger</h1>
				<h1 className="text-sm font-bold ">Running Jobs</h1>
				<CodeBlock src={{ ...jobs }} />
				<h1 className="text-sm font-bold ">Job History</h1>
				<CodeBlock src={{ ...jobHistory }} />
				<h1 className="text-sm font-bold ">Node State</h1>
				<CodeBlock src={{ ...nodeState }} />
				<h1 className="text-sm font-bold ">Libraries</h1>
				<CodeBlock src={{ ...libraryState }} />
				<h1 className="text-sm font-bold">Build Info</h1>
				<CodeBlock src={{ ...buildInfo }} />
			</div>
		</div>
	);
}
