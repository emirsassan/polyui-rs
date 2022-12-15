module.exports = {
	pluginSearchDirs: ['.'],
	useTabs: true,
	printWidth: 100,
	singleQuote: true,
	trailingComma: 'none',
	bracketSameLine: false,
	semi: true,
	quoteProps: 'consistent',
	importOrder: ['^[./]', '^@polyui/interface/(.*)$', '^@polyui/client/(.*)$', '^@polyui/ui/(.*)$'],
	importOrderSeparation: true,
	importOrderSortSpecifiers: true,
	plugins: ['@trivago/prettier-plugin-sort-imports']
};