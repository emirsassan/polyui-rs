export async function getPolyAction() {
	return new Action('export_polyui_model', {
		name: 'Export PolyUI Model',
		icon: 'fa-file-export',
		category: 'file',
		condition: {
			formats: ['polyui']
		},
		click: () => {
			Codecs.polyui.export?.();
		}
	});
}
