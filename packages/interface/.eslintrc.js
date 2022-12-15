module.exports = {
	...require('@polyui/config/eslint-config.js'),
	parserOptions: {
		tsconfigRootDir: __dirname,
		project: './tsconfig.json'
	}
};