<script lang="ts">
	import { Segment } from "@skeletonlabs/skeleton-svelte";

	let controls: string | null = $state("target");
	let type: string | null = $state("type");
</script>

<div
	class="bg-gray-500 p-4 sticky top-0 right-0 flex justify-between items-center"
>
	<Segment
		name="controls"
		value={controls}
		onValueChange={(e) => (controls = e.value)}
	>
		<Segment.Item value="target">Target</Segment.Item>
		<Segment.Item value="RR">Request & Response</Segment.Item>
		<Segment.Item value="editor">Request Editor</Segment.Item>
		<Segment.Item value="file">File Viewer</Segment.Item>
	</Segment>
	<div class="flex gap-5">
		<Segment name="type" value={type} onValueChange={(e) => (type = e.value)}>
			<Segment.Item value="Request">Request</Segment.Item>
			<Segment.Item value="Response">Response</Segment.Item>
		</Segment>
		<div
			class="btn-group preset-outlined-surface-200-800 flex-col p-2 md:flex-row"
		>
			<button type="button" class="btn hover:preset-filled">Send </button>
		</div>
	</div>
</div>
