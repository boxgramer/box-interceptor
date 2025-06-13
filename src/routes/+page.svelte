<script lang="ts">
  import Header from "$lib/layouts/Header.svelte";
  import Footer from "$lib/layouts/Footer.svelte";
  import Control from "$lib/layouts/Control.svelte";
  import Main from "$lib/layouts/Main.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    //   event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<div class="h-screen grid grid-rows-[auto_1fr_auto] relative">
  <!-- Header -->
  <Header />
  <Control />
  <!-- Main -->
  <Main />
  <!-- Footer -->
  <Footer />
</div>
