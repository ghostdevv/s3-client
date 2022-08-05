import { derived, writable } from 'svelte/store';
import type { Bucket } from './types';

export const buckets = writable<Bucket[]>([]);
export const selectedBucketId = writable<number | null>(null);

export const selectedBucket = derived(
    [buckets, selectedBucketId],
    ([buckets, id]) => (id && buckets.length ? buckets[id] : null),
);
