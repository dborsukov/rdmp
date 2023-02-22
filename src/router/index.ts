import { createRouter, createWebHistory } from 'vue-router';
import DashboardView from '@/views/DashboardView.vue';

const routes = [
  {
    path: '/',
    name: 'dashboard',
    component: DashboardView,
  },
  {
    path: '/roadmaps/:uuid',
    name: 'roadmap',
    component: () => import('../views/RoadmapView.vue'),
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: routes,
  linkActiveClass:
    'rounded-md bg-emerald-500/30 text-emerald-800 hover:bg-emerald-500/30 dark:bg-emerald-500/40 dark:text-violet-50 dark:hover:bg-emerald-500/40',
});

export default router;
