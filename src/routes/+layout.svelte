<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { isTauri } from "@tauri-apps/api/core";
  import { writable } from "svelte/store";

  const ready = writable(false);

  // Checks if it's tauri, if so, redirects to /play
  // There HAS to be a better way to do this but I can't find it
  onMount(async () => {
    try {
      const tauri = isTauri();
      const path = window.location.pathname;
      if (tauri && path !== "/play") {
        await goto("/play");
      }
      ready.set(true);
    } catch (e) {
      console.error("Tauri detection failed:", e);
      ready.set(true);
    }
  });
</script>

{#if $ready}
  <slot />
{:else}
  <div style="display: none;"></div>
{/if}
