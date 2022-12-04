/// <reference types="blockbench-types" />
import events from './events';
import { getPolyAction } from './services/action';
import { getPolyCodec } from './services/codec';
import { getPolyFormat } from './services/format';
import { getPolyModel } from './services/model';
import { groups } from './utils';

(async function () {
	let format: ModelFormat,
		codec: Codec,
		action: Action,
		config: Action,
		setResolution: (data?: object) => void,
		textureAdd: (data?: object) => void,
		model: THREE.Object3D;

	events.on('load', async () => {
		codec = await getPolyCodec();
		model = await getPolyModel();
		format = await getPolyFormat(codec);
		action = await getPolyAction();

		MenuBar.addAction(action, 'file.export.0');
		codec.export_action = action;

		setResolution = () => {
			if (Project?.format === format) {
				Project.texture_width = Project.texture_height = 64;
				for (const group of groups) new Group(group).init();
			}
		};
		Blockbench.on('new_project', setResolution);

		textureAdd = (e) => {
			if (Project?.format === format) {
				Texture.all.map((tex) => tex.getMaterial()).forEach((e) => (e.transparent = false));
			}
		};
		textureAdd();
		Blockbench.on('add_texture', textureAdd);

		events.once('unload', async () => {
			Blockbench.removeListener('new_project');
			Blockbench.removeListener('add_texture');
			codec.delete();
			format.delete();
			action.delete();
			config.delete();
		});
	});

	// @ts-ignore
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
