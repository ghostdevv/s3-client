<script lang="ts">
    import { Toy } from '@leveluptuts/svelte-toy';

    import {
        buckets,
        selectedBucket,
        selectedBucketId,
        selectedFile,
    } from '$lib/state';
</script>

<Toy register={{ selectedFile, selectedBucket, buckets, selectedBucketId }} />
