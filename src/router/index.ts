import { createRouter, createWebHistory } from 'vue-router';
import RoadmapView from '@/views/RoadmapView.vue';

const routes = [
  {
    path: '/',
    name: 'home',
    component: RoadmapView,
  },
  {
    path: '/about',
    name: 'about',
    component: RoadmapView,
  },
  // {
  //   path: "/about",
  //   name: "about",
  //   component: () => import("../views/AboutView.vue"),
  // },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: routes,
  linkActiveClass:
    'rounded-md bg-emerald-500/30 text-emerald-800 hover:bg-emerald-500/30 dark:bg-emerald-500/40 dark:text-violet-50 dark:hover:bg-emerald-500/40',
});

export default router;
