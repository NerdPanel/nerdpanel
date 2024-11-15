<script lang="ts">
    import { EthernetPort, Server } from 'lucide-svelte';

    let { server } = $props();
</script>

<a href="/servers/{server.id}">
    <div class="my-4 rounded bg-cyan-700 p-6">
        <div class="ml-4 flex flex-row justify-between">
            <div class="flex flex-row space-x-4">
                <Server />
                <div class="font-semibold text-gray-700">{server.name}</div>
            </div>
            <div class="flex flex-row space-x-4">
                <EthernetPort />
                <div class="text-white-500 text-sm">
                    {server.primary_port.ip}:{server.primary_port.port}
                </div>
            </div>
        </div>
    </div>
</a>
