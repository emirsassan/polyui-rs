export type NormiCache = Map<string /* $type */, Map<string /* $id */, any>>;

declare global {
	interface Window {
		normiCache?: NormiCache;
	}
}

export interface NormiOptions {
	contextSharing?: boolean;
}

export function getNormiCache(contextSharing: boolean): NormiCache {
	if (contextSharing) {
		if (window.normiCache === undefined) {
			window.normiCache = new Map();
		}

		return window.normiCache;
	} else {
		return new Map();
	}
}

export function getOrCreate<K, A, B>(map: Map<K, Map<A, B>>, key: K): Map<A, B> {
	let m = map.get(key);
	if (m === undefined) {
		m = new Map();
		map.set(key, m);
	}
	return m;
}

export function normaliseValue(value: any, normiCache: NormiCache): any {
	if (value === null || value === undefined) {
		return value;
	} else if (typeof value === 'object') {
		if ('$id' in value && '$type' in value) {
			getOrCreate(normiCache, value.$type).set(value.$id, normaliseValueForStorage(value, true));
			delete value.$id;
			delete value.$type;
		} else if ('$type' in value && 'edges' in value) {
			value = (value.edges as any[]).map((v) => normaliseValue(v, normiCache));
		}

		for (const [k, v] of Object.entries(value)) {
			value[k] = normaliseValue(v, normiCache);
		}
	}

	return value;
}

export function normaliseValueForStorage(value: any, rootElem: boolean): any {
	if (value === null || value === undefined) {
		return value;
	} else if (typeof value === 'object') {
		if ('$id' in value && '$type' in value) {
			if (rootElem) {
				let v = Object.assign({}, value);
				delete v.$id;
				delete v.$type;

				for (const [k, vv] of Object.entries(v)) {
					v[k] = normaliseValueForStorage(vv, false);
				}

				return v;
			}

			for (const [k, v] of Object.entries(value)) {
				value[k] = normaliseValueForStorage(v, false);
			}

			return {
				$id: value.$id,
				$type: value.$type
			};
		} else if ('$type' in value && 'edges' in value) {
			return {
				$type: value.$type,
				edges: Object.values(value.edges as any[]).map((v) => v.$id)
			};
		}

		for (const [k, v] of Object.entries(value)) {
			value[k] = normaliseValueForStorage(v, false);
		}
	}

	return value;
}

export function recomputeNormalisedValueFromStorage(value: any, normiCache: NormiCache): any {
	if (value === null || value === undefined) {
		return value;
	} else if (typeof value === 'object') {
		if ('$id' in value && '$type' in value) {
			value = normiCache.get(value.$type)!.get(value.$id);
		} else if ('$type' in value && 'edges' in value) {
			value = (value.edges as any[]).map((id) => normiCache.get(value.$type)!.get(id));
		}

		for (const [k, v] of Object.entries(value)) {
			value[k] = recomputeNormalisedValueFromStorage(v, normiCache);
		}
	}

	return value;
}

export function loadDataFromCache(value: any, normiCache: NormiCache): any {
	if (value === null || value === undefined) {
		return value;
	} else if (typeof value === 'object') {
		if ('$id' in value && '$type' in value) {
			let v = Object.assign({}, value);
			delete v.$id;
			delete v.$type;

			for (const [k, v] of Object.entries(value)) {
				value[k] = normaliseValueForStorage(v, false);
			}

			return v;
		} else if ('$type' in value && 'edges' in value) {
			return [];
		}

		for (const [k, v] of Object.entries(value)) {
			value[k] = normaliseValueForStorage(v, false);
		}
	}

	return value;
}
