<script lang="ts">
    import { EthernetPort, Server } from 'lucide-svelte';

    let { server }: { server: any } = $props();
</script>

<a href="/servers/{server.id}">
    <div class="my-4 rounded bg-slate-800 p-6">
        <div class="ml-4 flex flex-row justify-between">
            <div class="flex flex-row space-x-4">
                <Server />
                <div class="font-semibold text-gray-300">{server.name}</div>
            </div>
            <div class="flex flex-row space-x-4 items-center">
                <EthernetPort class="text-gray-300" />
                <div class="font-semibold text-gray-300 text-sm">
                    {server.primary_port.ip}:{server.primary_port.port}
                </div>
            </div>
        </div>
    </div>
</a>
