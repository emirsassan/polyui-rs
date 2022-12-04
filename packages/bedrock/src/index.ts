/// <reference types="blockbench-types" />
import events from './events';

(async function () {
	events.on('load', () => {
        
		events.once('unload', () => {

        });
	});

	Plugin.register('polyui_bedrock', {
		title: 'Polyfrost Bedrock',
		author: 'Polyfrost',
		description: 'Blockbench Plugin for Polyfrost internal use',
		icon: 'fa-brackets-curly',
		variant: 'both',
		oninstall: () => {
			events.emit('install');
		},
		onload: () => {
			events.emit('load');
		},
		onuninstall: () => {
			events.emit('uninstall');
		},
		onunload: () => {
			events.emit('unload');
		}
	} as unknown as PluginData);
})();
