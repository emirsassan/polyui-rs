module.exports = {
	...require('@polyui/config/eslint-config.js'),
	parserOptions: {
		tsconfigRootDir: __dirname,
		project: './tsconfig.json'
	},
	ignorePatterns: ['**/*.js', '**/*.json', 'node_modules', 'public', 'dist', 'server', 'vite.config.ts']
};