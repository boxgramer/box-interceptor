<script lang="ts">
	export let height: number = 150;

	let isExpanding: boolean = false;
	let initalHeight: number = height;
	let start: { x: number; y: number } | null = null;

	function startExpand(event: MouseEvent) {
		isExpanding = true;
		start = { x: event.pageX, y: event.pageY };
		initalHeight = height;
		event.preventDefault();
	}

	function stopExpand() {
		isExpanding = false;
		start = null;
	}

	function expand(event: MouseEvent) {
		if (!isExpanding || !start) return;
		const dy = event.pageY - start.y;
		height = initalHeight - dy;
	}
</script>

<svelte:window on:mouseup={stopExpand} on:mousemove={expand} />

<footer
	class="relative bg-blue-100 border border-gray-400 overflow-hidden w-full"
	style="height: {height}px;"
>
	<div class="p-4 text-center h-full select-none">Resizable Content</div>

	<!-- Top grabber -->
	<div
		class="absolute top-0 left-0 w-full h-[10px] bg-green-200 cursor-ns-resize hover:bg-blue-300/30"
		on:mousedown={startExpand.bind(this)}
	></div>
</footer>
