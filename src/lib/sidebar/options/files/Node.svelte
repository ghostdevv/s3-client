<script lang="ts">
    import { selectedFile } from '$lib/state';
    import type { TreeNode } from './utils';
    import Folder from './Folder.svelte';

    export let node: TreeNode;
</script>

{#if node.children.length}
    <Folder {node} />
{:else}
    <div
        class="item"
        on:click={() => ($selectedFile = node.path)}
        class:selected={$selectedFile == node.path}>
        <p>{node.name}</p>
    </div>
{/if}

<style lang="scss">
    .item {
        padding: 4px 8px;

        display: flex;
        align-items: center;
        gap: 8px;

        cursor: pointer;

        &:hover:not(.children:hover),
        &:focus:not(.children:focus),
        &.selected {
            background-color: var(--background-tertiary);
        }
    }
</style>
