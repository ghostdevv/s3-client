<script lang="ts">
    import type { IconDefinition } from '@fortawesome/fontawesome-common-types';
    import { faFile, faGear } from '@fortawesome/free-solid-svg-icons';
    import { selectedBucket, selectedBucketId } from '$lib/state';
    import Settings from './options/settings/Settings.svelte';
    import Files from './options/files/Files.svelte';
    import type { SvelteComponent } from 'svelte';
    import Fa from 'svelte-fa';

    type Manfiest<Keys extends string> = {
        [P in Keys]: {
            icon: IconDefinition;
            component: typeof SvelteComponent;
        };
    };

    const manifest: Manfiest<'files' | 'settings'> = {
        files: {
            icon: faFile,
            component: Files,
        },
        settings: {
            icon: faGear,
            component: Settings,
        },
    };

    let selected: keyof typeof manifest | null = null;

    // Close the sidebar when the user changes their bucket selection to no buckets
    $: if ($selectedBucketId) {
        selected = $selectedBucketId ? 'files' : null;
    }

    const toggle = (item: string) =>
        (selected = selected == item ? null : (item as keyof typeof manifest));
</script>

{#if $selectedBucket}
    <div class="sidebar">
        <div class="buttons">
            {#each Object.entries(manifest) as [key, { icon }]}
                <button
                    class:active={selected == key}
                    on:click={() => toggle(key)}
                    class="nav-button">
                    <Fa {icon} />
                </button>
            {/each}
        </div>

        {#if selected}
            <div class="content">
                <svelte:component this={manifest[selected].component} />
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
            padding: 0px 16px 0px 16px;
            height: 100%;

            width: 250px;
            max-width: 250px;
            max-height: 100%;
            overflow: auto;
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
