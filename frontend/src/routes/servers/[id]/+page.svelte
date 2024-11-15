<script lang="ts">
    import { Button } from '$lib/components/ui/button';
    import {
        Clock,
        CloudDownload,
        CloudUpload,
        Cpu,
        EthernetPort,
        HardDrive,
        MemoryStick,
        SquareChevronRight
    } from 'lucide-svelte';
    import type { PageData } from './$types';
    import ServerInfoCard from '$lib/components/server_info_card/server_info_card.svelte';
    import Input from '$lib/components/ui/input/input.svelte';

    let { data }: { data: PageData } = $props();
</script>

<div class="grid grid-cols-5 gap-x-4">
    <div class="col-span-4">
        <h1 class="text-xl font-bold text-slate-500">{data.server.name}</h1>
    </div>
    <div class="flex flex-1 flex-row justify-center space-x-4">
        <Button class="flex-1">Stop</Button>
        <Button class="flex-1">Restart</Button>
        <Button class="flex-1" variant="destructive">Kill</Button>
    </div>
    <div class="col-span-4 my-2 flex flex-col space-y-2">
        <div class="grow rounded bg-slate-800 p-4">CONSOLE</div>
        <div class="flex h-8 w-full flex-row space-x-2 rounded bg-slate-800 p-1 text-white">
            <SquareChevronRight class="text-gray-950" />
            <input class="w-full bg-transparent outline-none" />
        </div>
    </div>
    <div>
        <ServerInfoCard
            title="Address"
            text={`${data.server.primary_port.ip}:${data.server.primary_port.port}`}
        >
            <EthernetPort />
        </ServerInfoCard>
        <ServerInfoCard title="Uptime" text={'server.stats.uptime'}>
            <Clock />
        </ServerInfoCard>
        <ServerInfoCard title="CPU" text={'server.stats.cpu'}>
            <Cpu />
        </ServerInfoCard>
        <ServerInfoCard title="Memory" text={'server.stats.memory'}>
            <MemoryStick />
        </ServerInfoCard>
        <ServerInfoCard title="Disk" text={'server.stats.disk'}>
            <HardDrive />
        </ServerInfoCard>
        <ServerInfoCard title="Network (Down)" text={'server.stats.down'}>
            <CloudDownload />
        </ServerInfoCard>
        <ServerInfoCard title="Network (Up)" text={'server.stats.up'}>
            <CloudUpload />
        </ServerInfoCard>
    </div>
</div>
