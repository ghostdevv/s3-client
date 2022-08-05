<script lang="ts">
    import { faFile, faGear } from '@fortawesome/free-solid-svg-icons';
    import { selectedBucket } from '$lib/buckets/buckets';
    import Fa from 'svelte-fa';
</script>

{#if $selectedBucket}
    <div class="sidebar">
        <button class="nav-button">
            <Fa icon={faFile} />
        </button>

        <button class="nav-button active">
            <Fa icon={faGear} />
        </button>
    </div>
{/if}

<style lang="scss">
    .sidebar {
        grid-area: sidebar;

        height: 100%;
        width: fit-content;

        padding: 0px 16px 0px 16px;

        background-color: var(--background-secondary);

        display: flex;
        flex-direction: column;
        gap: 16px;

        .nav-button {
            $size: 50px;

            width: $size;
            height: $size;

            font-size: 1rem;

            display: grid;
            place-items: center;

            padding: 0;
        }
    }
</style>
