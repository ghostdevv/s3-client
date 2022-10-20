<script lang="ts">
    import { selectedBucket, selectedFile } from '$lib/state';
    import Sidebar from '$lib/sidebar/Sidebar.svelte';
    import Topbar from '$lib/topbar/Topbar.svelte';
    import Dev from '$lib/Dev.svelte';

    // Hook for selectedBucket change
    $: if ($selectedBucket) {
        $selectedFile = null;
    }
</script>

<Dev />

<main>
    <div class="bar-spacer" />

    <Sidebar />

    <Topbar />

    <div class="preview" class:sidebar={!!$selectedBucket} />
</main>

<style lang="scss">
    main {
        height: 100vh;
        width: 100%;

        display: grid;
        grid-template-columns: max-content 1fr;
        grid-template-rows: max-content 1fr;
        grid-template-areas:
            'bar-spacer topbar'
            'sidebar preview'
            'sidebar preview';
    }

    .preview {
        grid-area: preview;
        padding: 16px;

        border-top: 2px solid var(--background-tertiary);

        &.sidebar {
            border-left: 2px solid var(--background-tertiary);
        }
    }

    .bar-spacer {
        grid-area: bar-spacer;
        background-color: var(--background-secondary);

        display: grid;
        place-items: center;
    }
</style>
