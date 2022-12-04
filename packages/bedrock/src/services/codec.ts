export function getPolyCodec(): Codec {
    const bedrock = Codecs.bedrock;

    return new Codec('polyui_model_codec', {
        name: 'PolyUI Model',
        extension: 'json',
        remember: true,
        load_filter: {
            type: 'json',
            extensions: ['json'],
            condition: (model: any) => model.format_version
        },
        compile(options) {
            return bedrock.compile?.(options)
        },
        overwrite(content, path, callback) {
            return bedrock.overwrite?.(content, path, callback)
        },
        parse(data, path) {
            return bedrock.parse?.(data, path)
        },
        fileName(): string {
            return bedrock.fileName?.() ?? 'unknown'
        },
    })
}