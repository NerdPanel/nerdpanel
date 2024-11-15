<script lang="ts">
    import type { Snippet } from 'svelte';

    let { children, title, text }: { children: Snippet<[]>; title: String; text: String } =
        $props();
</script>

<div class="my-2 flex flex-row items-center space-x-4 rounded bg-slate-800 p-4">
    <div class="rounded-lg bg-slate-600 p-4">{@render children()}</div>
    <div>
        <div class="font-semibold text-slate-500">{title}</div>
        <div class="font-semibold text-slate-500">
            {text}
        </div>
    </div>
</div>
