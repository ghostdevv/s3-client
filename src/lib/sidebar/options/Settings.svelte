<script lang="ts">
    import { selectedBucket } from '$lib/state';

    $: bucket = $selectedBucket;
</script>

<form>
    <label>
        Name

        <input value={$selectedBucket.name} />
    </label>

    <label>
        Bucket

        <input value={$selectedBucket.bucket} />
    </label>

    <label>
        Endpoint

        <input value={$selectedBucket.endpoint} />
    </label>

    <label>
        Region

        <input value={$selectedBucket.region} />
    </label>

    <label>
        Access Key ID

        <input value={$selectedBucket.key_id} />
    </label>

    <label>
        Access Key

        <input value={$selectedBucket.key} />
    </label>
</form>

<style lang="scss">
    form {
        display: flex;
        flex-direction: column;
        align-items: stretch;
        gap: 12px;
    }

    input {
        margin-top: 8px;
    }
</style>
