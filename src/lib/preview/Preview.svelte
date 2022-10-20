<script lang="ts">
    import { faTriangleExclamation } from '@fortawesome/free-solid-svg-icons';
    import { faSpinner } from '@fortawesome/free-solid-svg-icons';
    import { faFile } from '@fortawesome/free-regular-svg-icons';
    import { selectedBucket, selectedFile } from '$lib/state';
    import type { FileMeta } from '$lib/buckets/types';
    import { invoke } from '@tauri-apps/api/tauri';
    import prettyBytes from 'pretty-bytes';
    import Icon from 'svelte-fa';

    function loadMeta(file: string): Promise<FileMeta> {
        return invoke('get_file_meta', {
            bucket: $selectedBucket,
            file,
        });
    }

    function getFileName(path: string) {
        return path.split('/').pop();
    }
</script>

<div class="preview" class:sidebar={!!$selectedBucket}>
    {#if $selectedFile}
        {#await loadMeta($selectedFile)}
            <Icon
                spin
                icon={faSpinner}
                size="3x"
                color="rgba(var(--text-rgb), 0.5)" />
        {:then meta}
            <Icon icon={faFile} size="8x" color="rgba(var(--text-rgb), 0.5)" />
            <h2>{getFileName($selectedFile)}</h2>
            <p>{prettyBytes(meta.size)}</p>
        {:catch error}
            <Icon
                icon={faTriangleExclamation}
                size="8x"
                color="rgba(var(--text-rgb), 0.5)" />

            <h4 class="error">{error}</h4>
        {/await}
    {/if}
</div>

<style lang="scss">
    .preview {
        grid-area: preview;
        padding: 16px;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 16px;

        border-top: 2px solid var(--background-tertiary);

        &.sidebar {
            border-left: 2px solid var(--background-tertiary);
        }
    }

    .error {
        color: var(--red);
    }
</style>
