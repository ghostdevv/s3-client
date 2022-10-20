<script lang="ts">
    import { buckets, selectedBucketId, selectedFile } from '$lib/state';
    import { invoke } from '@tauri-apps/api/tauri';
    import { Bucket } from '$lib/buckets/types';
    import App from './App.svelte';

    async function start() {
        const deps = [invoke<Bucket[]>('list_buckets')];
        const [bucketsData] = await Promise.all(deps);

        $buckets = bucketsData;
        $selectedBucketId = null;
        $selectedFile = null;
    }
</script>

{#await start() then _}
    <App />
{/await}
