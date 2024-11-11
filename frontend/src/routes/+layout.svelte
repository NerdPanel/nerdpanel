<script lang="ts">
	import '../app.css';
	let { children } = $props();
</script>

<div class="container mx-10">
	{@render children()}
</div>
