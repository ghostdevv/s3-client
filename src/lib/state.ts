import { derived, writable } from 'svelte/store';
import type { Bucket } from './buckets/types';

export const buckets = writable<Bucket[]>([]);
export const selectedBucketId = writable<number | null>(null);

export const selectedFile = writable<string | null>(null);

export const selectedBucket = derived(
    [buckets, selectedBucketId],
    ([buckets, id]) =>
        id && buckets.length ? buckets.find((bucket) => bucket.id == id) : null,
);
