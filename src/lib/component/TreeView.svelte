<script lang="ts" context="module">
	export const expansionState: Record<string, boolean> = {};
</script>

<script lang="ts">
	import { slide } from "svelte/transition";
	import { Folder, FolderOpen, File } from "lucide-svelte";
	import type { TreeNode } from "$lib/ui/Target.svelte";

	export let tree: TreeNode;

	const { label, children } = tree;
	const isFolder = !!children && children.length > 0;

	let expanded = expansionState[label] || false;

	function toggleExpansion() {
		if (isFolder) {
			expanded = expansionState[label] = !expanded;
		}
	}

	$: iconComponent = isFolder ? (expanded ? FolderOpen : Folder) : File;

	let showDropdown = false;

	function onFileClick(e: MouseEvent) {
		e.stopPropagation(); // prevent toggling folder accidentally
		if (!isFolder) {
			showDropdown = !showDropdown;
		}
	}

	function closeDropdown() {
		showDropdown = false;
	}

	// optional: close dropdown when clicking outside
	function onOutsideClick(e: MouseEvent) {
		if (!e.composedPath().includes(dropdownEl)) {
			closeDropdown();
		}
	}
	let dropdownEl: HTMLDivElement;
</script>

<svelte:window on:click={onOutsideClick} />

<ul class="pl-2 border-l border-gray-400 ml-2">
	<li class="relative">
		<div
			class="flex items-center gap-1 cursor-pointer select-none hover:text-blue-600 relative"
			on:click={isFolder ? toggleExpansion : onFileClick}
		>
			<div class="absolute -left-2 top-1/2 w-2 h-px bg-gray-400" />
			<svelte:component this={iconComponent} class="w-4 h-4 shrink-0" />
			<span>{label}</span>
		</div>

		<!-- Dropdown menu -->
		{#if showDropdown && !isFolder}
			<div
				bind:this={dropdownEl}
				class="ml-5 mt-1 p-2 bg-white border border-gray-300 rounded shadow-md w-32 text-sm z-10"
			>
				<ul class="flex flex-col space-y-1">
					<li class="hover:bg-gray-100 px-2 py-1 cursor-pointer">
						<a href="#"> Open File </a>
					</li>
				</ul>
			</div>
		{/if}

		{#if isFolder && expanded}
			<ul role="group" transition:slide>
				{#each children as child}
					<svelte:self tree={child} />
				{/each}
			</ul>
		{/if}
	</li>
</ul>
