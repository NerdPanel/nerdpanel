<script lang="ts">
    import '../app.css';
    let { children, data } = $props();
    function logout() {
        fetch('/api/auth/logout').then(() => {
            window.location.href = '/';
        });
    }
</script>

<div class="flex min-h-screen flex-col bg-background">
    <nav class="flex items-center justify-between bg-slate-900 p-4 text-white">
        <div class="logo">
            <a href="/">Logo</a>
        </div>
        <ul class="flex space-x-4">
            <li><a href="/">Home</a></li>
            <li><a href="/servers">Servers</a></li>
            {#if !data.user}
                <li><a href="/login">Login</a></li>
            {:else}
                <li><button type="button" onclick={logout}>Logout</button></li>
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
