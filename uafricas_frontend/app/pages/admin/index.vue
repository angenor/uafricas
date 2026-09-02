<script setup lang="ts">
import type { StatsCardData } from '~/types/admin'
import type { ChartSegment } from '~/components/admin/AdminChart.vue'

definePageMeta({
  layout: 'admin',
  middleware: ['admin'],
})

const { stats, activiteRecente, tendances, loading, chargerStats, chargerActiviteRecente, chargerTendances } = useAdminDashboard()

const periodeSelectionnee = ref('7j')

// Charger toutes les donnees au montage
onMounted(async () => {
  await Promise.all([
    chargerStats(),
    chargerActiviteRecente(),
    chargerTendances(periodeSelectionnee.value),
  ])
})

// Recharger les tendances quand la periode change
watch(periodeSelectionnee, (val) => chargerTendances(val))

// ── KPIs principaux ─────────────────────────────────────────
const kpis = computed<StatsCardData[]>(() => {
  if (!stats.value) return []
  const s = stats.value
  return [
    {
      label: 'Utilisateurs actifs',
      value: s.utilisateurs.actifs,
      icon: 'users',
      color: 'bg-primary/10 text-primary',
    },
    {
      label: 'Annonces publiées',
      value: s.annonces.publiees,
      icon: 'bullhorn',
      color: 'bg-success/10 text-success',
    },
    {
      label: 'Événements a venir',
      value: s.evenements.a_venir,
      icon: 'calendar',
      color: 'bg-info/10 text-info',
    },
    {
      label: 'Projets en revue',
      value: s.projets.en_revue,
      icon: 'diagram-project',
      color: 'bg-warning/10 text-warning',
    },
    {
      label: 'Candidatures en attente',
      value: s.programmes.candidatures_en_attente,
      icon: 'graduation-cap',
      color: 'bg-accent/10 text-accent',
    },
    {
      label: 'Contributions territoires',
      value: s.fiches_pays.contributions_en_attente,
      icon: 'earth-africa',
      color: 'bg-secondary/10 text-secondary',
    },
    {
      label: 'Sessions AfroLang',
      value: s.sessions_afrolang.en_cours,
      icon: 'language',
      color: 'bg-error/10 text-error',
    },
    {
      label: 'MOOC en cours',
      value: s.moocs.en_cours,
      icon: 'graduation-cap',
      color: 'bg-neutral/10 text-neutral',
    },
  ]
})

// ── Graphiques ──────────────────────────────────────────────
const segmentsAnnonces = computed<ChartSegment[]>(() => {
  if (!stats.value) return []
  const a = stats.value.annonces
  return [
    { label: 'Publiées', valeur: a.publiees, couleur: 'bg-success' },
    { label: 'En attente', valeur: a.en_attente, couleur: 'bg-warning' },
    { label: 'Expirées', valeur: a.expirees, couleur: 'bg-error' },
  ]
})

const segmentsFactchecks = computed<ChartSegment[]>(() => {
  if (!stats.value) return []
  const v = stats.value.factchecks.par_verdict
  return [
    { label: 'Vrai', valeur: v.vrai, couleur: 'text-success' },
    { label: 'Faux', valeur: v.faux, couleur: 'text-error' },
    { label: 'Partiellement vrai', valeur: v.partiellement_vrai, couleur: 'text-warning' },
    { label: 'Trompeur', valeur: v.trompeur, couleur: 'text-info' },
    { label: 'Non vérifié', valeur: v.non_verifie, couleur: 'text-neutral' },
  ]
})

const segmentsBadHabits = computed<ChartSegment[]>(() => {
  if (!stats.value) return []
  const g = stats.value.bad_habits.par_gravite
  return [
    { label: 'Faible', valeur: g.faible, couleur: 'bg-success' },
    { label: 'Elevee', valeur: g.elevee, couleur: 'bg-warning' },
    { label: 'Critique', valeur: g.critique, couleur: 'bg-error' },
  ]
})

// ── Courbe inscriptions (barres verticales CSS) ─────────────
const maxInscription = computed(() => {
  if (!tendances.value) return 1
  return Math.max(...tendances.value.inscriptions_utilisateurs.map(p => p.total), 1)
})

const formatJour = (dateStr: string) => {
  const d = new Date(dateStr)
  return d.toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit' })
}

// ── Modules overview (totaux) ───────────────────────────────
const modulesOverview = computed(() => {
  if (!stats.value) return []
  const s = stats.value
  return [
    { label: 'Utilisateurs', total: s.utilisateurs.total, icon: 'users', lien: '/admin/utilisateurs' },
    { label: 'Organisations', total: s.organisations.total, icon: 'building', lien: '/admin/organisations' },
    { label: 'Annonces', total: s.annonces.total, icon: 'bullhorn', lien: '/admin/annonces' },
    { label: 'Programmes', total: s.programmes.total, icon: 'plane', lien: '/admin/programmes' },
    { label: 'Innovations', total: s.innovations.total, icon: 'lightbulb', lien: '/admin/innovations' },
    { label: 'Projets', total: s.projets.total, icon: 'diagram-project', lien: '/admin/projets' },
    { label: 'Africantives', total: s.africantives.total, icon: 'rocket', lien: '/admin/africantives' },
    { label: 'Centres culturels', total: s.centres_culturels.total, icon: 'landmark', lien: '/admin/centres-culturels' },
    { label: 'Codi-Moi', total: s.codimoi.total, icon: 'quote-left', lien: '/admin/codimoi' },
    { label: 'Événements', total: s.evenements.total, icon: 'calendar', lien: '/admin/evenements' },
    { label: 'MOOC', total: s.moocs.total, icon: 'graduation-cap', lien: '/admin/mooc' },
    { label: 'Livres', total: s.livres.total, icon: 'book', lien: '/admin/livres' },
    { label: 'Radio', total: s.radio_tv.stations_radio, icon: 'radio', lien: '/admin/radio' },
    { label: 'TV', total: s.radio_tv.chaines_tv, icon: 'tv', lien: '/admin/television' },
    { label: 'Factchecks', total: s.factchecks.total, icon: 'scale-balanced', lien: '/admin/factcheck' },
    { label: 'Fiches territoires', total: s.fiches_pays.total, icon: 'earth-africa', lien: '/admin/profils-pays' },
  ]
})
</script>

