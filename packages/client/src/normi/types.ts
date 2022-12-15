import { ProcedureDef } from '@rspc/client';

export type OmitDistributive<T, K extends PropertyKey> = T extends any
	? T extends object
		? Id<OmitRecursively<T, K>>
		: T
	: never;
export type Id<T> = {} & { [P in keyof T]: T[P] };
export type OmitRecursively<T extends any, K extends PropertyKey> = Omit<
	{ [P in keyof T]: OmitDistributive<T[P], K> },
	K
>;

export type Normalized<T extends ProcedureDef> = T extends any
	? {
			key: T['key'];
			// TODO: Typescript transformation for arrays
			result: OmitRecursively<T['result'], '$id' | '$type'>;
			input: T['input'];
	  }
	: never;
