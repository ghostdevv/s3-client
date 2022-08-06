<script lang="ts">
    import { buckets, selectedBucketId } from '$lib/buckets/buckets';
    import { invoke } from '@tauri-apps/api/tauri';
    import { Bucket } from '$lib/buckets/types';
    import { Modal } from 'polykit';

    let open = false;
    let disabled = false;

    async function submit(form: HTMLFormElement) {
        disabled = true;

        const data = new FormData(form);

        const bucket: Omit<Bucket, 'id'> = {
            name: data.get('name') as string,
            bucket: data.get('bucket') as string,
            endpoint: data.get('endpoint') as string,
            region: data.get('region') as string,
            key_id: data.get('key_id') as string,
            key: data.get('key') as string,
        };

        const id = await invoke<number>('create_bucket', { bucket });

        $buckets = [...$buckets, { ...bucket, id }];
        $selectedBucketId = id;

        disabled = false;
        open = false;
    }
</script>

<Modal bind:open clickOutside={!disabled} escape={!disabled}>
    <slot name="activator" slot="activator" />

    <div class="modal card no-hover">
        <h2>Add Bucket</h2>

        <form on:submit|preventDefault={(e) => submit(e.currentTarget)}>
            <label>
                Name *

                <input
                    required
                    name="name"
                    placeholder="What you want the bucket to be called e.g. Media" />
            </label>

            <label>
                Bucket *

                <input
                    required
                    name="bucket"
                    placeholder="Name as appears on S3" />
            </label>

            <label>
                Endpoint *
                <input required name="endpoint" placeholder="S3 Endpoint" />
            </label>

            <label>
                Region
                <input name="region" placeholder="Defaults to auto" />
            </label>

            <label>
                Access Key ID *
                <input required name="key_id" placeholder="Access Key ID" />
            </label>

            <label>
                Access Key *
                <input required name="key" placeholder="Access Key" />
            </label>

            <button> Create </button>
        </form>
    </div>
</Modal>

<style lang="scss">
    .modal {
        background-color: var(--background-secondary);

        max-height: 80vh;
        overflow: auto;

        form {
            display: flex;
            flex-direction: column;
            align-items: stretch;
            gap: 22px;

            width: 100%;
            margin-top: 8px;

            input {
                margin-top: 8px;
            }
        }
    }
</style>
