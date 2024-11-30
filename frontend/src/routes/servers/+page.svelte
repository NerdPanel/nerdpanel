<script lang="ts">
    import ServerCard from '$lib/components/server_card/server_card.svelte';
    import type { PageData } from './$types';
    let { data }: { data: PageData } = $props();
</script>

<div>
    {#each data.servers as server}
        <ServerCard {server} />
    {/each}
</div>
