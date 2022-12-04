const path = require("path")
const terser = require("terser-webpack-plugin")

/** @type {import("webpack").Configuration} */
module.exports = {
    entry: './src/index.ts',
    optimization: {
        minimize: true,
        minimizer: [
            new terser({
                terserOptions: {
                    format: {
                        preamble: '/* Blockbench "Polyfrost Bedrock" Plugin created by Polyfrost */',
                        comments: false
                    }
                },
                extractComments: false
            })
        ],

    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: 'ts-loader',
                exclude: /node_modules/
            }
        ]
    },
    resolve: {
        extensions: ['.ts', '.js']
    },
    output: {
        filename: 'polyui-bedrock.js',
        path: path.resolve(__dirname, 'dist')
    }
}