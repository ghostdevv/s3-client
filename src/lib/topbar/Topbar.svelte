<script lang="ts">
    import { faPlus, faArchive } from '@fortawesome/free-solid-svg-icons';
    import { buckets, selectedBucketId } from '$lib/buckets/buckets';
    import Fa from 'svelte-fa';
</script>

<div class="topbar">
    <button class="nav-button">
        <Fa icon={faPlus} />
    </button>

    {#each $buckets as bucket}
        <button
            class="nav-button"
            on:click={() => ($selectedBucketId = bucket.id)}>
            <Fa icon={faArchive} />
            <p>{bucket.name}</p>
        </button>
    {/each}
</div>

<style lang="scss">
    .topbar {
        grid-area: topbar;

        width: 100%;
        height: fit-content;

        display: flex;
        align-items: stretch;
        gap: 16px;

        background-color: var(--background-secondary);

        .nav-button {
            border-radius: 0px;
            border: 0px;

            word-wrap: normal;
            white-space: nowrap;

            box-shadow: none;

            &.active {
                border-bottom: 2px solid var(--primary);
            }
        }
    }
</style>
