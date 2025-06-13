<script lang="ts">
	import { Segment } from "@skeletonlabs/skeleton-svelte";

	let controls = $state("target");
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
	</Segment>
</div>
