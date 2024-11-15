<script lang="ts">
    import type { Snippet } from "svelte";

    let { children, title, text }: { children: Snippet<[]>, title: String, text: String } = $props();
</script>

<div class="my-2 flex flex-row items-center space-x-4 rounded bg-slate-700 p-4">
    <div class="p-4 bg-slate-600 rounded-lg">{@render children()}</div>
    <div>
        <div>{title}</div>
        <div class="font-semibold text-white">
            {text}
        </div>
    </div>
</div>
