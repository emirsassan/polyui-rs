module.exports = {
    ...require('@polyui/config/eslint-base'),
    parserOptions: {
        tsconfigRootDir: __dirname,
        project: './tsconfig.json'
    }
};