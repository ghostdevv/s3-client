<script lang="ts">
    import { buckets, selectedBucketId } from '$lib/buckets/buckets';
    import { invoke } from '@tauri-apps/api/tauri';
    import { Bucket } from '$lib/buckets/types';
    import App from './App.svelte';

    const deps = [invoke<Bucket[]>('list_buckets')];

    async function start() {
        const [bucketsData] = await Promise.all(deps);

        $buckets = bucketsData;
        $selectedBucketId = bucketsData.length > 0 ? bucketsData[0].id : null;
    }
</script>

{#await start() then _}
    <App />
{/await}
