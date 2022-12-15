import Module, { syncBuiltinESMExports } from 'module';
import { join } from 'path';

module.exports = {
	config(dir = process.cwd(), override = false) {
		Module.prototype.require = new Proxy(Module.prototype.require, {
			apply(target, thisArg: Window, args: Array<string>) {
				const name = args[0];

				if (name.includes('~')) {
					const path = name.split('/').slice(1);
					args[0] = join(dir, ...path);
				}

				if (override) {
					args[0] = '@polyui/proxy';
				}

				return Reflect.apply(target, thisArg, args);
			}
		});
	}
};

module.exports = {
	config(dir = process.cwd(), override = false) {
		const polyimport = new Proxy(syncBuiltinESMExports, {
			apply(target, thisArg: Window, args: Array<string>) {
				return Reflect.apply(target, thisArg, args);
			}
		});
	}
};
