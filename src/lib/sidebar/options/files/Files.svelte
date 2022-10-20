<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { selectedBucket } from '$lib/state';
    import { Bucket } from '$lib/buckets/types';
    import Node from './Node.svelte';
    import { tree } from './utils';

    async function load(bucket: Bucket) {
        const paths: string[] = await invoke('list_bucket_tree', { bucket });
        return tree(paths);
    }

    $: promise = load($selectedBucket);
</script>

{#await promise}
    Loading...
{:then nodes}
    {#each nodes as node}
        <Node {node} />
    {/each}
{/await}
