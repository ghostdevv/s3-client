<script lang="ts">
    import { faPlus, faArchive } from '@fortawesome/free-solid-svg-icons';
    import Fa from 'svelte-fa';
</script>

<div class="topbar">
    <!-- <button class="nav-button">
        <Fa icon={faPlus} />
    </button> -->

    <button class="nav-button">
        <Fa icon={faArchive} />
        <p>My Bucket</p>
    </button>

    <button class="nav-button active">
        <Fa icon={faArchive} />
        <p>My Bucket</p>
    </button>

    <button class="nav-button">
        <Fa icon={faArchive} />
        <p>My Bucket</p>
    </button>
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
        border-bottom: 2px solid var(--background-tertiary);

        .nav-button {
            border-radius: 0px;
            border: 0px;

            box-shadow: none;

            &.active {
                border-bottom: 2px solid var(--primary);
            }
        }
    }
</style>
