<script lang="ts">
	import {
		ArrowRight as IconArrowRight,
		FilePlus,
		FolderOpen,
		Save,
		Settings,
		CirclePause,
		CirclePlay,
		Ban,
		CircleChevronRight,
	} from "lucide-svelte";
</script>

<header
	class="bg-red-500 p-4 sticky top-0 right-0 flex justify-between items-center"
>
	<div class="flex items-center gap-4">
		<!-- A simple icon button -->
		<button type="button" class="btn preset-filled">
			<span>New</span>
			<FilePlus size={18} />
		</button>
		<button type="button" class="btn preset-filled">
			<span>Open</span>
			<FolderOpen size={18} />
		</button>
		<button type="button" class="btn preset-filled">
			<span>Save</span>
			<Save size={18} />
		</button>
		<button type="button" class="btn preset-filled">
			<span>Settings</span>
			<Settings size={18} />
		</button>
	</div>

	<nav
		class="btn-group preset-outlined-surface-200-800 flex-col p-2 md:flex-row"
	>
		<button type="button" class="btn preset-filled"
			>Intercept <CirclePause size={18} />
		</button>
		<button type="button" class="btn hover:preset-tonal"
			>Step <CircleChevronRight size={18} />
		</button>
		<button type="button" class="btn hover:preset-tonal"
			>Continue <CirclePlay size={18} />
		</button>
		<button type="button" class="btn hover:preset-tonal"
			>Drop <Ban size={18} />
		</button>
	</nav>
</header>
