<script lang="ts">
    import { faFile, faGear } from '@fortawesome/free-solid-svg-icons';
    import { selectedBucket } from '$lib/buckets/buckets';
    import Settings from './options/Settings.svelte';
    import Files from './options/Files.svelte';
    import Fa from 'svelte-fa';

    const manifest = {
        files: Files,
        settings: Settings,
    };

    let selected: keyof typeof manifest | null;

    const toggle = (item: keyof typeof manifest) =>
        (selected = selected == item ? null : item);
</script>

{#if $selectedBucket}
    <div class="sidebar">
        <div class="buttons">
            <button
                class:active={selected == 'files'}
                on:click={() => toggle('files')}
                class="nav-button">
                <Fa icon={faFile} />
            </button>

            <button
                class:active={selected == 'settings'}
                on:click={() => toggle('settings')}
                class="nav-button">
                <Fa icon={faGear} />
            </button>
        </div>

        {#if selected}
            <div class="content">
                <svelte:component this={manifest[selected]} />
            </div>
        {/if}
    </div>
{/if}

<style lang="scss">
    .sidebar {
        grid-area: sidebar;

        height: 100%;
        width: fit-content;

        background-color: var(--background-secondary);

        display: flex;
        flex-flow: row nowrap;
        gap: 0px;

        .buttons {
            display: flex;
            flex-direction: column;
            gap: 16px;

            padding: 0px 16px 0px 16px;
        }

        .content {
            padding-right: 16px;
            height: 100%;
        }

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
