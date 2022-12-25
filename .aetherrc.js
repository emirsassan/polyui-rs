module.exports = {
    baseUri: './',
    pluginExt: 'aes://aether.local/plugin',
    plugin: {
        entrypoint: ['./core', './package.json'],
        command: 'INJECT'
    },
    init: (injector) => {
        injector({
            outline: 'escape'
        })
    }
}