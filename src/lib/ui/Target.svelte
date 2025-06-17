<script lang="ts" context="module">
	export type TreeNode = {
		label: string;
		children?: TreeNode[] | null;
	};
</script>

<script lang="ts">
	import { Switch } from "@skeletonlabs/skeleton-svelte";
	import TreeView from "$lib/component/TreeView.svelte";
	export let tableData: {
		url: string;
		isActive: boolean;
	}[] = [
		{ url: "https://Facebook.com", isActive: false },
		{ url: "https://google.com", isActive: true },
		{ url: "https://x.com", isActive: true },
	];

	export let tree: TreeNode = {
		label: "USA",
		children: [
			{
				label: "Florida",
				children: [
					{ label: "Jacksonville" },
					{
						label: "Orlando",
						children: [
							{ label: "Disney World" },
							{ label: "Universal Studio" },
							{ label: "Sea World" },
						],
					},
					{ label: "Miami" },
				],
			},
			{
				label: "California",
				children: [
					{ label: "San Francisco" },
					{ label: "Los Angeles" },
					{ label: "Sacramento" },
				],
			},
		],
	};

	const min_width = 350;
	const max_width = window.innerWidth;
	const threshold_max = max_width - min_width;
	let widthUrl: number = min_width; //use percent
	let widthTree: number = max_width - min_width;

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

		console.log(max_width, min_width, widthUrl, dx, event.pageX);
	}
</script>

<svelte:window on:mouseup={stopExpand} on:mousemove={expand} />

<div class="bg-yellow-500 w-full flex min-h-screen">
	<div
		class=" relative table-wrap overflow-y-scroll p-4"
		style="width: {widthUrl}px;"
	>
		<table class="table caption-bottom w-full">
			<thead>
				<tr>
					<th>#</th>
					<th>Url</th>
					<th style="text-align: right;">Target Active</th>
				</tr>
			</thead>
			<tbody class="[&>tr]:hover:preset-tonal-primary">
				{#each tableData as data, key}
					<tr>
						<td>{key + 1}</td>
						<td>{data.url}</td>
						<td class="flex justify-end"
							><Switch
								name="example"
								checked={data.isActive}
								onCheckedChange={(e) => (tableData[key].isActive = e.checked)}
							/></td
						>
					</tr>
				{/each}
			</tbody>
		</table>
		<!-- Top grabber -->
		<div
			class="absolute top-0 right-0 w-[10px] h-full bg-green-200 cursor-ew-resize hover:bg-blue-300/30"
			on:mousedown={startExpand.bind(this)}
		></div>
	</div>
	<div class="overflow-scroll bg-red-100" style="width: {widthTree}px;">
		<TreeView {tree} />
	</div>
</div>
