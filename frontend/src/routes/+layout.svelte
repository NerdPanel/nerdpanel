<script lang="ts">
    import '../app.css';
    let { children, data } = $props();
</script>

<div class="flex min-h-screen flex-col bg-cyan-800">
    <nav class="flex items-center justify-between bg-primary p-4 text-white">
        <div class="logo">
            <a href="/">Logo</a>
        </div>
        <ul class="flex space-x-4">
            <li><a href="/home">Home</a></li>
            <li><a href="/servers">Servers</a></li>
            {#if !data.user}
                <li><a href="/login">Login</a></li>
            {:else}
                <li><a href="/logout">Logout</a></li>
            {/if}
            {#if data.user?.staff}
                <li><a href="/admin">Admin</a></li>
            {/if}
        </ul>
    </nav>
    <div class="container mx-auto my-6 flex flex-1 flex-col">
        {@render children()}
    </div>
</div>
