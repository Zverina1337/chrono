<script setup lang="ts">
import { routes } from "./router";
import { icons } from "@/shared/types/icons";
</script>

<template>
  <main class="flex w-full">
    <aside
      class="nav-w bg-surface-nav relative flex flex-col items-center"
    >
      <div
        class="text-ink-inverse bg-brand-500 shadow-focus font-500
          mt-2 mb-4 flex h-10 w-10 items-center justify-center
          rounded-md text-xl"
      >
        C
      </div>
      <nav
        class="flex flex-col items-center gap-2 rounded-lg border
          border-white/10 bg-white/5"
      >
        <RouterLink
          v-for="route in routes"
          :to="route.path"
          class="rail-item"
        >
          <span
            :class="icons[route.icon]"
            class="block h-1/2 w-1/2"
          />
        </RouterLink>
      </nav>
      <nav
        class="absolute bottom-4 flex flex-col items-center gap-2
          rounded-lg border border-white/10 bg-white/5"
      >
        <RouterLink to="/settings" class="rail-item">
          <span
            :class="icons['settings']"
            class="block h-1/2 w-1/2"
          />
        </RouterLink>
      </nav>
    </aside>
    <RouterView class="bg-surface-canvas w-full" />
  </main>
</template>
