import React, { Suspense } from 'react';
import ReactDOM from 'react-dom/client';

import App from './App';
import './patches';

import '@polyui/ui/style';

const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);
root.render(
	<React.StrictMode>
		<Suspense>
			<App />
		</Suspense>
	</React.StrictMode>
);
