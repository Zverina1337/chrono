import { createApp } from "vue";
import { createPinia } from "pinia";
import "@unocss/reset/tailwind.css";
import "virtual:uno.css";
import App from "@/app/App.vue";

const app = createApp(App);

app.use(createPinia());
app.mount("#app");
