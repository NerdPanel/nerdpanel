<script lang="ts">
	import '../app.css';
	let { children } = $props();
</script>
<div class="bg-cyan-800 min-h-screen">
	<nav class="flex items-center justify-between bg-primary p-4 text-white">
		<div class="logo">
			<a href="/">Logo</a>
		</div>
		<ul class="flex space-x-4">
			<li><a href="/home">Home</a></li>
			<li><a href="/servers">Servers</a></li>
			<li><a href="/settings">Settings </a></li>
		</ul>
	</nav>
	<div class="container mx-auto my-6">
		{@render children()}
	</div>
</div>
