<script lang="ts">
    let { children, title, text } = $props();
</script>

<div class="my-2 flex flex-row items-center space-x-4 rounded bg-slate-700 p-4">
    {@render children()}
    <div>
        <div>{title}</div>
        <div class="font-semibold text-white">
            {text}
        </div>
    </div>
</div>