<template>
  <div>
    <!-- En-tete -->
    <div class="mb-8 flex items-center justify-between flex-wrap gap-2">
      <div>
        <h1 class="text-2xl font-display font-bold text-base-content">Dashboard</h1>
        <p class="text-sm text-base-content/60 mt-1">Vue d'ensemble de la plateforme AfricanS</p>
      </div>
      <div v-if="stats" class="text-xs text-base-content/40 flex items-center gap-2">
        <font-awesome-icon icon="clock-rotate-left" />
        <span>{{ stats.audit.actions_aujourd_hui }} actions aujourd'hui</span>
        <span class="text-base-content/20">|</span>
        <span>{{ stats.audit.actions_cette_semaine }} cette semaine</span>
      </div>
    </div>

    <!-- Loading state -->
    <div v-if="loading && !stats" class="flex items-center justify-center py-24">
      <font-awesome-icon icon="spinner" class="w-8 h-8 animate-spin text-custom-chocolat/40" />
    </div>

    <template v-else-if="stats">
      <!-- Section 1 : KPIs principaux -->
      <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-4 mb-8">
        <AdminStatsCard v-for="kpi in kpis" :key="kpi.label" :stat="kpi" />
      </div>

      <!-- Section 2 : Graphiques -->
      <div class="mb-8">
        <h2 class="text-lg font-display font-semibold text-base-content mb-4">Tendances & Répartition</h2>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-4">
          <!-- Courbe inscriptions utilisateurs -->
          <div class="card bg-base-100 border border-base-200 shadow-sm">
            <div class="card-body p-5">
              <div class="flex items-center justify-between mb-3">
                <h3 class="text-sm font-display font-semibold">Inscriptions utilisateurs</h3>
                <select
                  v-model="periodeSelectionnee"
                  class="select select-xs select-bordered"
                >
                  <option value="7j">7 jours</option>
                  <option value="30j">30 jours</option>
                  <option value="90j">90 jours</option>
                </select>
              </div>
              <div v-if="tendances" class="flex items-end gap-px h-32 mt-2">
                <div
                  v-for="(point, i) in tendances.inscriptions_utilisateurs"
                  :key="i"
                  class="flex-1 flex flex-col items-center justify-end group relative"
                >
                  <div
                    class="w-full bg-primary/80 rounded-t transition-all duration-300 hover:bg-primary min-h-[2px]"
                    :style="{ height: `${Math.max((point.total / maxInscription) * 100, point.total > 0 ? 5 : 2)}%` }"
                  />
                  <div class="absolute -top-6 bg-base-300 text-xs px-1.5 py-0.5 rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none z-10">
                    {{ point.total }} - {{ formatJour(point.jour) }}
                  </div>
                </div>
              </div>
              <div v-if="tendances && periodeSelectionnee === '7j'" class="flex justify-between text-[10px] text-base-content/40 mt-1">
                <span v-for="(point, i) in tendances.inscriptions_utilisateurs" :key="i">
                  {{ formatJour(point.jour) }}
                </span>
              </div>
            </div>
          </div>

          <!-- Barres annonces par etat -->
          <AdminChart
            titre="Annonces par état"
            type="barres"
            :segments="segmentsAnnonces"
          />
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <!-- Donut factchecks -->
          <AdminChart
            titre="Factchecks par verdict"
            type="donut"
            :segments="segmentsFactchecks"
          />
          <!-- Barres mauvaises pratiques -->
          <AdminChart
            titre="Mauvaises pratiques par gravité"
            type="barres"
            :segments="segmentsBadHabits"
          />
        </div>
      </div>

      <!-- Section 3 & 4 : Timeline + Actions rapides -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
        <AdminActivityTimeline :items="activiteRecente" />
        <AdminQuickActions :stats="stats" />
      </div>

      <!-- Section 5 : Vue d'ensemble des modules -->
      <div class="mb-8">
        <h2 class="text-lg font-display font-semibold text-base-content mb-4">Tous les modules</h2>
        <div class="grid grid-cols-2 sm:grid-cols-4 xl:grid-cols-8 gap-3">
          <NuxtLink
            v-for="mod in modulesOverview"
            :key="mod.label"
            :to="mod.lien"
            class="flex flex-col items-center gap-1 p-3 rounded-xl border border-base-200 bg-base-100 hover:bg-base-200/50 hover:border-custom-chocolat/20 transition-all group"
          >
            <font-awesome-icon :icon="mod.icon" class="w-5 h-5 text-base-content/40 group-hover:text-custom-chocolat transition-colors" />
            <span class="text-xl font-bold font-display">{{ mod.total }}</span>
            <span class="text-[10px] text-base-content/60 text-center leading-tight">{{ mod.label }}</span>
          </NuxtLink>
        </div>
      </div>
    </template>
  </div>
</template>
