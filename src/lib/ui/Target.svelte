<script lang="ts">
	import { Switch } from "@skeletonlabs/skeleton-svelte";
	let tableData: {
		url: string;
		isActive: boolean;
	}[] = [
		{ url: "https://Facebook.com", isActive: false },
		{ url: "https://google.com", isActive: true },
		{ url: "https://x.com", isActive: true },
	];
</script>

<div class="bg-yellow-500 w-full flex min-h-screen">
	<div class="table-wrap overflow-scroll w-1/2 p-4">
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
	</div>
	<div class="overflow-scroll w-1/2 bg-red-100"></div>
</div>
