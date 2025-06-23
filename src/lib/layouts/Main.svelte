<script lang="ts">
	import RequestResponse from "$lib/ui/RequestResponse.svelte";
	import Target from "$lib/ui/Target.svelte";
	let control = "requestresponse";
</script>

<main class="bg-green-500 w-full overflow-hidden">
	{#if control == "target"}
		<Target />
	{:else if control == "requestresponse"}
		<RequestResponse />
	{/if}
</main>
