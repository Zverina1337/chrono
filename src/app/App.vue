<script setup lang="ts">
import { routes } from "./router";
import { icons } from "@/shared/types/icons";
</script>

<template>
  <main flex="~" w="full">
    <aside
      w="70px"
      dark:color="dark-surface"
      bg="white-surface"
      flex="~ col"
      items="center"
      relative
    >
      <div
        flex="~"
        items="center"
        justify="center"
        text="white-base sm"
        font="600"
        bg="white-emerald"
        dark:bg="dark-emerald"
        rounded="xl"
        shadow-glow
        w="10"
        h="10"
        my="2"
      >
        C
      </div>
      <nav
        flex="~ col"
        items="center"
        gap="2"
        border="1px solid white-base/15"
        rounded="lg"
        bg="white-base/5"
        p-1.5
      >
        <RouterLink
          v-for="route in routes"
          :to="route.path"
          w="40px"
          h="40px"
          flex="~"
          justify="center"
          items="center"
          rounded="md"
          bg="white-base/25"
          hover:bg-white-emerald
          dark:hover:bg-dark-emerald
          transition-colors
        >
          <span
            :class="icons[route.icon]"
            h="1/2"
            w="1/2"
            style="display: block"
            text="white-base"
          />
        </RouterLink>
      </nav>
      <nav
        flex="~"
        items="center"
        justify="center"
        gap="2"
        border="1px solid white-base/15"
        rounded="lg"
        bg="white-base/5"
        p-1.5
        absolute
        bottom-4
      >
        <RouterLink
          to="/settings"
          w="40px"
          h="40px"
          flex="~"
          justify="center"
          items="center"
          rounded="md"
          bg="white-base/25"
          hover:bg-white-emerald
          dark:hover:bg-dark-emerald
          transition-colors
        >
          <span
            :class="icons['settings']"
            h="1/2"
            w="1/2"
            style="display: block"
            text="white-base"
          />
        </RouterLink>
      </nav>
    </aside>
    <RouterView w="full" dark:bg="dark-base" dark:text="zinc-100" bg="white-base" />
  </main>
</template>
