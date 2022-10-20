<script lang="ts">
    import { selectedBucket } from '$lib/state';
    import { invoke } from '@tauri-apps/api/tauri';
    import RenderNode from './RenderNode.svelte';
    import { Bucket } from '$lib/buckets/types';
    import toTree from 'path-list-to-tree';

    async function load(bucket: Bucket) {
        const paths: string[] = await invoke('list_bucket_tree', {
            bucket: $selectedBucket,
        });

        return toTree(paths);
    }

    $: promise = load($selectedBucket);
</script>

{#await promise}
    Loading...
{:then nodes}
    {#each nodes as node}
        <RenderNode {node} />
    {/each}
{/await}
