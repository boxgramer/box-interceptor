<script lang="ts">
	import { onMount } from "svelte";

	const min_width = 350;

	let max_width = 0;
	let threshold_max = 0;
	let widthUrl = 0;
	let widthTree = 0;

	onMount(() => {
		max_width = window.innerWidth;
		threshold_max = max_width - min_width;
		widthUrl = max_width / 2;
		widthTree = max_width - widthUrl;
	});

	let isExpanding: boolean = false;
	let initalWidth: number = widthUrl;
	let start: { x: number; y: number } | null = null;

	function startExpand(event: MouseEvent) {
		isExpanding = true;
		start = { x: event.pageX, y: event.pageY };
		initalWidth = widthUrl;
		console.log("start:", start.x);
		event.preventDefault();
	}

	function stopExpand() {
		isExpanding = false;
		start = null;
	}

	function expand(event: MouseEvent) {
		if (!isExpanding || !start) return;
		const dx = start.x - event.pageX;
		widthUrl = initalWidth - dx;
		if (widthUrl < min_width) {
			widthUrl = min_width;
		}
		if (widthUrl > threshold_max) {
			widthUrl = threshold_max;
		}

		widthTree = max_width - widthUrl;
	}
</script>

<svelte:window on:mouseup={stopExpand} on:mousemove={expand} />

<div class="bg-yellow-500 w-full flex min-h-screen">
	<div
		class=" relative table-wrap overflow-y-scroll p-1 pr-4"
		style="width: {widthUrl}px;"
	>
		<div class="sticky top-0 left-0 w-full flex gap-3">
			<select class="select">
				<option value="1">Method</option>
				<option value="2">CONNECT</option>
				<option value="3">DELETE</option>
				<option value="4">GET</option>
				<option value="5">HEAD</option>
				<option value="6">OPTIONS</option>
				<option value="7">PATH</option>
				<option value="8">POST</option>
				<option value="9">PUT</option>
				<option value="10">TRACE</option>
				<option value="11">TRACK</option>
			</select>
			<select class="select">
				<option value="1">Header</option>
				<option value="2">Text</option>
				<option value="4">HEX</option>
				<option value="5">Table(adv)</option>
			</select>
			<select class="select">
				<option value="1">Body</option>
				<option value="2">Text</option>
				<option value="3">JSON</option>
				<option value="4">HEX</option>
				<option value="5">Table(adv)</option>
				<option value="5">Table</option>
			</select>
			<select class="select">
				<option value="1">Redirect</option>
				<option value="2">Unfollow</option>
				<option value="3">Follow</option>
			</select>
		</div>
		<!-- Left grabber -->
		<div
			class="absolute top-0 right-0 w-[10px] h-full bg-green-200 cursor-ew-resize hover:bg-blue-300/30"
			on:mousedown={startExpand.bind(this)}
		></div>
	</div>
	<div class="overflow-scroll bg-red-100 p-1" style="width: {widthTree}px;">
		<div class=" sticky top-0 left-0 w-full flex gap-3">
			<select class="select">
				<option value="1">Header</option>
				<option value="2">Text</option>
				<option value="4">HEX</option>
				<option value="5">Table(adv)</option>
			</select>
			<select class="select">
				<option value="1">Body</option>
				<option value="2">Text</option>
				<option value="3">JSON</option>
				<option value="4">HEX</option>
				<option value="5">Table(adv)</option>
				<option value="5">Table</option>
			</select>
		</div>
	</div>
</div>
