<script lang="ts">
    import { Button } from '$lib/components/ui/button';
    import { Clock, CloudDownload, CloudUpload, Cpu, EthernetPort, HardDrive, MemoryStick } from 'lucide-svelte';
    import type { PageData } from './$types';
    import ServerInfoCard from '$lib/components/server_info_card/server_info_card.svelte';

    let { data }: { data: PageData } = $props();
</script>

<div class="grid grid-cols-5 gap-x-4">
    <div class="col-span-4">
        <h1 class="font-bold">{data.server.name}</h1>
    </div>
    <div class="flex flex-1 flex-row justify-center space-x-4">
        <Button class="flex-1">Stop</Button>
        <Button class="flex-1">Restart</Button>
        <Button class="flex-1" variant="destructive">Kill</Button>
    </div>
    <div class="my-2 col-span-4 rounded bg-white p-4">hi</div>
    <div>
        <ServerInfoCard title="Address" text={`${data.server.primary_port.ip}:${data.server.primary_port.port}`}>
            <EthernetPort />
        </ServerInfoCard>
        <ServerInfoCard title="Uptime" text={"server.stats.uptime"}>
            <Clock />
        </ServerInfoCard>
        <ServerInfoCard title="CPU" text={"server.stats.cpu"}>
            <Cpu />
        </ServerInfoCard>
        <ServerInfoCard title="Memory" text={"server.stats.memory"}>
            <MemoryStick />
        </ServerInfoCard>
        <ServerInfoCard title="Disk" text={"server.stats.disk"}>
            <HardDrive />
        </ServerInfoCard>
        <ServerInfoCard title="Network (Down)" text={"server.stats.down"}>
            <CloudDownload />
        </ServerInfoCard>
        <ServerInfoCard title="Network (Up)" text={"server.stats.up"}>
            <CloudUpload />
        </ServerInfoCard>
    </div>
</div>
