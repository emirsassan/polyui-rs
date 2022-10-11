import type { Options } from './types'
import unplugin from '.'

export default function (this: any, options?: Options) {
  this.extendBuild((config: any) => {
    config.plugins = config.plugins || []
    config.plugins.unshift(unplugin.webpack(options))
  })

  this.nuxt.hook('vite:extend', async (vite: any) => {
    vite.config.plugins = vite.config.plugins || []
    vite.config.plugins.push(unplugin.vite(options))
  })
}
